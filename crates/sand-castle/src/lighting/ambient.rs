use derive_builder::Builder;

#[derive(Builder)]
pub struct AmbientLight {
  color: u32,
}

impl AmbientLight {
  pub fn builder() -> AmbientLightBuilder {
    AmbientLightBuilder::default()
  }
}
