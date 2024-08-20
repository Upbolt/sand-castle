use std::any::TypeId;

use basic::BasicMaterial;
use getset::Getters;
use shader::ShaderMaterial;
use wgpu::{BindGroupLayoutDescriptor, ShaderModuleDescriptor};

pub mod basic;
pub mod phong;
pub mod shader;

#[derive(Getters, Debug, Clone)]
#[getset(get = "pub")]
pub struct Material {
  pub(crate) shader_type: TypeId,
  pub(crate) vertex_shader: ShaderModuleDescriptor<'static>,
  pub(crate) fragment_shader: ShaderModuleDescriptor<'static>,
  pub(crate) fragment_data: Vec<u8>,
  pub(crate) fragment_data_layout: BindGroupLayoutDescriptor<'static>,
}

pub trait ToMaterial {
  fn to_material(&self) -> Material;
}

impl From<BasicMaterial> for Material {
  fn from(value: BasicMaterial) -> Self {
    value.to_material()
  }
}

impl From<ShaderMaterial> for Material {
  fn from(value: ShaderMaterial) -> Self {
    value.to_material()
  }
}
