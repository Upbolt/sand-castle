use derive_getters::Getters;

use crate::renderer::Render;

#[derive(Default, Getters)]
pub struct Scene {
  renderables: Vec<Box<dyn Render>>,
}

impl Scene {
  pub fn new() -> Scene {
    Self::default()
  }

  pub fn push(&mut self, renderable: impl Render + 'static) {
    self.renderables.push(Box::new(renderable));
  }
}
