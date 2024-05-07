use derive_builder::Builder;
use web_sys::HtmlCanvasElement;
use wgpu::{CreateSurfaceError, InstanceDescriptor, SurfaceTarget};

use crate::scene::Scene;

#[derive(Clone, Copy)]
pub enum Driver {
  WebGL,
  WebGPU,
}

#[derive(Builder)]
#[builder(build_fn(skip))]
pub struct Renderer {
  driver: Driver,
  canvas: HtmlCanvasElement,
  #[builder(setter(skip))]
  surface: wgpu::Surface<'static>,
  pixel_ratio: f64,
  size: (f64, f64),
}

#[derive(Debug)]
pub enum RendererInitializationError {
  Incomplete(&'static str),
  SurfaceCreation(CreateSurfaceError),
}

impl RendererBuilder {
  pub fn build(&self) -> Result<Renderer, RendererInitializationError> {
    let Some(driver) = &self.driver else {
      return Err(RendererInitializationError::Incomplete(
        "field `driver` not provided",
      ));
    };

    let Some(size) = &self.size else {
      return Err(RendererInitializationError::Incomplete(
        "field `size` not provided",
      ));
    };

    let Some(pixel_ratio) = &self.pixel_ratio else {
      return Err(RendererInitializationError::Incomplete(
        "field `pixel_ratio` not provided",
      ));
    };

    let Some(canvas) = &self.canvas else {
      return Err(RendererInitializationError::Incomplete(
        "field `canvas` not provided",
      ));
    };

    let instance = wgpu::Instance::new(InstanceDescriptor::default());
    let surface = instance
      .create_surface(SurfaceTarget::Canvas(canvas.clone()))
      .map_err(|err| RendererInitializationError::SurfaceCreation(err))?;

    Ok(Renderer {
      driver: *driver,
      size: *size,
      pixel_ratio: *pixel_ratio,
      canvas: canvas.clone(),
      surface,
    })
  }
}

impl Renderer {
  pub fn builder() -> RendererBuilder {
    RendererBuilder::default()
  }

  pub fn render(&self, scene: &Scene) {}
}

pub trait Render {
  fn id(&self) -> u32;
  fn render(&self, renderer: &Renderer);
}
