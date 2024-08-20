use derive_builder::Builder;
use getset::{Getters, Setters};
use glam::Vec3;

#[derive(Getters, Setters, Builder, Default, Debug, Clone)]
#[builder(pattern = "owned", build_fn(private, name = "fallible_build"))]
#[getset(get = "pub", set = "pub")]
pub struct AmbientLight {
  #[builder(default)]
  #[getset(get = "pub", set = "pub")]
  color: Vec3,
}

impl AmbientLight {
  pub fn builder() -> AmbientLightBuilder {
    Default::default()
  }
}

impl AmbientLightBuilder {
  pub fn build(self) -> AmbientLight {
    self
      .fallible_build()
      .expect("could not build `AmbientLight`")
  }
}
