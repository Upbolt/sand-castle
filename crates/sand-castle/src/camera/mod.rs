use cgmath::Matrix4;
use derive_getters::Getters;
use wgpu::{BindGroup, BindGroupLayout, Buffer};

use crate::renderer::Renderer;

pub mod orthographic;
pub mod perspective;

#[derive(Getters)]
pub struct Camera {
  buffer: Buffer,
  bind_group: BindGroup,
}

impl Camera {
  pub fn update(&self, renderer: &Renderer, with_camera: &impl WithCamera) {
    renderer.queue().write_buffer(
      self.buffer(),
      0,
      bytemuck::cast_slice(&[with_camera.view_projection()]),
    );
  }
}

pub trait WithCamera {
  fn to_camera(&self, renderer: &Renderer) -> Camera;

  fn view_matrix(&self) -> Matrix4<f32>;
  fn view_projection(&self) -> ViewProjection {
    ViewProjection(self.view_matrix().into())
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ViewProjection([[f32; 4]; 4]);

#[derive(Default, Clone)]
pub struct ViewFrustum {
  pub near: f32,
  pub far: f32,
}
