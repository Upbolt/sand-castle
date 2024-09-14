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
pub struct BasicMaterial {
  #[getset(set = "pub")]
  #[builder(default)]
  diffuse_map_texture_id: Option<TextureId>,

  #[getset(set = "pub")]
  #[builder(default)]
  normal_map_texture_id: Option<TextureId>,

  #[builder(default)]
  color: Vec4,
}

impl BasicMaterialBuilder {
  pub fn build(self) -> BasicMaterial {
    self
      .infallible_build()
      .expect("could not build `BasicMaterial`")
  }
}

impl BasicMaterial {
  pub fn builder() -> BasicMaterialBuilder {
    Default::default()
  }

  pub fn with_color(color: Vec4) -> Self {
    Self {
      color,
      ..Default::default()
    }
  }
}

impl ToMaterial for BasicMaterial {
  fn to_material(&self) -> Material {
    let fragment_shader = if self.diffuse_map_texture_id.is_some() {
      include_wgsl!("shaders/basic/fs_basic_tex.wgsl")
    } else {
      include_wgsl!("shaders/basic/fs_basic.wgsl")
    };

    Material {
      id: Id::new(),
      diffuse_map_texture_id: self.diffuse_map_texture_id.clone(),
      normal_map_texture_id: self.normal_map_texture_id.clone(),
      fragment_shader,
      vertex_shader: include_wgsl!("shaders/basic/vs_basic.wgsl"),
      fragment_data_layout: BindGroupLayoutDescriptor {
        label: Some("BasicMaterial_BindGroupLayoutDescriptor"),
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
      fragment_data: Vec::from(bytemuck::cast_slice(&[self.color])),
    }
  }
}
