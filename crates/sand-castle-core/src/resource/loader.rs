use super::geometry::Geometry;
use getset::Getters;
use glam::{Quat, Vec3};

#[cfg(feature = "to_url")]
use js_sys::{Array, Uint8Array};
#[cfg(feature = "to_url")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "to_url")]
use web_sys::{Blob, BlobPropertyBag, Url};

pub mod geometry;
pub mod material;

#[cfg(feature = "loader_gltf")]
pub mod gltf;

pub mod textures;

#[derive(Getters, Clone, Copy)]
#[getset(get = "pub")]
pub struct LoadedTransform {
  pub translation: Vec3,
  pub rotation: Quat,
  pub scale: Vec3,
}

#[derive(Getters, Clone)]
#[getset(get = "pub")]
pub struct Textures {
  pbr: Vec<u8>,
  normal: Vec<u8>,
  displacement: Vec<u8>,
}

pub trait LoadedGeometry {
  fn to_geometry(&self, blob: Option<&[u8]>) -> Vec<(LoadedTransform, Vec<Geometry>)>;
}

pub trait LoadedTextures {
  fn to_textures(&self, blob: Option<Vec<u8>>) -> Vec<Textures>;
}

#[cfg(feature = "to_url")]
#[derive(Debug, Clone)]
pub struct ToUrlError(JsValue);

#[cfg(feature = "to_url")]
pub fn bytes_to_url(bytes: &[u8], mime_type: &str) -> Result<String, ToUrlError> {
  let buffer = Uint8Array::from(bytes);

  let options = BlobPropertyBag::new();
  options.set_type(mime_type);

  let blob = Blob::new_with_u8_array_sequence_and_options(
    &Array::from_iter([&JsValue::from(buffer)]),
    &options,
  )
  .map_err(ToUrlError)?;

  Url::create_object_url_with_blob(&blob).map_err(ToUrlError)
}
