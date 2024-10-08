use getset::Getters;
use glam::{Quat, Vec3};

use crate::{renderer::Renderer, scene::Scene};

use super::{Id, Resource};

pub mod mesh;

#[derive(Getters, Clone, Copy, Debug, PartialEq)]
#[getset(get = "pub")]
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
  fn geometry_id(&self) -> Option<Id>;
  fn material_id(&self) -> Option<Id>;

  fn set_geometry_id(&mut self, geometry_id: Id);
  fn set_material_id(&mut self, material_id: Id);
}

pub trait Transform {
  fn rot(&self) -> &Quat;
  fn pos(&self) -> &Vec3;
  fn scale(&self) -> &Scale;

  fn set_rot(&mut self, rot: Quat);
  fn set_pos(&mut self, pos: Vec3);
  fn set_scale(&mut self, scale: Scale);
}

pub trait SceneTransform: Transform + Resource {
  fn update_rot(&mut self, scene: &Scene, renderer: &Renderer, rot: Quat);
  fn update_pos(&mut self, scene: &Scene, renderer: &Renderer, pos: Vec3);
  fn update_scale(&mut self, scene: &Scene, renderer: &Renderer, scale: Scale);
}
