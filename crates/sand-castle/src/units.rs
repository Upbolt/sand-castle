use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[repr(align(4))]
#[derive(Default, Clone, Copy, Pod, Zeroable)]
pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
  pub position: [f32; 3],
  pub normal: [f32; 3],
}

#[repr(C)]
#[repr(align(4))]
#[derive(Default, Clone, Copy, Pod, Zeroable)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
  pub position: Vector3,
  pub color: Color,
}

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable)]
pub struct Box3 {
  pub min: Vector3,
  pub max: Vector3,
}
