#[cfg(feature = "loader_gltf")]
pub use gltf::{buffer::Source as BufferSource, image::Source, import_buffers, Document, Gltf};

#[derive(Debug, Clone)]
pub struct LoadGltfError;

impl std::fmt::Display for LoadGltfError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "LoadGltfError")
  }
}

impl std::error::Error for LoadGltfError {}

pub trait LoadGltf {
  #[cfg(feature = "loader_gltf")]
  async fn from_url(url: &str) -> Result<Gltf, LoadGltfError>;
}

impl LoadGltf for Gltf {
  #[cfg(feature = "loader_gltf")]
  async fn from_url(url: &str) -> Result<Gltf, LoadGltfError> {
    let binary = reqwasm::http::Request::get(url)
      .send()
      .await
      .map_err(|_| LoadGltfError)?
      .binary()
      .await
      .map_err(|_| LoadGltfError)?;

    Self::from_slice(&binary).map_err(|_| LoadGltfError)
  }
}
