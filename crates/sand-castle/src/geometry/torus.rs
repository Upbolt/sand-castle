use derive_builder::Builder;
use derive_getters::Getters;

use super::Geometry;

#[derive(Builder, Getters, Clone)]
pub struct Torus {
  radius: f32,
  tube: f32,
  radial_segments: u32,
  arc: u32,
}

impl Torus {
  pub fn builder() -> TorusBuilder {
    TorusBuilder::default()
  }
}

impl Geometry for Torus {}
