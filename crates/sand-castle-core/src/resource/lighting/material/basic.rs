use derive_getters::Getters;
use wgpu::include_wgsl;

use super::{Material, ToMaterial};

#[derive(Getters)]
pub struct BasicMaterial {
  color: u32,
}

impl BasicMaterial {
  pub fn with_color(color: u32) -> Self {
    Self { color }
  }
}

impl ToMaterial for BasicMaterial {
  fn to_material(&self) -> Material {
    Material {
      id: Default::default(),
      shader: Some(include_wgsl!("shaders/basic.wgsl")),
    }
  }
}
