pub mod instanced;

use derive_builder::Builder;
use getset::Getters;
use glam::{Mat4, Quat, Vec3};

use super::{Object3D, Scale, SceneTransform, Transform};
use crate::{
  renderer::Renderer,
  resource::{Id, Resource},
  scene::{Scene, Subject},
};

#[derive(Debug, Getters, Builder)]
#[builder(pattern = "owned", build_fn(private, name = "fallible_build"))]
pub struct Mesh {
  #[builder(setter(skip))]
  #[getset(skip)]
  id: Id,

  #[builder(default, setter(strip_option))]
  geometry_id: Option<Id>,
  #[builder(default, setter(strip_option))]
  material_id: Option<Id>,

  #[getset(skip)]
  scale: Scale,
  #[getset(skip)]
  position: Vec3,
  #[getset(skip)]
  rotation: Quat,
}

impl MeshBuilder {
  pub fn build(self) -> Mesh {
    self.fallible_build().expect("failed to build `Mesh`")
  }
}

impl Mesh {
  pub fn builder() -> MeshBuilder {
    MeshBuilder::default()
  }
}

impl Resource for Mesh {
  fn id(&self) -> Id {
    self.id
  }
}

impl Object3D for Mesh {
  fn geometry_id(&self) -> Option<Id> {
    self.geometry_id
  }

  fn material_id(&self) -> Option<Id> {
    self.material_id
  }

  fn set_geometry_id(&mut self, geometry_id: Id) {
    self.geometry_id = Some(geometry_id);
  }

  fn set_material_id(&mut self, material_id: Id) {
    self.material_id = Some(material_id);
  }
}

impl Transform for Mesh {
  fn pos(&self) -> &Vec3 {
    &self.position
  }

  fn rot(&self) -> &Quat {
    &self.rotation
  }

  fn set_pos(&mut self, pos: Vec3) {
    self.position = pos;
  }

  fn set_rot(&mut self, rot: Quat) {
    self.rotation = rot;
  }

  fn scale(&self) -> &Scale {
    &self.scale
  }

  fn set_scale(&mut self, dim: Scale) {
    self.scale = dim;
  }
}

impl SceneTransform for Mesh {
  fn update_pos(&mut self, scene: &Scene, renderer: &Renderer, pos: Vec3) {
    self.set_pos(pos);

    let Some(Subject {
      transform: (transform, _),
      normal: (normal, _),
      ..
    }) = &scene.subjects().get(&self.id())
    else {
      return;
    };

    renderer.queue().write_buffer(
      transform,
      0,
      bytemuck::cast_slice(&[Mat4::from_translation(pos) * Mat4::from_quat(self.rot().clone())]),
    );
    renderer.queue().write_buffer(
      normal,
      0,
      bytemuck::cast_slice(&[Mat4::from_quat(self.rot().clone())]),
    );
  }

  fn update_rot(&mut self, scene: &Scene, renderer: &Renderer, rot: Quat) {
    self.set_rot(rot);

    let Some(Subject {
      transform: (transform, _),
      normal: (normal, _),
      ..
    }) = &scene.subjects().get(&self.id())
    else {
      return;
    };

    renderer.queue().write_buffer(
      transform,
      0,
      bytemuck::cast_slice(&[Mat4::from_translation(self.pos().clone()) * Mat4::from_quat(rot)]),
    );
    renderer.queue().write_buffer(
      normal,
      0,
      bytemuck::cast_slice(&[Mat4::from_quat(self.rot().clone())]),
    );
  }

  fn update_scale(&mut self, scene: &Scene, renderer: &Renderer, scale: Scale) {
    self.set_scale(scale);

    let Some(Subject {
      transform: (transform, _),
      normal: (normal, _),
      ..
    }) = &scene.subjects().get(&self.id())
    else {
      return;
    };

    renderer.queue().write_buffer(
      transform,
      0,
      bytemuck::cast_slice(&[
        Mat4::from_translation(self.pos().clone()) * Mat4::from_quat(self.rot().clone())
      ]),
    );
    renderer.queue().write_buffer(
      normal,
      0,
      bytemuck::cast_slice(&[Mat4::from_quat(self.rot().clone())]),
    );
  }
}
