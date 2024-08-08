use std::sync::Arc;

use derive_builder::Builder;
use derive_getters::Getters;
use glam::{Mat4, Quat, Vec3, Vec4};
use indexmap::IndexMap;
use wgpu::{
  include_wgsl,
  util::{BufferInitDescriptor, DeviceExt},
  BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
  BindGroupLayoutEntry, BindingType, BlendState, Buffer, BufferBindingType, BufferUsages, Color,
  ColorTargetState, ColorWrites, Face, FragmentState, FrontFace, IndexFormat, LoadOp,
  MultisampleState, Operations, PipelineLayoutDescriptor, PolygonMode, PrimitiveState,
  PrimitiveTopology, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
  RenderPipelineDescriptor, ShaderStages, StoreOp, VertexState,
};

use crate::{
  renderer::Renderer,
  resource::{
    camera::Camera,
    geometry::Geometry,
    lighting::material::Material,
    object_3d::{Object3D, Scale, Transform},
    Id, Resource,
  },
};

struct Subject {
  transform: (Buffer, BindGroup),
  vertices: (Buffer, usize),
  indices: (Buffer, usize),
  pipeline: Arc<RenderPipeline>,
}

#[derive(Getters, Builder)]
#[builder(pattern = "owned", build_fn(private, name = "fallible_build"))]
pub struct Scene {
  #[builder(default = "Vec4::new(0.0, 0.0, 0.0, 1.0)")]
  color: Vec4,

  #[builder(default = "Default::default()", setter(skip))]
  id: Id,

  #[getter(skip)]
  #[builder(default = "Default::default()", setter(skip))]
  subjects: IndexMap<Id, Subject>,

  #[getter(skip)]
  #[builder(default = "Default::default()", setter(skip))]
  material_pipelines: IndexMap<Id, Arc<RenderPipeline>>,

  #[getter(skip)]
  #[builder(default = "Default::default()", setter(skip))]
  camera: Option<(Buffer, BindGroup)>,

  #[getter(skip)]
  #[builder(default = "Default::default()", setter(skip))]
  camera_layout: Option<BindGroupLayout>,
}

impl PartialEq for Scene {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl SceneBuilder {
  pub fn build(self) -> Scene {
    self.fallible_build().expect("failed to build `Scene`")
  }
}

impl Scene {
  pub fn builder() -> SceneBuilder {
    Default::default()
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

  pub fn insert<Object: Resource + Object3D>(&mut self, renderer: &Renderer, object: &Object) {
    let camera_layout = match &self.camera_layout {
      Some(layout) => layout,
      None => {
        self.camera_layout = Some(renderer.device().create_bind_group_layout(
          &BindGroupLayoutDescriptor {
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
          },
        ));

        self.camera_layout.as_ref().unwrap()
      }
    };

    if self.camera.is_none() {
      let camera_buffer = renderer.device().create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&[Mat4::default()]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
      });

      let camera_bind_group = renderer.device().create_bind_group(&BindGroupDescriptor {
        label: Some("camera bind group"),
        layout: camera_layout,
        entries: &[BindGroupEntry {
          binding: 0,
          resource: camera_buffer.as_entire_binding(),
        }],
      });

      self.camera = Some((camera_buffer, camera_bind_group));
    }

