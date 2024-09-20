use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use derive_builder::Builder;
use getset::Getters;
use glam::{Mat4, UVec3, UVec4, Vec3, Vec4};
use indexmap::IndexMap;
use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
  BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, BlendComponent,
  BlendFactor, BlendOperation, BlendState, Buffer, BufferBindingType, BufferUsages, Color,
  ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState, Extent3d,
  Face, FilterMode, FragmentState, FrontFace, ImageCopyTexture, ImageDataLayout, IndexFormat,
  LoadOp, MultisampleState, Operations, Origin3d, PipelineLayoutDescriptor, PolygonMode,
  PrimitiveState, PrimitiveTopology, RenderPassColorAttachment, RenderPassDepthStencilAttachment,
  RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, Sampler, SamplerBindingType,
  SamplerDescriptor, ShaderStages, StencilState, StoreOp, Texture, TextureAspect,
  TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages,
  TextureView, TextureViewDescriptor, TextureViewDimension, VertexState,
};

use wasm_bindgen::prelude::*;
use web_sys::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

use crate::{
  renderer::Renderer,
  resource::{
    camera::Camera,
    geometry::Geometry,
    lighting::light::{
      ambient_light::AmbientLight, directional_light::DirectionalLight, point_light::PointLight,
      spot_light::SpotLight,
    },
    loader::{geometry::GeometryLoader, material::MaterialLoader, textures::TextureLoader},
    object_3d::Object3D,
    texture::TextureId,
    Id, Resource,
  },
};

pub(crate) struct Subject {
  pub(crate) diffuse_texture_id: Option<TextureId>,
  pub(crate) material_data: Option<(Buffer, BindGroup)>,
  pub(crate) transform: (Buffer, BindGroup),
  pub(crate) normal: (Buffer, BindGroup),
  pub(crate) vertices: (Buffer, usize),
  pub(crate) indices: (Buffer, usize),
  pub(crate) pipeline: Option<Arc<RenderPipeline>>,
}

pub(crate) struct LightsBinding {
  pub(crate) directional_lights: Buffer,
  pub(crate) point_lights: Buffer,
  pub(crate) spot_lights: Buffer,
  pub(crate) directional_light_count: Buffer,
  pub(crate) point_light_count: Buffer,
  pub(crate) spot_light_count: Buffer,
  pub(crate) bind_group: BindGroup,
}

struct Depth {
  texture: Texture,
  view: TextureView,
  sampler: Sampler,
}

#[derive(Getters, Builder)]
#[getset(get = "pub")]
#[builder(pattern = "owned", build_fn(private, name = "fallible_build"))]
pub struct Scene {
  #[builder(default = "Vec4::new(0.0, 0.0, 0.0, 1.0)")]
  color: Vec4,

  #[builder(default = "Default::default()", setter(skip))]
  id: Id,

  #[getset(get = "pub(crate)")]
  #[builder(default = "Default::default()", setter(skip))]
  subjects: IndexMap<Id, Subject>,

  #[getset(get = "pub(crate)")]
  #[builder(setter(custom))]
  ambient_light: (Buffer, BindGroup),

  #[getset(skip)]
  #[builder(setter(custom))]
  ambient_light_layout: BindGroupLayout,

  #[getset(get = "pub(crate)")]
  #[builder(setter(custom))]
  lights: LightsBinding,

  #[getset(skip)]
  #[builder(default = "Default::default()", setter(skip))]
  material_pipelines: IndexMap<Id, Arc<RenderPipeline>>,

  #[getset(skip)]
  #[builder(default = "Default::default()", setter(skip))]
  material_layouts: IndexMap<Id, BindGroupLayout>,

  #[getset(skip)]
  #[builder(setter(custom))]
  texture_layout: BindGroupLayout,

  #[getset(skip)]
  #[builder(default = "Default::default()", setter(skip))]
  texture_map_views: IndexMap<TextureId, (TextureView, BindGroup)>,

  #[getset(skip)]
  #[builder(setter(custom))]
  texture_sampler: Sampler,

  #[getset(skip)]
  #[builder(setter(custom))]
  camera: (Buffer, BindGroup),

  #[getset(skip)]
  #[builder(setter(custom))]
  camera_layout: BindGroupLayout,

  #[getset(skip)]
  #[builder(setter(custom))]
  dynamic_lights_layout: BindGroupLayout,

  #[getset(skip)]
  #[builder(setter(custom))]
  depth: Depth,
}

