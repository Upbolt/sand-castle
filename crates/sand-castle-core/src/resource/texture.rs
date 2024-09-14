use getset::Getters;

#[cfg(feature = "loader_textures")]
use js_sys::Object;

#[cfg(feature = "loader_textures")]
use wasm_bindgen::{prelude::*, JsCast};

#[cfg(feature = "loader_textures")]
use web_sys::{
  window, CanvasRenderingContext2d, Element, Event, HtmlCanvasElement, HtmlImageElement, Url,
};

use derive_more::{Deref, From};

#[cfg(all(feature = "loader_textures", not(feature = "to_url")))]
use image::{ImageError, ImageFormat};

use uuid::Uuid;

#[cfg(feature = "to_url")]
use super::loader::{bytes_to_url, ToUrlError};

#[derive(Debug, Deref, From, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TextureId(u128);

impl TextureId {
  pub fn new() -> Self {
    Self(Uuid::new_v4().as_u128())
  }
}

impl Default for TextureId {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Getters)]
#[getset(get = "pub")]
pub struct Texture {
  pub(crate) id: TextureId,
  pub(crate) dimensions: (u32, u32),
  pub(crate) content: Vec<u8>,
}

#[cfg(feature = "loader_textures")]
#[derive(Debug, Clone)]
pub enum FromUrlError {
  NoDocument,
  NoContext,
  WebSys(JsValue),
  CreateCanvas(Element),
  GetContext(Object),
  ChannelRecv(async_channel::RecvError),
}

#[derive(Debug)]
pub enum LoadTextureError {
  #[cfg(feature = "loader_textures")]
  FromUrl(FromUrlError),

  #[cfg(feature = "to_url")]
  MemoryToUrl(ToUrlError),

  #[cfg(all(feature = "loader_textures", not(feature = "to_url")))]
  InvalidMimeType,
  #[cfg(all(feature = "loader_textures", not(feature = "to_url")))]
  Image(ImageError),
}

impl Texture {
  #[cfg(feature = "loader_textures")]
  pub async fn from_url(url: &str) -> Result<Self, FromUrlError> {
    let (send_rgba, recv_rgba) = async_channel::bounded(1);

    let document = window()
      .and_then(|window| window.document())
      .ok_or(FromUrlError::NoDocument)?;

    let canvas = document
      .create_element("canvas")
      .map_err(FromUrlError::WebSys)?
      .dyn_into::<HtmlCanvasElement>()
      .map_err(FromUrlError::CreateCanvas)?;

    let context = canvas
      .get_context("2d")
      .map_err(FromUrlError::WebSys)?
      .ok_or(FromUrlError::NoContext)?
      .dyn_into::<CanvasRenderingContext2d>()
      .map_err(FromUrlError::GetContext)?;

    let img = HtmlImageElement::new().map_err(FromUrlError::WebSys)?;

    let load_canv = canvas.clone();
    let load_ctx = context.clone();
    let load_url = url.to_string();
    let onload = Closure::<dyn FnMut(_)>::new(move |ev: Event| {
      let send_rgba = send_rgba.clone();

      let Some(target) = ev.target() else {
        send_rgba.close();
        return;
      };

      let Some(img) = target.dyn_ref::<HtmlImageElement>() else {
        send_rgba.close();
        return;
      };

      load_canv.set_width(img.width());
      load_canv.set_height(img.height());

      _ = load_ctx.draw_image_with_html_image_element(img, 0.0, 0.0);

      _ = Url::revoke_object_url(&load_url);

      let Ok(data) = load_ctx.get_image_data(0.0, 0.0, img.width().into(), img.height().into())
      else {
        send_rgba.close();
        return;
      };

      let dimensions = (img.width(), img.height());
      let send_rgba = send_rgba.clone();
      wasm_bindgen_futures::spawn_local(async move {
        _ = send_rgba
          .send(Self {
            id: Default::default(),
            dimensions,
            content: data.data().0,
          })
          .await;
      });
    });
    img.set_onload(Some(onload.as_ref().unchecked_ref()));
    onload.forget();

    img.set_src(url);

    recv_rgba.recv().await.map_err(FromUrlError::ChannelRecv)
  }

  #[cfg(feature = "to_url")]
  pub async fn from_bytes(bytes: &[u8], mime_type: &str) -> Result<Texture, LoadTextureError> {
    let url = bytes_to_url(bytes, mime_type).map_err(LoadTextureError::MemoryToUrl)?;

    Self::from_url(&url)
      .await
      .map_err(LoadTextureError::FromUrl)
  }

  #[cfg(all(feature = "loader_textures", not(feature = "to_url")))]
  pub async fn from_bytes(bytes: &[u8], mime_type: &str) -> Result<Texture, LoadTextureError> {
    let image = image::load_from_memory_with_format(
      bytes,
      ImageFormat::from_mime_type(mime_type).ok_or(LoadTextureError::InvalidMimeType)?,
    )
    .map_err(LoadTextureError::Image)?;

    Ok(Texture {
      id: TextureId::new(),
      dimensions: (image.width(), image.height()),
      content: image.to_rgba8().into_vec(),
    })
  }
}
