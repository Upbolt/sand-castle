use derive_builder::Builder;
use getset::Getters;
use glam::Vec4;
use wgpu::{
  include_wgsl, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType,
  ShaderStages,
};

use crate::resource::{texture::TextureId, Id};

use super::{Material, ToMaterial};

#[derive(Getters, Builder, Debug, Default, Clone)]
#[getset(get = "pub")]
#[builder(pattern = "owned", build_fn(private, name = "infallible_build"))]
pub struct PbrMaterial {
  #[getset(set = "pub")]
  #[builder(default)]
  diffuse_map_texture_id: Option<TextureId>,

  #[getset(set = "pub")]
  #[builder(default)]
  normal_map_texture_id: Option<TextureId>,

  #[builder(default = "Vec4::new(1.0, 1.0, 1.0, 1.0)")]
  color: Vec4,

  #[builder(default = "0.0")]
  metalness: f32,

  #[builder(default = "1.0")]
  roughness: f32,
}

impl PbrMaterialBuilder {
  pub fn build(self) -> PbrMaterial {
    self
      .infallible_build()
      .expect("could not build `PbrMaterial`")
  }
}

impl PbrMaterial {
  pub fn builder() -> PbrMaterialBuilder {
    Default::default()
  }
}

impl ToMaterial for PbrMaterial {
  fn to_material(&self) -> Material {
    let fragment_shader = if self.diffuse_map_texture_id.is_some() {
      include_wgsl!("shaders/pbr/fs_pbr_tex.wgsl")
    } else {
      include_wgsl!("shaders/pbr/fs_pbr.wgsl")
    };

    Material {
      id: Id::new(),
      diffuse_map_texture_id: self.diffuse_map_texture_id.clone(),
      normal_map_texture_id: self.normal_map_texture_id.clone(),
      fragment_shader,
      vertex_shader: include_wgsl!("shaders/pbr/vs_pbr.wgsl"),
      fragment_data_layout: BindGroupLayoutDescriptor {
        label: Some("PbrMaterial_BindGroupLayoutDescriptor"),
        entries: &[BindGroupLayoutEntry {
          binding: 0,
          visibility: ShaderStages::FRAGMENT,
          ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
          },
          count: None,
        }],
      },
      fragment_data: Vec::from(bytemuck::cast_slice(&[
        self.color,
        Vec4::new(self.roughness, self.metalness, 0.0, 0.0),
      ])),
    }
  }
}
