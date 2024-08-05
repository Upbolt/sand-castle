use derive_builder::Builder;
use derive_getters::Getters;
use glam::{Mat4, Quat, Vec3};

use crate::resource::{
  object_3d::{Scale, Transform},
  Id, Resource,
};

use super::Camera;

#[derive(Getters, Builder)]
#[builder(pattern = "owned", build_fn(private, name = "fallible_build"))]
pub struct OrthographicCamera {
  #[getter(skip)]
  #[builder(default = "Default::default()", setter(skip))]
  id: Id,

  yaw: f32,
  pitch: f32,

  #[builder(default = "70.0")]
  fov: f32,
  aspect_ratio: f32,

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

impl Camera for OrthographicCamera {
  fn to_matrix(&self) -> Mat4 {
    let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
    let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

    Mat4::look_to_rh(
      self.position,
      Vec3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
      Vec3::Y,
    )
  }
}

impl Resource for OrthographicCamera {
  fn id(&self) -> Id {
    self.id
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
