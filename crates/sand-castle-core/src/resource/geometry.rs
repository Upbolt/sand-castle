use derive_getters::Getters;
use glam::Vec3;
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

pub mod cuboid;

#[derive(Getters, Default, Clone)]
pub struct Geometry {
  pub(crate) vertices: Vec<Vec3>,
  pub(crate) indices: Vec<u32>,
}

pub trait ToGeometry {
  fn to_geometry(&self) -> Geometry;
}

impl Geometry {
  pub(crate) fn vertex_desc() -> VertexBufferLayout<'static> {
    VertexBufferLayout {
      array_stride: size_of::<Vec3>() as BufferAddress,
      step_mode: VertexStepMode::Vertex,
      attributes: &[VertexAttribute {
        offset: 0,
        shader_location: 0,
        format: VertexFormat::Float32x3,
      }],
    }
  }
}
