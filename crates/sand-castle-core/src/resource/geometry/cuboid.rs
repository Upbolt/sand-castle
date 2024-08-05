use derive_builder::Builder;
use derive_getters::Getters;
use glam::Vec3;

use super::{Geometry, ToGeometry};

#[derive(Getters, Builder)]
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
    let vertices = vec![
      [-1.0, -1.0, 1.0],
      [1.0, -1.0, 1.0],
      [1.0, 1.0, 1.0],
      [-1.0, 1.0, 1.0],
      // Back face
      [-1.0, -1.0, -1.0],
      [1.0, -1.0, -1.0],
      [1.0, 1.0, -1.0],
      [-1.0, 1.0, -1.0],
    ]
    .into_iter()
    .map(|vertices| Vec3::from_array(vertices))
    .collect::<Vec<_>>();

    Geometry {
      vertices,
      indices: vec![
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
      ],
    }
  }
}
