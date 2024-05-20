use cgmath::{perspective, Deg, Matrix4, Point3, Vector3};
use derive_builder::Builder;
use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
  BindingType, BufferBindingType, BufferUsages, ShaderStages,
};

use crate::renderer::Renderer;

use super::{Camera, ViewFrustum, WithCamera};

#[derive(Builder)]
pub struct OrthographicCamera {
  #[builder(default = "(0., 1., 2.).into()")]
  eye: Point3<f32>,
  #[builder(default = "(0., 0., 0.).into()")]
  target: Point3<f32>,
  #[builder(default = "Vector3::unit_y()")]
  up: Vector3<f32>,
  aspect_ratio: f32,
  #[builder(default = "70.")]
  field_of_view: f32,
  view_frustum: ViewFrustum,
}

impl OrthographicCamera {
  pub fn builder() -> OrthographicCameraBuilder {
    OrthographicCameraBuilder::default()
  }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
  1.0, 0.0, 0.0, 0.0,
  0.0, 1.0, 0.0, 0.0,
  0.0, 0.0, 0.5, 0.5,
  0.0, 0.0, 0.0, 1.0,
);

impl WithCamera for OrthographicCamera {
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
    let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
    let projection = perspective(
      Deg(self.field_of_view),
      self.aspect_ratio,
      self.view_frustum.near,
      self.view_frustum.far,
    );

    OPENGL_TO_WGPU_MATRIX * projection * view
  }
}
