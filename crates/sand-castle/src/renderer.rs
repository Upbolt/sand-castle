use derive_builder::Builder;
use web_sys::HtmlCanvasElement;

#[derive(Clone)]
pub enum Driver {
  WebGL,
  WebGPU,
}

#[derive(Builder)]
pub struct Renderer {
  driver: Driver,
  canvas: HtmlCanvasElement,
  pixel_ratio: f64,
  size: (u32, u32),
}

impl Renderer {
  fn builder() -> RendererBuilder {
    RendererBuilder::default()
  }
}
