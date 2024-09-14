use derive_builder::Builder;
use getset::Getters;
use glam::Vec3;

use crate::resource::Id;

use super::{Geometry, ToGeometry, Vertex};

#[derive(Getters, Builder, Debug, Clone)]
#[getset(get = "pub", set = "pub")]
#[builder(pattern = "owned", build_fn(private, name = "fallible_build"))]
pub struct Cuboid {
  #[builder(default = "1.0")]
  width: f32,
  #[builder(default = "1.0")]
  height: f32,
  #[builder(default = "1.0")]
  depth: f32,
  #[builder(default = "1")]
  width_segments: u32,
  #[builder(default = "1")]
  height_segments: u32,
  #[builder(default = "1")]
  depth_segments: u32,
}

impl CuboidBuilder {
  pub fn build(self) -> Cuboid {
    self.fallible_build().expect("could not build `Cuboid`")
  }
}

impl Cuboid {
  pub fn builder() -> CuboidBuilder {
    CuboidBuilder::default()
  }
}

impl ToGeometry for Cuboid {
  fn to_geometry(&self) -> Geometry {
    let vertices = [
      Vec3::new(-1.0, -1.0, 1.0),
      Vec3::new(1.0, -1.0, 1.0),
      Vec3::new(1.0, 1.0, 1.0),
      Vec3::new(-1.0, 1.0, 1.0),
      // Back face
      Vec3::new(-1.0, -1.0, -1.0),
      Vec3::new(1.0, -1.0, -1.0),
      Vec3::new(1.0, 1.0, -1.0),
      Vec3::new(-1.0, 1.0, -1.0),
    ];

    let indices = vec![
      0, 1, 2, //Front face
      0, 2, 3, //
      4, 6, 5, //Back face
      4, 7, 6, //
      4, 5, 1, //Left face
      4, 1, 0, //
      3, 2, 6, //Right face
      3, 6, 7, //
      1, 5, 6, //Top face
      1, 6, 2, //
      4, 0, 3, //Bottom face
      4, 3, 7, //
    ];

    let normals = Geometry::calculate_normals(&vertices, &indices);

    Geometry {
      id: Id::new(),
      vertices: vertices
        .into_iter()
        .zip(normals.into_iter())
        .map(|(position, normal)| Vertex {
          position,
          normal,
          tex_coords: Default::default(),
        })
        .collect::<Vec<_>>(),
      indices,
    }
  }
}
