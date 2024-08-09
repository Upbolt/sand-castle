use derive_builder::Builder;
use derive_getters::Getters;
use glam::{Quat, Vec3};

use super::{Object3D, Scale, Transform};
use crate::resource::{geometry::Geometry, lighting::material::Material, Id, Resource};

#[derive(Debug, Getters, Builder)]
#[builder(pattern = "owned", build_fn(private, name = "fallible_build"))]
pub struct Mesh {
  #[builder(setter(skip))]
  #[getter(skip)]
  id: Id,

  #[builder(default)]
  geometry: Geometry,
  #[builder(default, setter(strip_option))]
  material: Option<Material>,

  #[getter(skip)]
  scale: Scale,
  #[getter(skip)]
  position: Vec3,
  #[getter(skip)]
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
  fn geometry(&self) -> &Geometry {
    &self.geometry
  }

  fn material(&self) -> Option<&Material> {
    self.material.as_ref()
  }

  fn set_geometry(&mut self, geometry: Geometry) {
    self.geometry = geometry;
  }

  fn set_material(&mut self, material: Material) {
    self.material = Some(material);
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
