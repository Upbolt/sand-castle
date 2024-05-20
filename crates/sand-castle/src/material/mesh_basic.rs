use derive_builder::Builder;
use derive_getters::Getters;
use wgpu::ShaderSource;

use super::{Material, WithMaterial};

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
  fn into_material<'a>(self) -> Material<'a> {
    Material {
      shader: ShaderSource::Wgsl(include_str!("shaders/mesh_basic.wgsl").into()),
    }
  }
}
