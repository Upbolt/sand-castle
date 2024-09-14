use std::sync::Arc;

use indexmap::IndexMap;
use leptos::prelude::*;

use derive_builder::Builder;

pub use sand_castle_core::resource::loader::{gltf::LoadGltfError, LoadedTransform};

use sand_castle_core::{
  resource::{
    geometry::{Geometry, Vertex},
    loader::{
      geometry::GeometryLoader,
      gltf::{import_buffers, Source},
      material::MaterialLoader,
      textures::TextureLoader,
    },
    texture::{Texture, TextureId},
    Id,
  },
  Quat, Vec2, Vec3, Vec4,
};

pub use sand_castle_core::resource::loader::{
  gltf::{Gltf, LoadGltf},
  LoadedGeometry,
};

use crate::scene::SceneContextValue;

#[derive(Clone)]
pub enum GltfSource {
  Url(MaybeProp<String>),
  Memory(MaybeProp<Vec<u8>>),
}

impl From<MaybeProp<String>> for GltfSource {
  fn from(value: MaybeProp<String>) -> Self {
    Self::Url(value)
  }
}

impl From<MaybeProp<Vec<u8>>> for GltfSource {
  fn from(value: MaybeProp<Vec<u8>>) -> Self {
    Self::Memory(value)
  }
}

impl From<String> for GltfSource {
  fn from(value: String) -> Self {
    Self::Url(MaybeProp::from(value))
  }
}

impl From<Vec<u8>> for GltfSource {
  fn from(value: Vec<u8>) -> Self {
    Self::Memory(MaybeProp::from(value))
  }
}

pub fn use_gltf_loader(
  url: impl Into<MaybeProp<String>>,
) -> Signal<
  Option<Result<Vec<(LoadedTransform, Id, Vec4, Option<TextureId>)>, LoadGltfError>>,
  LocalStorage,
> {
  let url: MaybeProp<String> = url.into();

  let SceneContextValue {
    texture_loader,
    geometry_loader,
    ..
  } = use_context().expect("`use_gltf_loader` must be used in a component inside of a `Scene`");

  let model = AsyncDerived::new_unsync(move || {
    let url = url.clone();
    async move {
      if geometry_loader.with(|loader| loader.is_none())
        || texture_loader.with(|loader| loader.is_none())
      {
        return None;
      }

      let model = match Gltf::from_url(&url.get()?).await {
        Ok(model) => model,
        Err(err) => {
          return Some(Err(LoadGltfError));
        }
      };

      let model = match load_gltf(model, geometry_loader, texture_loader).await {
        Ok(model) => model,
        Err(err) => {
          return Some(Err(err));
        }
      };

      Some(Ok(model))
    }
  });

  Signal::derive_local(move || model.get().flatten())
}

pub fn use_gltf_loader_from_source(
  source: impl Into<GltfSource>,
) -> Signal<Option<Result<Vec<(LoadedTransform, Id, Vec4, Option<TextureId>)>, LoadGltfError>>> {
  let source: GltfSource = source.into();

  let SceneContextValue {
    texture_loader,
    geometry_loader,
    ..
  } = use_context().expect("`use_gltf_loader` must be used in a component inside of a `Scene`");

  let model = AsyncDerived::new_unsync(move || {
    let source = source.clone();

    async move {
      if geometry_loader.with(|loader| loader.is_none())
        || texture_loader.with(|loader| loader.is_none())
      {
        return None;
      }

      let model = match source {
        GltfSource::Url(url) => match Gltf::from_url(&url.get()?).await {
          Ok(model) => model,
          Err(err) => {
            return Some(Err(LoadGltfError));
          }
        },
        GltfSource::Memory(memory) => {
          match memory.with(|memory| memory.as_ref().map(|memory| Gltf::from_slice(&memory)))? {
            Ok(model) => model,
            Err(err) => {
              return Some(Err(LoadGltfError));
            }
          }
        }
      };

      let model = match load_gltf(model, geometry_loader, texture_loader).await {
        Ok(model) => model,
        Err(err) => {
          return Some(Err(LoadGltfError));
        }
      };

      Some(Ok(model))
    }
  });

  Signal::derive(move || model.get().flatten())
}

