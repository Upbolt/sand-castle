use std::{any::TypeId, borrow::Cow};

use derive_builder::Builder;
use derive_getters::Getters;
use wgpu::{BindGroupLayoutDescriptor, ShaderModuleDescriptor, ShaderSource};

use super::{Material, ToMaterial};

#[derive(Getters, Builder)]
#[builder(pattern = "owned", build_fn(private, name = "infallible_build"))]
pub struct ShaderMaterial {
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
      shader_type: TypeId::of::<Self>(),
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
