use derive_builder::Builder;
use getset::{Getters, Setters};
use glam::Vec3;

use crate::resource::{Id, Resource};

#[derive(Getters, Setters, Builder, Debug, Clone)]
#[builder(pattern = "owned", build_fn(private, name = "fallible_build"))]
#[getset(get = "pub", set = "pub")]
pub struct DirectionalLight {
  #[getset(skip)]
  #[builder(default)]
  id: Id,
  #[builder(default)]
  color: Vec3,
  #[builder(default)]
  direction: Vec3,
}

impl DirectionalLight {
  pub fn builder() -> DirectionalLightBuilder {
    Default::default()
  }
}

impl Resource for DirectionalLight {
  fn id(&self) -> Id {
    self.id
  }
}

impl DirectionalLightBuilder {
  pub fn build(self) -> DirectionalLight {
    self
      .fallible_build()
      .expect("could not build `DirectionalLight`")
  }
}
