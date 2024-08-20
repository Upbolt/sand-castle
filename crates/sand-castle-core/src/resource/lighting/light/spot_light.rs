use derive_builder::Builder;
use getset::{Getters, Setters};
use glam::Vec3;

use crate::resource::{Id, Resource};

#[derive(Builder, Getters, Setters, Clone, Debug)]
#[getset(get = "pub", set = "pub")]
#[builder(pattern = "owned", build_fn(private, name = "fallible_build"))]
pub struct SpotLight {
  #[getset(skip)]
  #[builder(default)]
  id: Id,
  #[builder(default = "Vec3::new(1.0, 1.0, 1.0)")]
  color: Vec3,
  #[builder(default)]
  position: Vec3,
  #[builder(default)]
  direction: Vec3,
  #[builder(default)]
  cutoff_angle: f32,
}

impl SpotLight {
  pub fn builder() -> SpotLightBuilder {
    Default::default()
  }
}

impl Resource for SpotLight {
  fn id(&self) -> Id {
    self.id
  }
}

impl SpotLightBuilder {
  pub fn build(self) -> SpotLight {
    self.fallible_build().expect("could not build `SpotLight`")
  }
}
