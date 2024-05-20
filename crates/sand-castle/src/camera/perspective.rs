use cgmath::{perspective, Angle, Deg, EuclideanSpace, InnerSpace, Matrix4, Point3, Rad, Vector3};
use derive_builder::Builder;
use derive_getters::Getters;
use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
  BindingType, BufferBindingType, BufferUsages, ShaderStages,
};

use crate::renderer::Renderer;

use super::{Camera, ViewFrustum, WithCamera};

#[derive(Builder, Getters)]
pub struct PerspectiveCamera {
  #[builder(default = "(0., 5., 10.).into()")]
  position: Point3<f32>,
  #[builder(default = "Deg(-90.).into()")]
  yaw: Rad<f32>,
  #[builder(default = "Deg(-20.).into()")]
  pitch: Rad<f32>,

  aspect_ratio: f32,
  #[builder(default = "70.")]
  field_of_view: f32,
  #[builder(default = "ViewFrustum { near: 0.1, far: 1000.0 }")]
  view_frustum: ViewFrustum,
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
  1.0, 0.0, 0.0, 0.0,
  0.0, 1.0, 0.0, 0.0,
  0.0, 0.0, 0.5, 0.5,
  0.0, 0.0, 0.0, 1.0,
);

impl PerspectiveCamera {
  pub fn builder() -> PerspectiveCameraBuilder {
    PerspectiveCameraBuilder::default()
  }

  pub fn position_mut(&mut self) -> &mut Point3<f32> {
    &mut self.position
  }

  pub fn pitch_mut(&mut self) -> &mut Rad<f32> {
    &mut self.pitch
  }

  pub fn yaw_mut(&mut self) -> &mut Rad<f32> {
    &mut self.yaw
  }
}

impl WithCamera for PerspectiveCamera {
  fn to_camera(&self, renderer: &Renderer) -> Camera {
    let buffer = renderer.device().create_buffer_init(&BufferInitDescriptor {
      label: None,
      contents: bytemuck::cast_slice(&[self.view_projection()]),
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let bind_group = renderer.device().create_bind_group(&BindGroupDescriptor {
      label: None,
      layout: renderer.camera_bind_group_layout(),
      entries: &[BindGroupEntry {
        binding: 0,
        resource: buffer.as_entire_binding(),
      }],
    });

    Camera { buffer, bind_group }
  }

  fn view_matrix(&self) -> Matrix4<f32> {
    let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
    let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

    let view = Matrix4::look_at_rh(
      self.position,
      Point3::from_vec(
        Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
      ),
      Vector3::unit_y(),
    );

    let projection = perspective(
      Deg(self.field_of_view),
      self.aspect_ratio,
      self.view_frustum.near,
      self.view_frustum.far,
    );

    OPENGL_TO_WGPU_MATRIX * projection * view
  }
}
