use std::borrow::Cow;

use derive_builder::Builder;
use getset::Getters;
use wgpu::{BindGroupLayoutDescriptor, ShaderModuleDescriptor, ShaderSource};

use crate::resource::{texture::TextureId, Id};

use super::{Material, ToMaterial};

#[derive(Getters, Builder, Debug, Clone)]
#[getset(get = "pub")]
#[builder(pattern = "owned", build_fn(private, name = "infallible_build"))]
pub struct ShaderMaterial {
  #[getset(set = "pub")]
  #[builder(default)]
  diffuse_map_texture_id: Option<TextureId>,

  #[getset(set = "pub")]
  #[builder(default)]
  normal_map_texture_id: Option<TextureId>,

  #[builder(setter(into))]
  vertex_shader: Cow<'static, str>,
  #[builder(setter(into))]
  fragment_shader: Cow<'static, str>,
}

impl ShaderMaterialBuilder {
  pub fn build(self) -> ShaderMaterial {
    self
      .infallible_build()
      .expect("could not build `ShaderMaterial`")
  }
}

impl ShaderMaterial {
  pub fn builder() -> ShaderMaterialBuilder {
    Default::default()
  }
}

impl ToMaterial for ShaderMaterial {
  fn to_material(&self) -> Material {
    Material {
      id: Id::new(),
      diffuse_map_texture_id: None,
      normal_map_texture_id: None,
      vertex_shader: ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(self.vertex_shader.clone()),
      },
      fragment_shader: ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(self.fragment_shader.clone()),
      },
      fragment_data_layout: BindGroupLayoutDescriptor {
        label: Some("ShaderMaterial_BindGroupLayoutDescriptor"),
        entries: &[],
      },
      fragment_data: vec![],
    }
  }
}
