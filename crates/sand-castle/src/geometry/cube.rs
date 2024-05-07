use derive_builder::Builder;

#[derive(Builder)]
pub struct Cube {
  width: f64,
  height: f64,
  depth: f64,
}

impl Cube {
  pub fn builder() -> CubeBuilder {
    CubeBuilder::default()
  }
}
