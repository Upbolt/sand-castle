use crate::resource::texture::{Texture, TextureId};
use indexmap::IndexMap;

#[derive(Default)]
pub struct TextureLoader {
  textures: IndexMap<TextureId, Texture>,
}

impl TextureLoader {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn insert(&mut self, texture: Texture) {
    self.textures.insert(*texture.id(), texture);
  }

  pub fn get_from_id(&self, texture_id: TextureId) -> Option<&Texture> {
    self.textures.get(&texture_id)
  }
}
