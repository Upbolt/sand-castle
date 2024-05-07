use derive_builder::Builder;

use super::Camera;

#[derive(Builder)]
pub struct OrthographicCamera {}

impl OrthographicCamera {
  pub fn builder() -> OrthographicCameraBuilder {
    OrthographicCameraBuilder::default()
  }
}

impl Camera for OrthographicCamera {
  fn view() {}
}
