use derive_builder::Builder;
use derive_getters::Getters;

use super::Geometry;

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

impl Geometry for Torus {}
