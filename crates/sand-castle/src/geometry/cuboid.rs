use derive_builder::Builder;
use derive_getters::Getters;

use super::Geometry;

#[derive(Builder, Getters, Clone)]
pub struct Cuboid {
  #[builder(default = "1.")]
  width: f64,
  #[builder(default = "1.")]
  height: f64,
  #[builder(default = "1.")]
  depth: f64,
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

impl Geometry for Cuboid {}
