use derive_builder::Builder;
use derive_getters::Getters;
use wgpu::ShaderSource;

use super::{Material, WithMaterial};

#[derive(Builder, Getters)]
pub struct Shader<'a> {
  color: u32,
  wireframe: bool,
  source: ShaderSource<'a>,
}

// impl<'a> Shader<'a> {
//   pub fn builder() -> ShaderBuilder<'a> {
//     ShaderBuilder::default()
//   }
// }

// impl<'a> WithMaterial for Shader<'a> {
//   fn into_material(self) -> Material<'a> {
//     Material {
//       shader: self.source.clone(),
//     }
//   }
// }
