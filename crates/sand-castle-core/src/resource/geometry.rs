use std::mem::offset_of;

use bytemuck::{Pod, Zeroable};
use derive_more::From;
use getset::Getters;
use glam::{Vec2, Vec3};
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

use super::Id;

pub mod cuboid;

#[repr(C)]
#[derive(Getters, Default, Pod, Zeroable, From, Clone, Copy, Debug)]
pub struct Vertex {
  pub(crate) position: Vec3,
  pub(crate) normal: Vec3,
  pub(crate) tex_coords: Vec2,
}

#[derive(Getters, Default, Clone, Debug)]
#[getset(get = "pub")]
pub struct Geometry {
  pub(crate) id: Id,
  pub(crate) vertices: Vec<Vertex>,
  pub(crate) indices: Vec<u32>,
}

pub trait ToGeometry {
  fn to_geometry(&self) -> Geometry;
}

impl Vertex {
  pub fn new(position: Vec3, normal: Vec3, tex_coords: Vec2) -> Self {
    Self {
      position,
      normal,
      tex_coords,
    }
  }
}

impl Geometry {
  pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
    Self {
      id: Id::new(),
      vertices,
      indices,
    }
  }

  pub(crate) fn vertex_desc() -> VertexBufferLayout<'static> {
    VertexBufferLayout {
      array_stride: size_of::<Vertex>() as BufferAddress,
      step_mode: VertexStepMode::Vertex,
      attributes: &[
        VertexAttribute {
          offset: 0,
          shader_location: 0,
          format: VertexFormat::Float32x3,
        },
        VertexAttribute {
          offset: offset_of!(Vertex, normal) as u64,
          shader_location: 1,
          format: VertexFormat::Float32x3,
        },
        VertexAttribute {
          offset: offset_of!(Vertex, tex_coords) as u64,
          shader_location: 2,
          format: VertexFormat::Float32x2,
        },
      ],
    }
  }

  pub(crate) fn calculate_normals(vertices: &[Vec3], indices: &[u32]) -> Vec<Vec3> {
    let mut normals = vec![Vec3::ZERO; vertices.len()];

    for i in (0..indices.len()).step_by(3) {
      let v0 = vertices[indices[i] as usize];
      let v1 = vertices[indices[i + 1] as usize];
      let v2 = vertices[indices[i + 2] as usize];

      let normal = (v1 - v0).cross(v2 - v0).normalize(); // Compute face normal and normalize it.

      normals[indices[i] as usize] += normal;
      normals[indices[i + 1] as usize] += normal;
      normals[indices[i + 2] as usize] += normal;
    }

    // Normalize all the vertex normals
    for normal in &mut normals {
      if normal.length() > 0.0 {
        *normal = normal.normalize();
      }
    }

    normals
  }
}
