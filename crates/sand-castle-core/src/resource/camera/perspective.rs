use derive_builder::Builder;
use derive_getters::Getters;
use glam::{Mat4, Quat, Vec3, Vec4};

use crate::resource::{
  object_3d::{Scale, Transform},
  Id, Resource,
};

use super::{Camera, ViewFrustum};

#[derive(Getters, Builder)]
#[builder(pattern = "owned", build_fn(private, name = "fallible_build"))]
pub struct PerspectiveCamera {
  #[getter(skip)]
  #[builder(default = "Default::default()", setter(skip))]
  id: Id,

  yaw: f32,
  pitch: f32,

  #[builder(default = "70.0")]
  fov: f32,
  aspect_ratio: f32,

  #[builder(default = "Default::default()")]
  view_frustum: ViewFrustum,

  #[getter(skip)]
  position: Vec3,
  #[getter(skip)]
  rotation: Quat,
  #[getter(skip)]
  scale: Scale,
}

impl PerspectiveCameraBuilder {
  pub fn build(self) -> PerspectiveCamera {
    self
      .fallible_build()
      .expect("could not build `PerspectiveCamera`")
  }
}

impl PerspectiveCamera {
  pub fn builder() -> PerspectiveCameraBuilder {
    Default::default()
  }
}

pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols(
  Vec4::new(1.0, 0.0, 0.0, 0.0),
  Vec4::new(0.0, 1.0, 0.0, 0.0),
  Vec4::new(0.0, 0.0, 0.5, 0.5),
  Vec4::new(0.0, 0.0, 0.0, 1.0),
);

impl Camera for PerspectiveCamera {
  fn to_matrix(&self) -> Mat4 {
    let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
    let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

    OPENGL_TO_WGPU_MATRIX
      * Mat4::perspective_rh(
        self.fov,
        self.aspect_ratio,
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

impl PerspectiveCamera {
  pub fn set_fov(&mut self, fov: f32) {
    self.fov = fov;
  }

  pub fn set_yaw(&mut self, yaw: f32) {
    self.yaw = yaw;
  }

  pub fn set_pitch(&mut self, pitch: f32) {
    self.pitch = pitch;
  }
}

impl Resource for PerspectiveCamera {
  fn id(&self) -> Id {
    self.id
  }
}

impl Transform for PerspectiveCamera {
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
