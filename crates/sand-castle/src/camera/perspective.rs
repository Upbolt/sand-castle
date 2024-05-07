use derive_builder::Builder;

use super::Camera;

#[derive(Default, Clone)]
pub struct ViewFrustum {
  pub near: f32,
  pub far: f32,
}

#[derive(Builder)]
pub struct PerspectiveCamera {
  field_of_view: u8,
  aspect_ratio: f32,
  view_frustum: ViewFrustum,
}

impl PerspectiveCamera {
  pub fn builder() -> PerspectiveCameraBuilder {
    PerspectiveCameraBuilder::default()
  }
}

impl Camera for PerspectiveCamera {
  fn view() {}
}
