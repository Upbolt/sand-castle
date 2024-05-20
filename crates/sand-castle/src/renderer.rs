use std::cell::RefCell;

use derive_builder::Builder;
use web_sys::HtmlCanvasElement;
use wgpu::{
  include_wgsl, Backends, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
  BindingType, BlendState, BufferAddress, BufferBindingType, Color, ColorTargetState, ColorWrites,
  CommandBuffer, CommandEncoderDescriptor, CreateSurfaceError, DeviceDescriptor, Face, Features,
  FragmentState, FrontFace, InstanceDescriptor, Limits, LoadOp, MultisampleState, Operations,
  PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode, PowerPreference,
  PrimitiveState, PrimitiveTopology, RenderPassColorAttachment, RenderPassDescriptor,
  RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, ShaderModuleDescriptor,
  ShaderStages, StoreOp, SurfaceConfiguration, SurfaceTarget, TextureUsages, TextureViewDescriptor,
  VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};

use smaa::{SmaaMode, SmaaTarget};

use derive_more::{Deref, DerefMut, From, Into};

use crate::{camera::Camera, scene::Scene, units::Vertex};

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Driver {
  #[default]
  WebGL,
  WebGPU,
}

#[derive(Builder)]
#[builder(build_fn(skip))]
pub struct Renderer {
  driver: Driver,
  canvas: HtmlCanvasElement,
  #[builder(setter(skip))]
  surface: wgpu::Surface<'static>,
  #[builder(setter(skip))]
  adapter: wgpu::Adapter,
  #[builder(setter(skip))]
  device: wgpu::Device,
  #[builder(setter(skip))]
  queue: wgpu::Queue,
  #[builder(setter(skip))]
  surface_config: wgpu::SurfaceConfiguration,
  #[builder(setter(skip))]
  smaa: RefCell<SmaaTarget>,
  #[builder(setter(skip))]
  cameras: Vec<Camera>,
  #[builder(setter(skip))]
  current_camera_index: Option<usize>,
  #[builder(setter(skip))]
  mesh_pipeline: RenderPipeline,
  #[builder(setter(skip))]
  camera_bind_group_layout: BindGroupLayout,
  pixel_ratio: f64,
  size: (i32, i32),
}

#[derive(From, Into)]
pub struct GpuCommand(CommandBuffer);

#[derive(From, Into, Deref, DerefMut)]
pub struct RenderPass<'a>(wgpu::RenderPass<'a>);

impl Renderer {
  pub fn builder() -> RendererBuilder {
    RendererBuilder::default()
  }

  pub fn render<'a>(&'a self, scene: &'a Scene, camera: impl Into<Option<&'a Camera>>) {
    let camera: Option<&Camera> = camera.into();

    let Ok(output) = self.surface.get_current_texture() else {
      return;
    };

    let view = output
      .texture
      .create_view(&TextureViewDescriptor::default());

    let mut smaa = self.smaa.borrow_mut();
    let smaa_frame = smaa.start_frame(&self.device, &self.queue, &view);

    let mut encoder = self
      .device
      .create_command_encoder(&CommandEncoderDescriptor {
        label: Some("mesh"),
      });

    {
      let mut render_pass = RenderPass(encoder.begin_render_pass(&RenderPassDescriptor {
        label: Some("render pass"),
        color_attachments: &[Some(RenderPassColorAttachment {
          view: &smaa_frame,
          resolve_target: None,
          ops: Operations {
            load: LoadOp::Clear(Color {
              r: 0.01,
              g: 0.01,
              b: 0.01,
              a: 1.0,
            }),
            store: StoreOp::Store,
          },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
      }));

      for renderable in scene.renderables() {
        renderable.render(RenderOptions {
          render_pass: &mut render_pass,
          pipeline: &self.mesh_pipeline,
          camera,
        });
      }
    }

    self.queue.submit([encoder.finish()]);

    smaa_frame.resolve();
    output.present();
  }

  pub fn resize(&self, size: (u32, u32)) {
    let mut new_config = self.surface_config.clone();

    new_config.width = size.0;
    new_config.height = size.1;

    self.surface.configure(&self.device, &new_config);
  }

  pub(crate) fn surface(&self) -> &wgpu::Surface {
    &self.surface
  }

  pub(crate) fn adapter(&self) -> &wgpu::Adapter {
    &self.adapter
  }

  pub(crate) fn device(&self) -> &wgpu::Device {
    &self.device
  }

  pub(crate) fn queue(&self) -> &wgpu::Queue {
    &self.queue
  }

  pub(crate) fn surface_config(&self) -> &wgpu::SurfaceConfiguration {
    &self.surface_config
  }

  pub fn cameras(&self) -> &Vec<Camera> {
    &self.cameras
  }

  pub(crate) fn cameras_mut(&mut self) -> &mut Vec<Camera> {
    &mut self.cameras
  }

  pub fn current_camera_index(&self) -> Option<usize> {
    self.current_camera_index
  }

  pub(crate) fn current_camera_index_mut(&mut self) -> &mut Option<usize> {
    &mut self.current_camera_index
  }

  pub(crate) fn camera_bind_group_layout(&self) -> &BindGroupLayout {
    &self.camera_bind_group_layout
  }
}

pub struct RenderOptions<'a, 'b: 'a> {
  // pub renderer: &'a Renderer,
  pub pipeline: &'b RenderPipeline,
  pub render_pass: &'a mut RenderPass<'b>,
  pub camera: Option<&'b Camera>,
}

pub trait Render {
  fn id(&self) -> u32;
  fn render<'a, 'b: 'a>(&'b self, options: RenderOptions<'a, 'b>);
}

