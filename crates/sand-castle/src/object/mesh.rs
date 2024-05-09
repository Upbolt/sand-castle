use wgpu::{
  Color, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment,
  RenderPassDescriptor, StoreOp, TextureViewDescriptor,
};

use crate::{
  geometry::Geometry,
  material::Material,
  renderer::{Render, Renderer},
};

pub struct Mesh {
  geometry: Box<dyn Geometry>,
  material: Box<dyn Material>,
}

impl Mesh {
  pub fn from_geometry(
    geometry: impl Geometry + 'static,
    material: impl Material + 'static,
  ) -> Self {
    Self {
      geometry: Box::new(geometry),
      material: Box::new(material),
    }
  }
}

impl Render for Mesh {
  fn id(&self) -> u32 {
    0
  }

  fn render(&self, renderer: &Renderer) {}
}
