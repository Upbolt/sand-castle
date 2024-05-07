use derive_builder::Builder;
use derive_getters::Getters;

use crate::units::Vector3;

use super::{Camera, ViewFrustum};

#[derive(Builder, Getters)]
pub struct PerspectiveCamera {
  field_of_view: f32,
  aspect_ratio: f64,
  view_frustum: ViewFrustum,
  position: Vector3,
}

impl PerspectiveCamera {
  pub fn builder() -> PerspectiveCameraBuilder {
    PerspectiveCameraBuilder::default()
  }
}

impl Camera for PerspectiveCamera {
  fn view() {}
}
