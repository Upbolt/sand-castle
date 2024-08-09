use std::any::TypeId;

use derive_builder::Builder;
use derive_getters::Getters;
use glam::Vec4;
use wgpu::{
  include_wgsl, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType,
  ShaderStages,
};

use super::{Material, ToMaterial};

#[derive(Getters, Builder)]
#[builder(pattern = "owned", build_fn(private, name = "infallible_build"))]
pub struct BasicMaterial {
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
    Self { color }
  }
}

impl ToMaterial for BasicMaterial {
  fn to_material(&self) -> Material {
    Material {
      shader_type: TypeId::of::<Self>(),
      fragment_shader: include_wgsl!("shaders/basic/fs_basic.wgsl"),
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
