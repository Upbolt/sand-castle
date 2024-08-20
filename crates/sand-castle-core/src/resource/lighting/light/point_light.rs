use derive_builder::Builder;
use getset::{Getters, Setters};
use glam::Vec3;

use crate::resource::{Id, Resource};

#[derive(Builder, Getters, Setters, Clone, Debug)]
#[getset(get = "pub", set = "pub")]
#[builder(pattern = "owned", build_fn(private, name = "fallible_build"))]
pub struct PointLight {
  #[getset(skip)]
  #[builder(default)]
  id: Id,
  #[builder(default = "Vec3::new(1.0, 1.0, 1.0)")]
  color: Vec3,
  #[builder(default)]
  position: Vec3,
}

impl PointLight {
  pub fn builder() -> PointLightBuilder {
    Default::default()
  }
}

impl Resource for PointLight {
  fn id(&self) -> Id {
    self.id
  }
}

impl PointLightBuilder {
  pub fn build(self) -> PointLight {
    self.fallible_build().expect("could not build `PointLight`")
  }
}
