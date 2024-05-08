#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Vector3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct Box3 {
  pub min: Vector3,
  pub max: Vector3,
}
