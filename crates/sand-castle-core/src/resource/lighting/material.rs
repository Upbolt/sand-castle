use derive_getters::Getters;
use wgpu::ShaderModuleDescriptor;

use crate::resource::{Id, Resource};

pub mod basic;
pub mod shader;

#[derive(Getters, Default)]
pub struct Material {
  pub(crate) id: Id,
  pub(crate) shader: Option<ShaderModuleDescriptor<'static>>,
}

pub trait ToMaterial {
  fn to_material(&self) -> Material;
}

impl Resource for Material {
  fn id(&self) -> Id {
    self.id
  }
}
