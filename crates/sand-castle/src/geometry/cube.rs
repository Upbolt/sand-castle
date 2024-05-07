use derive_builder::Builder;
use derive_getters::Getters;

use super::Geometry;

#[derive(Builder, Getters, Clone)]
pub struct Cube {
  width: f64,
  height: f64,
  depth: f64,
}

impl Cube {
  pub fn builder() -> CubeBuilder {
    CubeBuilder::default()
  }
}

impl Geometry for Cube {}