impl PartialEq for Scene {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl SceneBuilder {
  fn init_textures(mut self, renderer: &Renderer) -> Self {
    let sampler = renderer.device().create_sampler(&SamplerDescriptor {
      address_mode_u: AddressMode::ClampToEdge,
      address_mode_v: AddressMode::ClampToEdge,
      address_mode_w: AddressMode::ClampToEdge,
      mag_filter: FilterMode::Linear,
      min_filter: FilterMode::Nearest,
      mipmap_filter: FilterMode::Nearest,
      ..Default::default()
    });

    let texture_layout = renderer
      .device()
      .create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("texture bind group layout"),
        entries: &[
          BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Texture {
              multisampled: false,
              view_dimension: TextureViewDimension::D2,
              sample_type: TextureSampleType::Float { filterable: true },
            },
            count: None,
          },
          BindGroupLayoutEntry {
            binding: 1,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Sampler(SamplerBindingType::Filtering),
            count: None,
          },
        ],
      });

    self.texture_layout = Some(texture_layout);
    self.texture_sampler = Some(sampler);

    self
  }

  fn init_camera(mut self, renderer: &Renderer) -> Self {
    let camera_layout = renderer
      .device()
      .create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("camera bind group layout"),
        entries: &[BindGroupLayoutEntry {
          binding: 0,
          visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
          ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
          },
          count: None,
        }],
      });

    let camera_buffer_contents = [0.0f32; 4 * 5];
    let camera_buffer = renderer.device().create_buffer_init(&BufferInitDescriptor {
      label: Some("camera buffer"),
      contents: bytemuck::cast_slice(&camera_buffer_contents),
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let camera_bind_group = renderer.device().create_bind_group(&BindGroupDescriptor {
      label: Some("camera bind group"),
      layout: &camera_layout,
      entries: &[BindGroupEntry {
        binding: 0,
        resource: camera_buffer.as_entire_binding(),
      }],
    });

    self.camera = Some((camera_buffer, camera_bind_group));
    self.camera_layout = Some(camera_layout);

    self
  }

  fn init_ambient_light(mut self, renderer: &Renderer) -> Self {
    let ambient_light_layout =
      renderer
        .device()
        .create_bind_group_layout(&BindGroupLayoutDescriptor {
          label: Some("ambient light layout"),
          entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Buffer {
              ty: BufferBindingType::Uniform,
              has_dynamic_offset: false,
              min_binding_size: None,
            },
            count: None,
          }],
        });

    let ambient_light_buffer = renderer.device().create_buffer_init(&BufferInitDescriptor {
      label: Some("ambient light buffer"),
      contents: bytemuck::cast_slice(&[Vec4::new(0.0, 0.0, 0.0, 1.0)]),
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let ambient_light_bind_group = renderer.device().create_bind_group(&BindGroupDescriptor {
      label: Some("ambient_light bind group"),
      layout: &ambient_light_layout,
      entries: &[BindGroupEntry {
        binding: 0,
        resource: ambient_light_buffer.as_entire_binding(),
      }],
    });

    self.ambient_light = Some((ambient_light_buffer, ambient_light_bind_group));
    self.ambient_light_layout = Some(ambient_light_layout);

    self
  }

  fn init_dynamic_lights(mut self, renderer: &Renderer) -> Self {
    let dynamic_lights_layout =
      renderer
        .device()
        .create_bind_group_layout(&BindGroupLayoutDescriptor {
          label: Some("dynamic lights layout"),
          entries: &[
            BindGroupLayoutEntry {
              binding: 0,
              visibility: ShaderStages::FRAGMENT,
              ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
              },
              count: None,
            },
            BindGroupLayoutEntry {
              binding: 1,
              visibility: ShaderStages::FRAGMENT,
              ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
              },
              count: None,
            },
            BindGroupLayoutEntry {
              binding: 2,
              visibility: ShaderStages::FRAGMENT,
              ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
              },
              count: None,
            },
            BindGroupLayoutEntry {
              binding: 3,
              visibility: ShaderStages::FRAGMENT,
              ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
              },
              count: None,
            },
            BindGroupLayoutEntry {
              binding: 4,
              visibility: ShaderStages::FRAGMENT,
              ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
              },
              count: None,
            },
            BindGroupLayoutEntry {
              binding: 5,
              visibility: ShaderStages::FRAGMENT,
              ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
              },
              count: None,
            },
          ],
        });

    {
      let directional_light_buffer = [DirectionalLightBuffer::default(); 16];
      let point_light_buffer = [PointLightBuffer::default(); 16];
      let spot_light_buffer = [SpotLightBuffer::default(); 16];

      let directional_lights = renderer.device().create_buffer_init(&BufferInitDescriptor {
        label: Some("initial directional lights"),
        contents: bytemuck::cast_slice(&directional_light_buffer),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
      });

      let point_lights = renderer.device().create_buffer_init(&BufferInitDescriptor {
        label: Some("initial point lights"),
        contents: bytemuck::cast_slice(&point_light_buffer),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
      });

      let spot_lights = renderer.device().create_buffer_init(&BufferInitDescriptor {
        label: Some("initial spot lights"),
        contents: bytemuck::cast_slice(&spot_light_buffer),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
      });

      let directional_light_count = renderer.device().create_buffer_init(&BufferInitDescriptor {
        label: Some("initial directional light count"),
        contents: bytemuck::cast_slice(&[UVec4::from((0, UVec3::default()))]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
      });

      let point_light_count = renderer.device().create_buffer_init(&BufferInitDescriptor {
        label: Some("initial point light count"),
        contents: bytemuck::cast_slice(&[UVec4::from((0, UVec3::default()))]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
      });

      let spot_light_count = renderer.device().create_buffer_init(&BufferInitDescriptor {
        label: Some("initial spot light count"),
        contents: bytemuck::cast_slice(&[UVec4::from((0, UVec3::default()))]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
      });

      self.lights = Some(LightsBinding {
        bind_group: renderer.device().create_bind_group(&BindGroupDescriptor {
          label: Some("initial directional light bind group"),
          layout: &dynamic_lights_layout,
          entries: &[
            BindGroupEntry {
              binding: 0,
              resource: directional_lights.as_entire_binding(),
            },
            BindGroupEntry {
              binding: 1,
              resource: point_lights.as_entire_binding(),
            },
            BindGroupEntry {
              binding: 2,
              resource: spot_lights.as_entire_binding(),
            },
            BindGroupEntry {
              binding: 3,
              resource: directional_light_count.as_entire_binding(),
            },
            BindGroupEntry {
              binding: 4,
              resource: point_light_count.as_entire_binding(),
            },
            BindGroupEntry {
              binding: 5,
              resource: spot_light_count.as_entire_binding(),
            },
          ],
        }),
        directional_lights,
        point_lights,
        spot_lights,
        directional_light_count,
        point_light_count,
        spot_light_count,
      });
    }

    self.dynamic_lights_layout = Some(dynamic_lights_layout);
    self
  }

  fn init_depth(mut self, renderer: &Renderer) -> Self {
    let texture = renderer.device().create_texture(&TextureDescriptor {
      label: Some("depth texture"),
      size: Extent3d {
        width: renderer.canvas().client_width() as u32,
        height: renderer.canvas().client_height() as u32,
        depth_or_array_layers: 1,
      },
      mip_level_count: 1,
      sample_count: 1,
      dimension: TextureDimension::D2,
      format: TextureFormat::Depth32Float,
      usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
      view_formats: &[],
    });

    self.depth = Some(Depth {
      view: texture.create_view(&TextureViewDescriptor::default()),
      sampler: renderer.device().create_sampler(&SamplerDescriptor {
        address_mode_u: AddressMode::ClampToEdge,
        address_mode_v: AddressMode::ClampToEdge,
        address_mode_w: AddressMode::ClampToEdge,
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Linear,
        mipmap_filter: FilterMode::Nearest,
        compare: Some(CompareFunction::LessEqual),
        lod_min_clamp: 0.0,
        lod_max_clamp: 100.0,
        ..Default::default()
      }),
      texture,
    });

    self
  }

  pub fn build(self) -> Scene {
    self.fallible_build().expect("failed to build `Scene`")
  }
}

#[repr(C)]
#[derive(Pod, Zeroable, Default, Clone, Copy)]
struct PointLightBuffer {
  position: Vec4,
  color: Vec4,
}

#[repr(C)]
#[derive(Pod, Zeroable, Default, Clone, Copy)]
struct DirectionalLightBuffer {
  color: Vec4,
  direction: Vec4,
}

#[repr(C)]
#[derive(Pod, Zeroable, Default, Clone, Copy)]
struct SpotLightBuffer {
  position: Vec4,
  color: Vec4,
  direction: Vec4,
  // actually an f32, but to satisfy bytemuck we need an extra 12 bytes of padding so we use a Vec4
  cutoff_angle: Vec4,
}

impl Scene {
  pub fn builder(renderer: &Renderer) -> SceneBuilder {
    SceneBuilder::default()
      .init_camera(&renderer)
      .init_depth(&renderer)
      .init_ambient_light(&renderer)
      .init_dynamic_lights(&renderer)
      .init_textures(&renderer)
  }

  pub fn subject_count(&self) -> usize {
    self.subjects.len()
  }

  pub fn set_color(&mut self, color: Vec4) {
    self.color = color;
  }

  pub fn has_subject(&self, object: &(impl Resource + Object3D)) -> bool {
    self.subjects.contains_key(&object.id())
  }

  pub fn insert<Object: Resource + Object3D>(
    &mut self,
    renderer: &Renderer,
    geometry_loader: &GeometryLoader,
    material_loader: &MaterialLoader,
    texture_loader: &TextureLoader,
    object: &Object,
  ) {
    let geometry = object
      .geometry_id()
      .and_then(|id| geometry_loader.get_from_id(id));

    let material = object
      .material_id()
      .and_then(|id| material_loader.get_from_id(id));

    let vertices = (
      renderer.device().create_buffer_init(&BufferInitDescriptor {
        label: Some("vertex buffer"),
        contents: bytemuck::cast_slice(
          geometry
            .map(|geometry| geometry.vertices())
            .unwrap_or(&vec![]),
        ),
        usage: BufferUsages::VERTEX,
      }),
      geometry
        .map(|geometry| geometry.vertices().len())
        .unwrap_or_default(),
    );

    let indices = (
      renderer.device().create_buffer_init(&BufferInitDescriptor {
        label: Some("index buffer"),
        contents: bytemuck::cast_slice(
          geometry
            .map(|geometry| geometry.indices())
            .unwrap_or(&vec![]),
        ),
        usage: BufferUsages::INDEX,
      }),
      geometry
        .map(|geometry| geometry.indices().len())
        .unwrap_or_default(),
    );

    let transform = renderer.device().create_buffer_init(&BufferInitDescriptor {
      label: Some("transform buffer"),
      contents: bytemuck::cast_slice(&[
        Mat4::from_translation(object.pos().clone()) * Mat4::from_quat(object.rot().clone())
      ]),
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let normal = renderer.device().create_buffer_init(&BufferInitDescriptor {
      label: Some("normal buffer"),
      contents: bytemuck::cast_slice(&[Mat4::from_quat(object.rot().clone())]),
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    if let Some(id) = material.and_then(|material| material.diffuse_map_texture_id().clone()) {
      if let Some((content, dimensions)) = texture_loader
        .get_from_id(id)
        .map(|texture| (texture.content.as_slice(), texture.dimensions))
      {
        let size = Extent3d {
          width: dimensions.0,
          height: dimensions.1,
          depth_or_array_layers: 1,
        };

        let texture = renderer.device().create_texture(&TextureDescriptor {
          size,
          mip_level_count: 1,
          sample_count: 1,
          dimension: TextureDimension::D2,
          format: TextureFormat::Rgba8UnormSrgb,
          usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
          label: Some(&format!(r#"TextureId({:?})"#, id)),
          view_formats: &[],
        });

        renderer.queue().write_texture(
          ImageCopyTexture {
            aspect: TextureAspect::All,
            texture: &texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
          },
          content,
          ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * dimensions.0),
            rows_per_image: Some(dimensions.1),
          },
          size,
        );

        let texture_view = texture.create_view(&TextureViewDescriptor::default());

        let texture_bind_group = renderer.device().create_bind_group(&BindGroupDescriptor {
          label: Some(&format!("TextureId({:?}) bind group", id)),
          layout: &self.texture_layout,
          entries: &[
            BindGroupEntry {
              binding: 0,
              resource: BindingResource::TextureView(&texture_view),
            },
            BindGroupEntry {
              binding: 1,
              resource: BindingResource::Sampler(&self.texture_sampler),
            },
          ],
        });

        self
          .texture_map_views
          .insert(id, (texture_view, texture_bind_group));
      }
    }

    let transform_layout = renderer
      .device()
      .create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("transform layout"),
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

    let transform_bind_group = renderer.device().create_bind_group(&BindGroupDescriptor {
      label: Some("transform bind group"),
      layout: &transform_layout,
      entries: &[BindGroupEntry {
        binding: 0,
        resource: transform.as_entire_binding(),
      }],
    });

    let normal_layout = renderer
      .device()
      .create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("normal bind group layout"),
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

    let normal_bind_group = renderer.device().create_bind_group(&BindGroupDescriptor {
      label: Some("normal bind group"),
      layout: &normal_layout,
      entries: &[BindGroupEntry {
        binding: 0,
        resource: normal.as_entire_binding(),
      }],
    });

    let (pipeline, material_data) = object
      .material_id()
      .and_then(|id| material_loader.get_from_id(id))
      .map(|material| {
        let fragment_data_layout =
          self
            .material_layouts
            .entry(*material.id())
            .or_insert_with(|| {
              renderer
                .device()
                .create_bind_group_layout(material.fragment_data_layout())
            });

        let fragment_data_buffer = renderer.device().create_buffer_init(&BufferInitDescriptor {
          label: Some("fragment data buffer"),
          contents: &material.fragment_data(),
          usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let fragment_data_bind_group = renderer.device().create_bind_group(&BindGroupDescriptor {
          label: Some("fragment data bind group"),
          layout: fragment_data_layout,
          entries: &[BindGroupEntry {
            binding: 0,
            resource: fragment_data_buffer.as_entire_binding(),
          }],
        });

        (
          self
            .material_pipelines
            .get(material.id())
            .cloned()
            .unwrap_or_else(|| {
              let mut bind_group_layouts = vec![
                &self.camera_layout,
                &transform_layout,
                fragment_data_layout,
                &normal_layout,
                &self.ambient_light_layout,
                &self.dynamic_lights_layout,
              ];

              if material.diffuse_map_texture_id().is_some() {
                bind_group_layouts.push(&self.texture_layout);
              }

              let pipeline_layout =
                renderer
                  .device()
                  .create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: Some("insert pipeline layout"),
                    bind_group_layouts: &bind_group_layouts,
                    push_constant_ranges: &[],
                  });

              let target = [renderer.supported_format().map(|format| ColorTargetState {
                format,
                blend: Some(BlendState {
                  color: BlendComponent {
                    src_factor: BlendFactor::SrcAlpha,
                    dst_factor: BlendFactor::OneMinusSrcAlpha,
                    operation: BlendOperation::Add,
                  },
                  alpha: BlendComponent::OVER,
                }),
                write_mask: ColorWrites::ALL,
              })];

              let vertex_shader = renderer
                .device()
                .create_shader_module(material.vertex_shader().clone());
              let fragment_shader = renderer
                .device()
                .create_shader_module(material.fragment_shader().clone());

              let pipeline = Arc::new(renderer.device().create_render_pipeline(
                &RenderPipelineDescriptor {
                  label: Some("insertion pipeline"),
                  layout: Some(&pipeline_layout),
                  vertex: VertexState {
                    module: &vertex_shader,
                    entry_point: "vs_main",
                    buffers: &[Geometry::vertex_desc()],
                    compilation_options: Default::default(),
                  },
                  primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: Some(Face::Back),
                    polygon_mode: PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                  },
                  depth_stencil: Some(DepthStencilState {
                    format: TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: CompareFunction::Less,
                    stencil: StencilState::default(),
                    bias: DepthBiasState::default(),
                  }),
                  multisample: MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                  },
                  fragment: Some(FragmentState {
                    module: &fragment_shader,
                    entry_point: "fs_main",
                    compilation_options: Default::default(),
                    targets: &target,
                  }),
                  multiview: None,
                  cache: None,
                },
              ));

              self
                .material_pipelines
                .insert(*material.id(), pipeline.clone());

              pipeline
            }),
          (fragment_data_buffer, fragment_data_bind_group),
        )
      })
      .map(|(material_data, pipeline)| (Some(material_data), Some(pipeline)))
      .unwrap_or((None, None));

    self.subjects.insert(
      object.id(),
      Subject {
        diffuse_texture_id: material.and_then(|material| material.diffuse_map_texture_id().clone()),
        material_data,
        normal: (normal, normal_bind_group),
        transform: (transform, transform_bind_group),
        vertices,
        indices,
        pipeline,
      },
    );
  }

  pub fn remove(&mut self, object: &impl Resource) {
    self.subjects.shift_remove(&object.id());
  }

  pub fn render(&self, renderer: &Renderer) {
    let Ok(output) = renderer.surface().get_current_texture() else {
      return;
    };

    let (_, camera_bind_group) = &self.camera;

    let view = output.texture.create_view(&Default::default());

    let mut encoder = renderer
      .device()
      .create_command_encoder(&Default::default());

    let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
      color_attachments: &[Some(RenderPassColorAttachment {
        view: &view,
        resolve_target: None,
        ops: Operations {
          load: LoadOp::Clear(Color {
            r: self.color.x as f64,
            g: self.color.y as f64,
            b: self.color.z as f64,
            a: self.color.w as f64,
          }),
          store: StoreOp::Store,
        },
      })],
      depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
        view: &self.depth.view,
        depth_ops: Some(Operations {
          load: LoadOp::Clear(1.0),
          store: StoreOp::Store,
        }),
        stencil_ops: None,
      }),
      ..Default::default()
    });

    for Subject {
      diffuse_texture_id,
      material_data,
      normal: (_, normal_bind_group),
      transform: (_, transform_bind_group),
      vertices: (vertices, _),
      indices: (indices, index_count),
      pipeline,
    } in self.subjects.values()
    {
      let Some(pipeline) = pipeline else {
        continue;
      };

      render_pass.set_pipeline(pipeline);

      render_pass.set_bind_group(0, camera_bind_group, &[]);
      render_pass.set_bind_group(1, transform_bind_group, &[]);

      if let Some((_, material_data_bind_group)) = material_data {
        render_pass.set_bind_group(2, material_data_bind_group, &[]);
      }

      render_pass.set_bind_group(3, normal_bind_group, &[]);
      render_pass.set_bind_group(4, &self.ambient_light.1, &[]);
      render_pass.set_bind_group(5, &self.lights.bind_group, &[]);

      if let Some((_, bind_group)) = diffuse_texture_id
        .as_ref()
        .and_then(|id| self.texture_map_views.get(id))
      {
        render_pass.set_bind_group(6, bind_group, &[]);
      }

      render_pass.set_vertex_buffer(0, vertices.slice(..));
      render_pass.set_index_buffer(indices.slice(..), IndexFormat::Uint32);

      render_pass.draw_indexed(0..*index_count as u32, 0, 0..1);
    }

    drop(render_pass);

    renderer.queue().submit([encoder.finish()]);
    output.present();
  }

  pub fn update_ambient_light(&mut self, renderer: &Renderer, ambient_light: &AmbientLight) {
    let (buffer, _) = &self.ambient_light;

    renderer.queue().write_buffer(
      buffer,
      0,
      bytemuck::cast_slice(&[Vec4::from((*ambient_light.color(), 1.0))]),
    );
  }

  pub fn update_point_light(
    &mut self,
    renderer: &Renderer,
    index: usize,
    point_light: &PointLight,
  ) {
    renderer.queue().write_buffer(
      &self.lights.point_lights,
      (index * size_of::<PointLightBuffer>()) as u64,
      bytemuck::cast_slice(&[PointLightBuffer {
        position: Vec4::from((*point_light.position(), 0.0)),
        color: Vec4::from((*point_light.color(), 1.0)),
      }]),
    )
  }

  pub fn update_spot_light(&mut self, renderer: &Renderer, index: usize, spot_light: &SpotLight) {
    renderer.queue().write_buffer(
      &self.lights.spot_lights,
      (index * size_of::<SpotLightBuffer>()) as u64,
      bytemuck::cast_slice(&[SpotLightBuffer {
        position: Vec4::from((*spot_light.position(), 0.0)),
        color: Vec4::from((*spot_light.color(), 1.0)),
        direction: Vec4::from((*spot_light.direction(), 1.0)),
        cutoff_angle: Vec4::from((*spot_light.cutoff_angle(), Vec3::default())),
      }]),
    )
  }

  pub fn update_directional_light(
    &mut self,
    renderer: &Renderer,
    index: usize,
    spot_light: &DirectionalLight,
  ) {
    renderer.queue().write_buffer(
      &self.lights.directional_lights,
      (index * size_of::<DirectionalLightBuffer>()) as u64,
      bytemuck::cast_slice(&[DirectionalLightBuffer {
        color: Vec4::from((*spot_light.color(), 1.0)),
        direction: Vec4::from((*spot_light.direction(), 1.0)),
      }]),
    )
  }

  pub fn bind_point_lights(&mut self, renderer: &Renderer, point_lights: &[PointLight]) {
    let mut lights = [PointLightBuffer::default(); 16];

    for (i, buffer) in (0..16).zip(point_lights.iter()).map(|(i, light)| {
      (
        i,
        PointLightBuffer {
          position: Vec4::from((*light.position(), 0.0)),
          color: Vec4::from((*light.color(), 1.0)),
        },
      )
    }) {
      lights[i] = buffer;
    }

    renderer
      .queue()
      .write_buffer(&self.lights.point_lights, 0, bytemuck::cast_slice(&lights));

    renderer.queue().write_buffer(
      &self.lights.point_light_count,
      0,
      bytemuck::cast_slice(&[UVec4::from((
        point_lights.len().min(16) as u32,
        UVec3::default(),
      ))]),
    );
  }

  pub fn bind_spot_lights(&mut self, renderer: &Renderer, spot_lights: &[SpotLight]) {
    let mut lights = [SpotLightBuffer::default(); 16];

    for (i, buffer) in (0..16).zip(spot_lights.iter()).map(|(i, light)| {
      (
        i,
        SpotLightBuffer {
          position: Vec4::from((*light.position(), 0.0)),
          color: Vec4::from((*light.color(), 1.0)),
          direction: Vec4::from((*light.direction(), 0.0)),
          cutoff_angle: Vec4::from((*light.cutoff_angle(), Vec3::default())),
        },
      )
    }) {
      lights[i] = buffer;
    }

    renderer
      .queue()
      .write_buffer(&self.lights.spot_lights, 0, bytemuck::cast_slice(&lights));

    renderer.queue().write_buffer(
      &self.lights.spot_light_count,
      0,
      bytemuck::cast_slice(&[UVec4::from((
        spot_lights.len().min(16) as u32,
        UVec3::default(),
      ))]),
    );
  }

  pub fn bind_directional_lights(
    &mut self,
    renderer: &Renderer,
    directional_lights: &[DirectionalLight],
  ) {
    let mut lights = [DirectionalLightBuffer::default(); 16];

    for (i, buffer) in (0..16).zip(directional_lights.iter()).map(|(i, light)| {
      (
        i,
        DirectionalLightBuffer {
          color: Vec4::from((*light.color(), 1.0)),
          direction: Vec4::from((*light.direction(), 0.0)),
        },
      )
    }) {
      lights[i] = buffer;
    }

    renderer.queue().write_buffer(
      &self.lights.directional_lights,
      0,
      bytemuck::cast_slice(&lights),
    );

    renderer.queue().write_buffer(
      &self.lights.directional_light_count,
      0,
      bytemuck::cast_slice(&[UVec4::from((
        directional_lights.len().min(16) as u32,
        UVec3::default(),
      ))]),
    );
  }

  pub fn set_camera(&mut self, renderer: &Renderer, camera: &impl Camera) {
    let mut camera_buffer_contents = [0.0; 4 * 5];
    camera
      .to_matrix()
      .write_cols_to_slice(&mut camera_buffer_contents);
    camera
      .pos()
      .write_to_slice(&mut camera_buffer_contents[16..]);

    let camera_buffer = renderer.device().create_buffer_init(&BufferInitDescriptor {
      label: Some("camera buffer"),
      contents: bytemuck::cast_slice(&camera_buffer_contents),
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let camera_bind_group = renderer.device().create_bind_group(&BindGroupDescriptor {
      label: Some("camera bind group"),
      layout: &self.camera_layout,
      entries: &[BindGroupEntry {
        binding: 0,
        resource: camera_buffer.as_entire_binding(),
      }],
    });

    self.camera = (camera_buffer, camera_bind_group);
  }

  pub fn update_camera(&self, renderer: &Renderer, camera: &impl Camera) {
    let (camera_buffer, _) = &self.camera;

    renderer.queue().write_buffer(
      camera_buffer,
      0,
      bytemuck::cast_slice(&[camera.to_matrix()]),
    );
  }

  pub fn update_material(
    &mut self,
    renderer: &Renderer,
    resource: &mut (impl Resource + Object3D),
    texture_loader: &TextureLoader,
    material_loader: &MaterialLoader,
    material_id: Id,
  ) {
    let (Some(subject), Some(material)) = (
      self.subjects.get_mut(&resource.id()),
      material_loader.get_from_id(material_id),
    ) else {
      return;
    };

    if let Some(id) = material.diffuse_map_texture_id() {
      if let Some((content, dimensions)) = texture_loader
        .get_from_id(*id)
        .map(|texture| (texture.content.as_slice(), texture.dimensions))
      {
        let size = Extent3d {
          width: dimensions.0,
          height: dimensions.1,
          depth_or_array_layers: 1,
        };

        let texture = renderer.device().create_texture(&TextureDescriptor {
          size,
          mip_level_count: 1,
          sample_count: 1,
          dimension: TextureDimension::D2,
          format: TextureFormat::Rgba8UnormSrgb,
          usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
          label: Some(&format!(
            r#"TextureId({:?})"#,
            material.diffuse_map_texture_id()
          )),
          view_formats: &[],
        });

        renderer.queue().write_texture(
          ImageCopyTexture {
            aspect: TextureAspect::All,
            texture: &texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
          },
          content,
          ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * dimensions.0),
            rows_per_image: Some(dimensions.1),
          },
          size,
        );

        let texture_view = texture.create_view(&TextureViewDescriptor::default());

        let texture_bind_group = renderer.device().create_bind_group(&BindGroupDescriptor {
          label: material
            .diffuse_map_texture_id()
            .map(|id| format!("TextureId({:?}) bind group", id))
            .as_deref(),
          layout: &self.texture_layout,
          entries: &[
            BindGroupEntry {
              binding: 0,
              resource: BindingResource::TextureView(&texture_view),
            },
            BindGroupEntry {
              binding: 1,
              resource: BindingResource::Sampler(&self.texture_sampler),
            },
          ],
        });

        self
          .texture_map_views
          .insert(*id, (texture_view, texture_bind_group));
      }
    }

    let transform_layout = renderer
      .device()
      .create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("transform bind group layout"),
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

    let normal_layout = renderer
      .device()
      .create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("normal bind group layout"),
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

    let fragment_data_layout = self
      .material_layouts
      .entry(*material.id())
      .or_insert_with(|| {
        renderer
          .device()
          .create_bind_group_layout(material.fragment_data_layout())
      });

    let fragment_data_buffer = renderer.device().create_buffer_init(&BufferInitDescriptor {
      label: Some("fragment data buffer"),
      contents: &material.fragment_data(),
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let fragment_data_bind_group = renderer.device().create_bind_group(&BindGroupDescriptor {
      label: Some("fragment data bind group"),
      layout: fragment_data_layout,
      entries: &[BindGroupEntry {
        binding: 0,
        resource: fragment_data_buffer.as_entire_binding(),
      }],
    });

    let pipeline = self
      .material_pipelines
      .get(material.id())
      .cloned()
      .unwrap_or_else(|| {
        let mut bind_group_layouts = vec![
          &self.camera_layout,
          &transform_layout,
          fragment_data_layout,
          &normal_layout,
          &self.ambient_light_layout,
          &self.dynamic_lights_layout,
        ];

        if material.diffuse_map_texture_id().is_some() {
          bind_group_layouts.push(&self.texture_layout);
        }

        let pipeline_layout = renderer
          .device()
          .create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("update pipeline layout"),
            bind_group_layouts: &bind_group_layouts,
            push_constant_ranges: &[],
          });

        let target = [renderer.supported_format().map(|format| ColorTargetState {
          format,
          blend: Some(BlendState {
            color: BlendComponent {
              src_factor: BlendFactor::SrcAlpha,
              dst_factor: BlendFactor::OneMinusSrcAlpha,
              operation: BlendOperation::Add,
            },
            alpha: BlendComponent::OVER,
          }),
          write_mask: ColorWrites::ALL,
        })];

        let vertex_shader = renderer
          .device()
          .create_shader_module(material.vertex_shader().clone());

        let fragment_shader = renderer
          .device()
          .create_shader_module(material.fragment_shader().clone());

        let pipeline = Arc::new(renderer.device().create_render_pipeline(
          &RenderPipelineDescriptor {
            label: Some("update pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
              module: &vertex_shader,
              entry_point: "vs_main",
              buffers: &[Geometry::vertex_desc()],
              compilation_options: Default::default(),
            },
            primitive: PrimitiveState {
              topology: PrimitiveTopology::TriangleList,
              strip_index_format: None,
              front_face: FrontFace::Ccw,
              cull_mode: Some(Face::Back),
              polygon_mode: PolygonMode::Fill,
              unclipped_depth: false,
              conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
              format: TextureFormat::Depth32Float,
              depth_write_enabled: true,
              depth_compare: CompareFunction::Less,
              stencil: StencilState::default(),
              bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState {
              count: 1,
              mask: !0,
              alpha_to_coverage_enabled: false,
            },
            fragment: Some(FragmentState {
              module: &fragment_shader,
              entry_point: "fs_main",
              compilation_options: Default::default(),
              targets: &target,
            }),
            multiview: None,
            cache: None,
          },
        ));

        self
          .material_pipelines
          .insert(*material.id(), pipeline.clone());

        pipeline
      });

    subject.pipeline = Some(pipeline);
    subject.diffuse_texture_id = material.diffuse_map_texture_id().clone();
    subject.material_data = Some((fragment_data_buffer, fragment_data_bind_group));

    resource.set_material_id(material_id);
  }

  pub fn update_geometry(
    &mut self,
    renderer: &Renderer,
    resource: &mut (impl Resource + Object3D),
    loader: &GeometryLoader,
    geometry_id: Id,
  ) {
    let Some(geometry) = loader.get_from_id(geometry_id) else {
      return;
    };

    let vertices = (
      renderer.device().create_buffer_init(&BufferInitDescriptor {
        label: Some("update vertex buffer"),
        contents: bytemuck::cast_slice(geometry.vertices()),
        usage: BufferUsages::VERTEX,
      }),
      geometry.vertices().len(),
    );

    let indices = (
      renderer.device().create_buffer_init(&BufferInitDescriptor {
        label: Some("update index buffer"),
        contents: bytemuck::cast_slice(geometry.indices()),
        usage: BufferUsages::INDEX,
      }),
      geometry.indices().len(),
    );

    if let Some(subject) = self.subjects.get_mut(&resource.id()) {
      subject.indices = indices;
      subject.vertices = vertices;
    }

    resource.set_geometry_id(geometry_id);
  }

  pub fn update_material_data(
    &self,
    renderer: &Renderer,
    resource: &(impl Resource + Object3D),
    data: &[u8],
  ) {
    let Some(subject) = self.subjects.get(&resource.id()) else {
      return;
    };

    if let Some((buffer, _)) = &subject.material_data {
      renderer.queue().write_buffer(buffer, 0, data);
    }
  }

  pub fn remove_material(&mut self, resource: &(impl Resource + Object3D)) {
    let Some(subject) = self.subjects.get_mut(&resource.id()) else {
      return;
    };

    subject.material_data = None;
    subject.pipeline = None;
  }
}
