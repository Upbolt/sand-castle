pub mod camera;
pub mod geometry;
pub mod lighting;
pub mod object_3d;
pub mod texture;

pub mod loader;

use derive_more::{Deref, From};
use uuid::Uuid;

#[derive(Debug, Deref, From, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Id(u128);

impl Id {
  pub fn new() -> Self {
    Self(Uuid::new_v4().as_u128())
  }
}

impl Default for Id {
  fn default() -> Self {
    Self::new()
  }
}

pub trait Resource {
  fn id(&self) -> Id;
}
