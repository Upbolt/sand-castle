use std::any::TypeId;

use derive_builder::Builder;
use getset::Getters;
use glam::Vec4;
use wgpu::{
  include_wgsl, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType,
  ShaderStages,
};

use super::{Material, ToMaterial};

#[derive(Getters, Builder, Debug, Clone)]
#[getset(get = "pub")]
#[builder(pattern = "owned", build_fn(private, name = "infallible_build"))]
pub struct PhongMaterial {
  #[builder(default)]
  color: Vec4,
}

impl PhongMaterialBuilder {
  pub fn build(self) -> PhongMaterial {
    self
      .infallible_build()
      .expect("could not build `PhongMaterial`")
  }
}

impl PhongMaterial {
  pub fn builder() -> PhongMaterialBuilder {
    Default::default()
  }

  pub fn with_color(color: Vec4) -> Self {
    Self { color }
  }
}

impl ToMaterial for PhongMaterial {
  fn to_material(&self) -> Material {
    Material {
      shader_type: TypeId::of::<Self>(),
      fragment_shader: include_wgsl!("shaders/phong/fs_phong.wgsl"),
      vertex_shader: include_wgsl!("shaders/phong/vs_phong.wgsl"),
      fragment_data_layout: BindGroupLayoutDescriptor {
        label: Some("PhongMaterial_BindGroupLayoutDescriptor"),
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
