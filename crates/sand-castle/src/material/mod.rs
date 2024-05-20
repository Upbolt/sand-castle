use wgpu::ShaderSource;

pub mod mesh_basic;
pub mod shader;

pub trait WithMaterial
where
  Self: Sized,
{
  fn into_material<'a>(self) -> Material<'a>;
}

#[derive(Clone)]
pub struct Material<'a> {
  pub shader: ShaderSource<'a>,
}
