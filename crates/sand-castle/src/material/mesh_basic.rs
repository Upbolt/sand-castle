use derive_builder::Builder;
use derive_getters::Getters;
use wgpu::ShaderSource;

use super::WithMaterial;

#[derive(Builder, Getters)]
pub struct MeshBasic {
  color: u32,
  wireframe: bool,
}

impl MeshBasic {
  pub fn builder() -> MeshBasicBuilder {
    MeshBasicBuilder::default()
  }
}

impl WithMaterial for MeshBasic {
  fn shader(&self) -> ShaderSource {
    ShaderSource::Wgsl(include_str!("shaders/mesh_basic.wgsl").into())
  }
}
