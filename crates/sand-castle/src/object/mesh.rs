use wgpu::{
  Color, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment,
  RenderPassDescriptor, StoreOp, TextureViewDescriptor,
};

use crate::{
  geometry::Geometry,
  material::Material,
  renderer::{Render, Renderer},
};

pub struct Mesh<T: Geometry, U: Material> {
  geometry: T,
  material: U,
}

impl<T: Geometry, U: Material> Mesh<T, U> {
  pub fn from_geometry(geometry: T, material: U) -> Self {
    Self { geometry, material }
  }
}

impl<T: Geometry, U: Material> Render for Mesh<T, U> {
  fn id(&self) -> u32 {
    0
  }

  fn render(&self, renderer: &Renderer) {}
}
