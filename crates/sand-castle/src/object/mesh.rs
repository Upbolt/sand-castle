use derive_builder::Builder;
use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  BlendState, Buffer, BufferUsages, Color, ColorTargetState, ColorWrites, CommandEncoderDescriptor,
  Face, FragmentState, FrontFace, IndexFormat, LoadOp, MultisampleState, Operations,
  PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode, PrimitiveState,
  PrimitiveTopology, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
  RenderPipelineDescriptor, ShaderModuleDescriptor, StoreOp, TextureFormat, TextureViewDescriptor,
  VertexState,
};

use crate::{
  geometry::{Geometry, VerticesLayout, WithGeometry},
  material::{Material, WithMaterial},
  renderer::{GpuCommand, Render, RenderPass, Renderer},
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
  pipeline: RenderPipeline,
  index_buffer: IndexBuffer,
  indices: usize,
}

impl Mesh {
  pub fn new<G: WithGeometry>(
    renderer: &Renderer,
    geometry: G,
    material: impl WithMaterial,
  ) -> Self {
    let geometry = geometry.into_geometry();
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

    let pipeline_layout = renderer
      .device()
      .create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
      });

    let vertex_shader = renderer
      .device()
      .create_shader_module(ShaderModuleDescriptor {
        label: Some("Mesh Vertex Shader"),
        source: geometry.shader().clone(),
      });

    let fragment_shader = renderer
      .device()
      .create_shader_module(ShaderModuleDescriptor {
        label: Some("Mesh Fragment Shader"),
        source: material.shader(),
      });

    let pipeline = renderer
      .device()
      .create_render_pipeline(&RenderPipelineDescriptor {
        vertex: VertexState {
          buffers: &[G::vertices_layout().into()],
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
            format: renderer.surface_config().format,
            blend: Some(BlendState::REPLACE),
            write_mask: ColorWrites::ALL,
          })],
        }),
        multiview: None,
      });

    Self {
      pipeline,
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

  fn render<'a, 'b: 'a>(&'b self, renderer: &'a Renderer, render_pass: &'a mut RenderPass<'b>) {
    render_pass.set_pipeline(&self.pipeline);
    render_pass.set_vertex_buffer(0, self.vertex_buffer.0.slice(..));
    render_pass.set_index_buffer(self.index_buffer.0.slice(..), IndexFormat::Uint32);
    render_pass.draw_indexed(0..self.indices as u32, 0, 0..1);
  }
}
