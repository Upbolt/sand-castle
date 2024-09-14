use indexmap::IndexMap;

use crate::resource::{geometry::Geometry, Id};

#[derive(Default)]
pub struct GeometryLoader {
  geometries: IndexMap<Id, Geometry>,
}

impl GeometryLoader {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn insert(&mut self, geometry: Geometry) {
    self.geometries.insert(*geometry.id(), geometry);
  }

  pub fn get_from_id(&self, id: Id) -> Option<&Geometry> {
    self.geometries.get(&id)
  }
}
