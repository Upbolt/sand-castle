use derive_builder::Builder;
use derive_getters::Getters;
use wgpu::{ShaderModuleDescriptor, ShaderSource};

use super::WithMaterial;

#[derive(Builder, Getters)]
pub struct Shader<'a> {
  color: u32,
  wireframe: bool,
  source: ShaderSource<'a>,
}

impl<'a> Shader<'a> {
  pub fn builder() -> ShaderBuilder<'a> {
    ShaderBuilder::default()
  }
}

impl<'a> WithMaterial for Shader<'a> {
  fn shader(&self) -> ShaderSource<'a> {
    self.source.clone()
  }
}
