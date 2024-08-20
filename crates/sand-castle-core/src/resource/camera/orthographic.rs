use derive_builder::Builder;
use getset::Getters;
use glam::{Mat4, Quat, Vec2, Vec3, Vec4};

use crate::resource::object_3d::{Scale, Transform};

use super::{Camera, ViewFrustum};

#[derive(Getters, Builder, Debug)]
#[getset(get = "pub")]
#[builder(pattern = "owned", build_fn(private, name = "fallible_build"))]
pub struct OrthographicCamera {
  yaw: f32,
  pitch: f32,

  screen_size: Vec2,

  #[builder(default = "Default::default()")]
  view_frustum: ViewFrustum,

  position: Vec3,
  rotation: Quat,
  scale: Scale,
}

impl OrthographicCameraBuilder {
  pub fn build(self) -> OrthographicCamera {
    self
      .fallible_build()
      .expect("could not build `OrthographicCamera`")
  }
}

impl OrthographicCamera {
  pub fn builder() -> OrthographicCameraBuilder {
    Default::default()
  }
}

pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols(
  Vec4::new(1.0, 0.0, 0.0, 0.0),
  Vec4::new(0.0, 1.0, 0.0, 0.0),
  Vec4::new(0.0, 0.0, 0.5, 0.5),
  Vec4::new(0.0, 0.0, 0.0, 1.0),
);

impl Camera for OrthographicCamera {
  fn to_matrix(&self) -> Mat4 {
    let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
    let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

    OPENGL_TO_WGPU_MATRIX
      * Mat4::orthographic_lh(
        0.0,
        self.screen_size.x,
        self.screen_size.y,
        0.0,
        self.view_frustum.near,
        self.view_frustum.far,
      )
      * Mat4::look_to_rh(
        self.position,
        Vec3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
        Vec3::Y,
      )
  }
}

impl Transform for OrthographicCamera {
  fn pos(&self) -> &Vec3 {
    &self.position
  }

  fn rot(&self) -> &Quat {
    &self.rotation
  }

  fn scale(&self) -> &Scale {
    &self.scale
  }

  fn set_pos(&mut self, pos: Vec3) {
    self.position = pos;
  }

  fn set_rot(&mut self, rot: Quat) {
    self.rotation = rot;
  }

  fn set_scale(&mut self, scale: Scale) {
    self.scale = scale;
  }
}