    let vertices = (
      renderer.device().create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(object.geometry().vertices()),
        usage: BufferUsages::VERTEX,
      }),
      object.geometry().vertices().len(),
    );

    let indices = (
      renderer.device().create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(object.geometry().indices()),
        usage: BufferUsages::INDEX,
      }),
      object.geometry().indices().len(),
    );

    let transform = renderer.device().create_buffer_init(&BufferInitDescriptor {
      label: None,
      contents: bytemuck::cast_slice(&[
        Mat4::from_translation(object.pos().clone()) * Mat4::from_quat(object.rot().clone())
      ]),
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let transform_layout = renderer
      .device()
      .create_bind_group_layout(&BindGroupLayoutDescriptor {
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

    let transform_bind_group = renderer.device().create_bind_group(&BindGroupDescriptor {
      label: Some("transform bind group"),
      layout: &transform_layout,
      entries: &[BindGroupEntry {
        binding: 0,
        resource: transform.as_entire_binding(),
      }],
    });

    let vertex_shader = renderer
      .device()
      .create_shader_module(include_wgsl!("shaders/vertex.wgsl"));

    let pipeline = self
      .material_pipelines
      .get(object.material().id())
      .cloned()
      .unwrap_or_else(|| {
        let transform_layout =
          renderer
            .device()
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
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

        let pipeline_layout = renderer
          .device()
          .create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&transform_layout, camera_layout],
            push_constant_ranges: &[],
          });

        let target = [renderer.supported_format().map(|format| ColorTargetState {
          format,
          blend: Some(BlendState::REPLACE),
          write_mask: ColorWrites::ALL,
        })];

        let fragment = object
          .material()
          .shader()
          .as_ref()
          .map(|shader| renderer.device().create_shader_module(shader.clone()));

        let fragment = fragment.as_ref().map(|module| FragmentState {
          module: &vertex_shader,
          entry_point: "fs_main",
          compilation_options: Default::default(),
          targets: &target,
        });

        let pipeline = Arc::new(renderer.device().create_render_pipeline(
          &RenderPipelineDescriptor {
            label: None,
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
            depth_stencil: None,
            multisample: MultisampleState {
              count: 1,
              mask: !0,
              alpha_to_coverage_enabled: false,
            },
            fragment: Some(FragmentState {
              module: &vertex_shader,
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
          .insert(object.material().id().clone(), pipeline.clone());

        pipeline
      });

    self.subjects.insert(
      object.id(),
      Subject {
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
    let (Ok(output), Some((_, camera_bind_group))) =
      (renderer.surface().get_current_texture(), &self.camera)
    else {
      return;
    };

    let view = output.texture.create_view(&Default::default());

    let mut encoder = renderer
      .device()
      .create_command_encoder(&Default::default());

    {
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
        ..Default::default()
      });

      for Subject {
        transform: (_, transform_bind_group),
        vertices: (vertices, _),
        indices: (indices, index_count),
        pipeline,
      } in self.subjects.values()
      {
        render_pass.set_pipeline(pipeline);
        render_pass.set_bind_group(0, camera_bind_group, &[]);
        render_pass.set_bind_group(1, transform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertices.slice(..));
        render_pass.set_index_buffer(indices.slice(..), IndexFormat::Uint32);
        render_pass.draw_indexed(0..*index_count as u32, 0, 0..1);
      }
    }

    renderer.queue().submit([encoder.finish()]);
    output.present();
  }

  pub fn transform_rot(
    &self,
    renderer: &Renderer,
    resource: &mut (impl Resource + Transform),
    rot: Quat,
  ) {
    resource.set_rot(rot);

    let Some(Subject {
      transform: (transform, _),
      ..
    }) = &self.subjects.get(&resource.id())
    else {
      return;
    };

    renderer.queue().write_buffer(
      transform,
      0,
      bytemuck::cast_slice(&[
        Mat4::from_translation(resource.pos().clone()) * Mat4::from_quat(resource.rot().clone())
      ]),
    );
  }

  pub fn transform_pos(
    &self,
    renderer: &Renderer,
    resource: &mut (impl Resource + Transform),
    pos: Vec3,
  ) {
    resource.set_pos(pos);

    let Some(Subject {
      transform: (transform, _),
      ..
    }) = &self.subjects.get(&resource.id())
    else {
      return;
    };

    renderer.queue().write_buffer(
      transform,
      0,
      bytemuck::cast_slice(&[
        Mat4::from_translation(resource.pos().clone()) * Mat4::from_quat(resource.rot().clone())
      ]),
    );
  }

  pub fn transform_scale(
    &self,
    renderer: &Renderer,
    resource: &mut (impl Resource + Object3D),
    scale: Scale,
  ) {
    resource.set_scale(scale);

    let Some(Subject {
      transform: (transform, _),
      ..
    }) = &self.subjects.get(&resource.id())
    else {
      return;
    };

    renderer.queue().write_buffer(
      transform,
      0,
      bytemuck::cast_slice(&[
        Mat4::from_translation(resource.pos().clone()) * Mat4::from_quat(resource.rot().clone())
      ]),
    );
  }

  pub fn set_camera(&mut self, renderer: &Renderer, camera: &impl Camera) {
    let camera_buffer = renderer.device().create_buffer_init(&BufferInitDescriptor {
      label: None,
      contents: bytemuck::cast_slice(&[camera.to_matrix()]),
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let camera_layout = renderer
      .device()
      .create_bind_group_layout(&BindGroupLayoutDescriptor {
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

    let camera_bind_group = renderer.device().create_bind_group(&BindGroupDescriptor {
      label: Some("camera bind group"),
      layout: &camera_layout,
      entries: &[BindGroupEntry {
        binding: 0,
        resource: camera_buffer.as_entire_binding(),
      }],
    });

    self.camera = Some((camera_buffer, camera_bind_group));
  }

  pub fn update_camera(&self, renderer: &Renderer, camera: &impl Camera) {
    let Some((camera_buffer, _)) = &self.camera else {
      return;
    };

    renderer.queue().write_buffer(
      camera_buffer,
      0,
      bytemuck::cast_slice(&[camera.to_matrix()]),
    );
  }

  pub fn update_material(&mut self, resource: &mut (impl Resource + Object3D), material: Material) {
    resource.set_material(material);
  }

  pub fn update_geometry(
    &mut self,
    renderer: &Renderer,
    resource: &mut (impl Resource + Object3D),
    geometry: Geometry,
  ) {
    let vertices = (
      renderer.device().create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(geometry.vertices()),
        usage: BufferUsages::VERTEX,
      }),
      geometry.vertices().len(),
    );

    let indices = (
      renderer.device().create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(geometry.indices()),
        usage: BufferUsages::INDEX,
      }),
      geometry.indices().len(),
    );

    if let Some(subject) = self.subjects.get_mut(&resource.id()) {
      subject.indices = indices;
      subject.vertices = vertices;
    }

    resource.set_geometry(geometry);
  }
}