async fn load_gltf(
  model: Gltf,
  geometry_loader: RwSignal<Option<GeometryLoader>, LocalStorage>,
  texture_loader: RwSignal<Option<TextureLoader>, LocalStorage>,
) -> Result<Vec<(LoadedTransform, Id, Vec4, Option<TextureId>)>, LoadGltfError> {
  let (document, blob) = (model.document, model.blob);

  let buffers = match import_buffers(&document, None, blob) {
    Ok(buffers) => buffers,
    Err(err) => {
      return Err(LoadGltfError);
    }
  };

  let model = document
    .nodes()
    .filter_map(|node| {
      let (translation, rotation, scale) = node.transform().decomposed();

      let transform = LoadedTransform {
        translation: Vec3::from_array(translation),
        rotation: Quat::from_array(rotation),
        scale: Vec3::from_array(scale),
      };

      node.mesh().map(|mesh| {
        let transform = transform.clone();
        let buffers = buffers.as_slice();

        mesh.primitives().map(move |primitive| {
          let color = Vec4::from_array(
            primitive
              .material()
              .pbr_metallic_roughness()
              .base_color_factor(),
          );

          let vertices = primitive.reader(|buffer| {
            buffers
              .get(buffer.index())
              .map(|buffer| buffer.0.as_slice())
          });
          let mut positions = vertices.read_positions().into_iter().flatten();
          let mut normals = vertices.read_normals().into_iter().flatten();
          let mut tex_coords = vertices
            .read_tex_coords(0)
            .map(|iter| iter.into_f32())
            .into_iter()
            .flatten();

          let geometry = Geometry::new(
            std::iter::from_fn(move || {
              match (positions.next(), normals.next(), tex_coords.next()) {
                (None, None, None) => None,
                (position, normal, tex_coords) => Some(Vertex::new(
                  Vec3::from_array(position.unwrap_or_default()),
                  Vec3::from_array(normal.unwrap_or_default()),
                  Vec2::from_array(tex_coords.unwrap_or_default()),
                )),
              }
            })
            .collect::<Vec<_>>(),
            vertices
              .read_indices()
              .map(|indices| indices.into_u32().collect::<Vec<_>>())
              .unwrap_or_default(),
          );

          let geometry_id = *geometry.id();

          geometry_loader.update_untracked(|loader| {
            if let Some(loader) = loader {
              loader.insert(geometry);
            }
          });

          let diffuse_map = primitive
            .material()
            .pbr_metallic_roughness()
            .base_color_texture();

          (geometry_id, transform, color, diffuse_map)
        })
      })
    })
    .flatten()
    .collect::<Vec<_>>();

  let textures = model
    .iter()
    .enumerate()
    .map(|(index, (_, _, _, diffuse_map))| async {
      if let Some(diffuse_map) = diffuse_map.as_ref() {
        match diffuse_map.texture().source().source() {
          Source::View { view, mime_type } => {
            if let Some(data) = buffers.get(view.buffer().index()) {
              Texture::from_bytes(
                &data[view.offset()..view.offset() + view.length()],
                mime_type,
              )
              .await
              .ok()
              .map(|texture| {
                let id = *texture.id();

                texture_loader.update_untracked(|loader| {
                  if let Some(loader) = loader {
                    loader.insert(texture);
                  }
                });

                id
              })
            } else {
              None
            }
          }
          _ => None,
        }
      } else {
        None
      }
    })
    .collect::<Vec<_>>();

  let textures = futures::future::join_all(textures).await;

  let model = model
    .into_iter()
    .zip(textures.into_iter())
    .map(|((geometry_id, transform, color, _), texture_id)| {
      (transform, geometry_id, color, texture_id)
    })
    .collect::<Vec<_>>();

  Ok(model)
}
