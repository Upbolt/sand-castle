use wgpu::ShaderSource;

pub mod mesh_basic;
pub mod shader;

pub trait WithMaterial
where
  Self: Sized,
{
  fn shader(&self) -> ShaderSource;

  fn into_material(self) -> Material {
    Material
  }
}

#[derive(Clone)]
pub struct Material;
