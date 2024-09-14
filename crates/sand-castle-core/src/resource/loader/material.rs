use indexmap::IndexMap;

use crate::resource::{lighting::material::Material, Id};

#[derive(Default)]
pub struct MaterialLoader {
  materials: IndexMap<Id, Material>,
}

impl MaterialLoader {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn insert(&mut self, material: Material) {
    self.materials.insert(*material.id(), material);
  }

  pub fn get_from_id(&self, material_id: Id) -> Option<&Material> {
    self.materials.get(&material_id)
  }
}
