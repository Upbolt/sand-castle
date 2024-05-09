use derive_builder::Builder;
use derive_getters::Getters;

use super::Material;

#[derive(Builder, Getters)]
pub struct Shader {
  color: u32,
  wireframe: bool,
}

impl Shader {
  pub fn builder() -> ShaderBuilder {
    ShaderBuilder::default()
  }
}

impl Material for Shader {}
