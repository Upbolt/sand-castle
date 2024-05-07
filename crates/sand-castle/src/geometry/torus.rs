use derive_builder::Builder;

#[derive(Builder)]
pub struct Torus {
  radius: f32,
  tube: f32,
  radial_segmentse: u32,
  arc: u32,
}

impl Torus {
  pub fn builder() -> TorusBuilder {
    TorusBuilder::default()
  }
}
