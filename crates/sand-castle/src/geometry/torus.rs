use derive_builder::Builder;
use derive_getters::Getters;
use wgpu::{ShaderSource, VertexAttribute, VertexBufferLayout, VertexStepMode};

use crate::units::Vertex;

use super::{Geometry, VerticesLayout, WithGeometry};

#[derive(Builder, Getters, Clone)]
pub struct Torus {
  #[builder(default = "1.")]
  radius: f32,
  #[builder(default = "0.4")]
  tube: f32,
  #[builder(default = "12")]
  radial_segments: u32,
  #[builder(default = "48")]
  tubular_segments: u32,
  #[builder(default = "std::f64::consts::PI * 2.")]
  arc: f64,
}

impl Torus {
  pub fn builder() -> TorusBuilder {
    TorusBuilder::default()
  }
}

impl WithGeometry for Torus {
  fn vertices_layout() -> VerticesLayout {
    const ATTRIBUTES: [VertexAttribute; 2] =
      wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    VertexBufferLayout {
      array_stride: std::mem::size_of::<Vertex> as u64,
      step_mode: VertexStepMode::Vertex,
      attributes: &ATTRIBUTES,
    }
    .into()
  }

  fn into_geometry<'a>(self) -> Geometry<'a> {
    Geometry {
      shader: ShaderSource::Wgsl(include_str!("shaders/torus.wgsl").into()),
      vertices: vec![],
      indices: vec![],
    }
  }
}