#[derive(Debug)]
pub enum RendererInitializationError {
  Incomplete(&'static str),
  NoAdapter,
  CouldNotGetDevice,
  SurfaceCreation(CreateSurfaceError),
}

impl RendererBuilder {
  pub async fn build(&self) -> Result<Renderer, RendererInitializationError> {
    let Some(driver) = &self.driver else {
      return Err(RendererInitializationError::Incomplete(
        "field `driver` not provided",
      ));
    };

    let Some(size) = &self.size else {
      return Err(RendererInitializationError::Incomplete(
        "field `size` not provided",
      ));
    };

    let Some(pixel_ratio) = &self.pixel_ratio else {
      return Err(RendererInitializationError::Incomplete(
        "field `pixel_ratio` not provided",
      ));
    };

    let Some(canvas) = &self.canvas else {
      return Err(RendererInitializationError::Incomplete(
        "field `canvas` not provided",
      ));
    };

    let instance = wgpu::Instance::new(InstanceDescriptor {
      backends: Backends::GL,
      ..Default::default()
    });

    let surface = instance
      .create_surface(SurfaceTarget::Canvas(canvas.clone()))
      .map_err(|err| RendererInitializationError::SurfaceCreation(err))?;

    let Some(adapter) = instance
      .request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      })
      .await
    else {
      return Err(RendererInitializationError::NoAdapter);
    };

    let Ok((device, queue)) = adapter
      .request_device(
        &DeviceDescriptor {
          required_features: Features::empty(),
          required_limits: Limits::downlevel_webgl2_defaults(),
          label: None,
        },
        None,
      )
      .await
    else {
      return Err(RendererInitializationError::CouldNotGetDevice);
    };

    let capabilities = surface.get_capabilities(&adapter);

    let swapchain_format = capabilities.formats[0];

    let cap_format = capabilities
      .formats
      .iter()
      .copied()
      .filter(|f| f.is_srgb())
      .next()
      .unwrap_or(swapchain_format);

    let surface_config = SurfaceConfiguration {
      usage: TextureUsages::RENDER_ATTACHMENT,
      format: cap_format,
      width: size.0 as u32,
      height: size.1 as u32,
      present_mode: capabilities.present_modes[0],
      alpha_mode: capabilities.alpha_modes[0],
      view_formats: vec![],
      desired_maximum_frame_latency: 2,
    };

    let smaa = RefCell::new(SmaaTarget::new(
      &device,
      &queue,
      size.0 as u32,
      size.1 as u32,
      swapchain_format,
      SmaaMode::Smaa1X,
    ));

    surface.configure(&device, &surface_config);

    let vertex_shader = device.create_shader_module(include_wgsl!("object/shaders/mesh.wgsl"));
    let fragment_shader =
      device.create_shader_module(include_wgsl!("material/shaders/mesh_basic.wgsl"));

    let camera_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
      label: None,
      entries: &[BindGroupLayoutEntry {
        binding: 0,
        visibility: ShaderStages::VERTEX,
        ty: BindingType::Buffer {
          ty: BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: None,
        },
        count: None,
      }],
    });

    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
      label: None,
      bind_group_layouts: &[&camera_bind_group_layout],
      push_constant_ranges: &[],
    });

    let mesh_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
      vertex: VertexState {
        buffers: &[VertexBufferLayout {
          array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
          step_mode: VertexStepMode::Vertex,
          attributes: &[VertexAttribute {
            offset: 0,
            shader_location: 0,
            format: VertexFormat::Float32x3,
          }],
        }],
        // buffers: &[G::vertices_layout().into()],
        module: &vertex_shader,
        entry_point: "vs_main",
        compilation_options: PipelineCompilationOptions::default(),
      },
      label: None,
      layout: Some(&pipeline_layout),
      primitive: PrimitiveState {
        topology: PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: FrontFace::Ccw,
        cull_mode: Some(Face::Back),
        unclipped_depth: false,
        polygon_mode: PolygonMode::Fill,
        conservative: false,
      },
      depth_stencil: None,
      multisample: MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false,
      },
      fragment: Some(FragmentState {
        module: &fragment_shader,
        entry_point: "fs_main",
        compilation_options: PipelineCompilationOptions::default(),
        targets: &[Some(ColorTargetState {
          format: surface_config.format,
          blend: Some(BlendState::REPLACE),
          write_mask: ColorWrites::ALL,
        })],
      }),
      multiview: None,
    });

    Ok(Renderer {
      driver: *driver,
      size: *size,
      pixel_ratio: *pixel_ratio,
      canvas: canvas.clone(),
      adapter,
      surface,
      device,
      queue,
      smaa,
      cameras: vec![],
      current_camera_index: None,
      surface_config,
      mesh_pipeline,
      camera_bind_group_layout,
    })
  }
}
