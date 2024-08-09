use derive_getters::Getters;
use glam::{Quat, Vec3};

use super::{geometry::Geometry, lighting::material::Material};

pub mod mesh;

#[derive(Getters, Clone, Copy, Debug, PartialEq)]
pub struct Scale {
  pub width: f32,
  pub height: f32,
  pub depth: f32,
}

impl Default for Scale {
  fn default() -> Self {
    Self {
      width: 1.0,
      height: 1.0,
      depth: 1.0,
    }
  }
}

pub trait Object3D: Transform {
  fn geometry(&self) -> &Geometry;
  fn material(&self) -> Option<&Material>;

  fn set_geometry(&mut self, geometry: Geometry);
  fn set_material(&mut self, material: Material);
}

pub trait Transform {
  fn rot(&self) -> &Quat;
  fn pos(&self) -> &Vec3;
  fn scale(&self) -> &Scale;

  fn set_rot(&mut self, rot: Quat);
  fn set_pos(&mut self, pos: Vec3);
  fn set_scale(&mut self, scale: Scale);
}
