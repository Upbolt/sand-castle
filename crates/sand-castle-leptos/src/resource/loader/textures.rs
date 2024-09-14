use leptos::prelude::*;
use sand_castle_core::resource::{
  loader::textures::TextureLoader as CoreTextureLoader,
  // texture::{/*FromUrlError,*/ Texture},
  texture::{FromUrlError, Texture},
};

use crate::scene::SceneContextValue;

pub use sand_castle_core::resource::texture::TextureId;

#[derive(Debug, Clone)]
enum LoadTextureError {
  FromUrl(FromUrlError),
}

pub fn use_texture_loader(
  url: impl Into<MaybeProp<String>>,
  // ) -> Signal<Option</*Result<TextureId, FromUrlError>*/ TextureId>> {
) -> Signal<Option<Result<TextureId, FromUrlError>>, LocalStorage> {
  let SceneContextValue { texture_loader, .. } = use_context()
    .expect("`use_texture_loader` must be used in a component inside of a `Scene` component");
  let url: MaybeProp<String> = url.into();

  let texture_id = AsyncDerived::new_unsync(move || {
    let url = url.clone();
    async move {
      if texture_loader.with(|loader| loader.is_none()) {
        return None;
      }

      let texture = match Texture::from_url(&url.get()?).await {
        Ok(texture) => texture,
        Err(err) => {
          return Some(Err(err));
        }
      };
      let texture_id = *texture.id();

      texture_loader.update(|loader| {
        if let Some(loader) = loader {
          loader.insert(texture);
        }
      });

      Some(Ok(texture_id))
    }
  });

  Signal::derive_local(move || texture_id.get().flatten())
}
