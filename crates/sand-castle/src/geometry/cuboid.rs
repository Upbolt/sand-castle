use derive_builder::Builder;
use derive_getters::Getters;
use wgpu::{BufferAddress, ShaderSource, VertexAttribute, VertexBufferLayout, VertexStepMode};

use crate::units::{Color, Vector3, Vertex};

use super::{Geometry, VerticesLayout, WithGeometry};

#[derive(Builder, Getters, Clone)]
pub struct Cuboid {
  #[builder(default = "1.")]
  width: f32,
  #[builder(default = "1.")]
  height: f32,
  #[builder(default = "1.")]
  depth: f32,
  #[builder(default = "1")]
  width_segments: u32,
  #[builder(default = "1")]
  height_segments: u32,
  #[builder(default = "1")]
  depth_segments: u32,
}

impl Cuboid {
  pub fn builder() -> CuboidBuilder {
    CuboidBuilder::default()
  }
}

impl WithGeometry for Cuboid {
  fn name() -> &'static str {
    "cuboid"
  }

  fn vertices_layout<'a>() -> VerticesLayout<'a> {
    // const ATTRIBUTES: [VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];

    VertexBufferLayout {
      array_stride: std::mem::size_of::<Vertex> as BufferAddress,
      step_mode: VertexStepMode::Vertex,
      attributes: &[wgpu::VertexAttribute {
        offset: 0,
        shader_location: 0,
        format: wgpu::VertexFormat::Float32x3,
      }],
    }
    .into()
  }

  fn into_geometry(self) -> Geometry {
    let (half_width, half_height, half_depth) =
      (self.width / 2., self.height / 2., self.depth / 2.);

    let (width_ratio, height_ratio, depth_ratio) = (
      self.width / self.width_segments as f32,
      self.height / self.height_segments as f32,
      self.depth / self.depth_segments as f32,
    );

    let mut vertices = Vec::with_capacity(
      ((self.width_segments + 1) + (self.height_segments + 1) + (self.depth_segments + 1)) as usize,
    );

    for x in 0..=self.width_segments {
      for y in 0..=self.height_segments {
        for z in 0..=self.depth_segments {
          vertices.push(Vertex {
            position: Vector3 {
              x: x as f32 * width_ratio,
              y: y as f32 * height_ratio,
              z: z as f32 * depth_ratio,
            },
          });
        }
      }
    }

    let left_indices = get_indices_from_vertices(&vertices, |(index, vertex)| {
      (vertex.position.x == 0.).then_some(index as u32)
    });
    let bottom_indices = get_indices_from_vertices(&vertices, |(index, vertex)| {
      (vertex.position.y == 0.).then_some(index as u32)
    });
    let back_indices = get_indices_from_vertices(&vertices, |(index, vertex)| {
      (vertex.position.z == 0.).then_some(index as u32)
    });
    let top_indices = get_indices_from_vertices(&vertices, |(index, vertex)| {
      (vertex.position.y == self.height).then_some(index as u32)
    });
    let right_indices = get_indices_from_vertices(&vertices, |(index, vertex)| {
      (vertex.position.x == self.width).then_some(index as u32)
    });
    let front_indices = get_indices_from_vertices(&vertices, |(index, vertex)| {
      (vertex.position.z == self.depth).then_some(index as u32)
    });

    let mut indices = vec![];
    indices.extend(left_indices);
    indices.extend(bottom_indices);
    indices.extend(back_indices);
    indices.extend(top_indices);
    indices.extend(right_indices);
    indices.extend(front_indices);

    for vertex in vertices.iter_mut() {
      vertex.position.x -= half_width;
      vertex.position.y -= half_height;
      vertex.position.z -= half_depth;
    }

    Geometry { vertices, indices }
  }
}

fn get_indices_from_vertices(
  vertices: &[Vertex],
  predicate: impl FnMut((usize, &Vertex)) -> Option<u32>,
) -> Vec<u32> {
  vertices
    .iter()
    .enumerate()
    .filter_map(predicate)
    .collect::<Vec<_>>()
    .windows(3)
    .map(|indices| indices.into_iter().map(|index| *index).collect::<Vec<_>>())
    .flatten()
    .collect::<Vec<_>>()
}
