pub mod orthographic;
pub mod perspective;

pub trait Camera {
  fn view();
}

#[derive(Default, Clone)]
pub struct ViewFrustum {
  pub near: f32,
  pub far: f32,
}
