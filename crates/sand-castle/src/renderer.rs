use derive_builder::Builder;
use web_sys::HtmlCanvasElement;
use wgpu::{
  Backends, Color, CommandEncoderDescriptor, CreateSurfaceError, DeviceDescriptor, Features,
  InstanceDescriptor, Limits, LoadOp, Operations, PowerPreference, RenderPassColorAttachment,
  RenderPassDescriptor, RequestAdapterOptions, StoreOp, SurfaceConfiguration, SurfaceTarget,
  TextureUsages, TextureViewDescriptor,
};

use crate::scene::Scene;

#[derive(Default, Clone, Copy)]
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
  pixel_ratio: f64,
  size: (i32, i32),
}

impl Renderer {
  pub fn builder() -> RendererBuilder {
    RendererBuilder::default()
  }

  pub fn render(&self, scene: &Scene) {
    let Ok(output) = self.surface.get_current_texture() else {
      return;
    };

    let view = output
      .texture
      .create_view(&TextureViewDescriptor::default());

    let mut encoder = self
      .device
      .create_command_encoder(&CommandEncoderDescriptor {
        label: Some("mesh"),
      });

    drop(encoder.begin_render_pass(&RenderPassDescriptor {
      label: Some("render pass"),
      color_attachments: &[Some(RenderPassColorAttachment {
        view: &view,
        resolve_target: None,
        ops: Operations {
          load: LoadOp::Clear(Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
          }),
          store: StoreOp::Store,
        },
      })],
      depth_stencil_attachment: None,
      timestamp_writes: None,
      occlusion_query_set: None,
    }));

    self.queue.submit([encoder.finish()]);

    output.present();
    for renderable in scene.renderables() {
      renderable.render(&self);
    }
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
}

pub trait Render {
  fn id(&self) -> u32;
  fn render(&self, renderer: &Renderer);
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
    let cap_format = capabilities
      .formats
      .iter()
      .copied()
      .filter(|f| f.is_srgb())
      .next()
      .unwrap_or(capabilities.formats[0]);

    let surface_config = SurfaceConfiguration {
      usage: TextureUsages::RENDER_ATTACHMENT,
      format: cap_format,
      width: size.0 as u32,
      height: size.1 as u32,
      // width: 300,
      // height: 200,
      present_mode: capabilities.present_modes[0],
      alpha_mode: capabilities.alpha_modes[0],
      view_formats: vec![],
      desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &surface_config);

    Ok(Renderer {
      driver: *driver,
      size: *size,
      pixel_ratio: *pixel_ratio,
      canvas: canvas.clone(),
      adapter,
      surface,
      device,
      queue,
      surface_config,
    })
  }
}
