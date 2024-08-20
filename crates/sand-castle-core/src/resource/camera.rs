use derive_builder::Builder;
use getset::Getters;
use glam::Mat4;

use super::object_3d::Transform;

pub mod orthographic;
pub mod perspective;

pub trait Camera: Transform {
  fn to_matrix(&self) -> Mat4;
}

#[derive(Getters, Builder, Clone, Debug)]
#[builder(pattern = "owned", build_fn(private, name = "fallible_build"))]
pub struct ViewFrustum {
  #[builder(default = "1000.0")]
  pub(crate) far: f32,
  #[builder(default = "0.1")]
  pub(crate) near: f32,
}

impl Default for ViewFrustum {
  fn default() -> Self {
    Self {
      far: 1000.0,
      near: 0.1,
    }
  }
}

impl ViewFrustumBuilder {
  pub fn build(self) -> ViewFrustum {
    self
      .fallible_build()
      .expect("could not build `ViewFrustum`")
  }
}
