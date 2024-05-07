use derive_builder::Builder;
use derive_getters::Getters;

use super::Material;

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

impl Material for MeshBasic {}
