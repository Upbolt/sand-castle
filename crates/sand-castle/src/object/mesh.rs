use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  BlendState, Buffer, BufferUsages, ColorTargetState, ColorWrites, Face, FragmentState, FrontFace,
  IndexFormat, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode,
  PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor,
  ShaderModuleDescriptor, VertexState,
};

use crate::{
  geometry::{Geometry, WithGeometry},
  material::{Material, WithMaterial},
  renderer::{Render, RenderOptions, RenderPass, Renderer},
  units::Vertex,
};

use derive_more::{From, Into};

#[derive(From, Into)]
pub struct VertexBuffer(Buffer);

#[derive(From, Into)]
pub struct IndexBuffer(Buffer);

#[derive(From, Into)]
pub struct Pipeline(RenderPipeline);

pub struct Mesh {
  vertex_buffer: VertexBuffer,
  index_buffer: IndexBuffer,
  indices: usize,
}

impl Mesh {
  pub fn new<G: WithGeometry>(
    renderer: &Renderer,
    geometry: G,
    material: impl WithMaterial,
  ) -> Self {
    Self::from_parts(renderer, geometry.into_geometry(), material.into_material())
  }

  pub fn from_parts(renderer: &Renderer, geometry: Geometry, material: Material) -> Self {
    let indices = geometry.indices();
    let indices_len = indices.len();

    let vertex_buffer = renderer
      .device()
      .create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(geometry.vertices()),
        usage: BufferUsages::VERTEX,
      })
      .into();

    let index_buffer = renderer
      .device()
      .create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(indices),
        usage: BufferUsages::INDEX,
      })
      .into();

    Self {
      vertex_buffer,
      index_buffer,
      indices: indices_len,
    }
  }
}

impl Render for Mesh {
  fn id(&self) -> u32 {
    0
  }

  fn render<'a, 'b: 'a>(&'b self, options: RenderOptions<'a, 'b>) {
    let RenderOptions {
      render_pass,
      camera,
      ..
    } = options;

    render_pass.set_pipeline(&options.pipeline);

    if let Some(camera) = camera {
      render_pass.set_bind_group(0, camera.bind_group(), &[]);
    }

    render_pass.set_vertex_buffer(0, self.vertex_buffer.0.slice(..));
    render_pass.set_index_buffer(self.index_buffer.0.slice(..), IndexFormat::Uint32);
    render_pass.draw_indexed(0..self.indices as u32, 0, 0..1);
  }
}
