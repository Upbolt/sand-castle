use derive_builder::Builder;
use getset::Getters;
use web_sys::HtmlCanvasElement;
use wgpu::{
  Adapter, Backends, CreateSurfaceError, Device, DeviceDescriptor, Features, Instance,
  InstanceDescriptor, Limits, MemoryHints, PowerPreference, Queue, RequestAdapterOptions,
  RequestDeviceError, Surface, SurfaceCapabilities, SurfaceConfiguration, SurfaceTarget,
  TextureFormat, TextureUsages,
};

#[derive(Default, Clone, PartialEq)]
pub enum Backend {
  #[default]
  WebGL,
  WebGPU,
}

#[derive(Builder, Getters)]
#[getset(get = "pub")]
#[builder(pattern = "owned", build_fn(skip))]
pub struct Renderer {
  backend: Backend,
  canvas: HtmlCanvasElement,

  #[builder(setter(skip))]
  surface: Surface<'static>,

  #[builder(setter(skip))]
  adapter: Adapter,

  #[builder(setter(skip))]
  device: Device,

  #[builder(setter(skip))]
  queue: Queue,

  #[builder(setter(skip))]
  surface_capabilities: SurfaceCapabilities,

  #[builder(setter(skip))]
  supported_format: Option<TextureFormat>,
}

#[derive(Debug)]
pub enum RendererBuildError {
  NoAdapter,
  Incomplete(&'static str),
  CreateSurfaceError(CreateSurfaceError),
  RequestDeviceError(RequestDeviceError),
}

impl RendererBuilder {
  pub async fn build(self) -> Result<Renderer, RendererBuildError> {
    let canvas = self.canvas.ok_or(RendererBuildError::Incomplete(
      "`canvas` is a required field on the `Renderer` builder",
    ))?;

    let backend = self.backend.unwrap_or_default();

    let instance = Instance::new(InstanceDescriptor {
      backends: if backend == Backend::WebGL {
        Backends::GL
      } else {
        Backends::BROWSER_WEBGPU
      },
      ..Default::default()
    });

    let surface = instance
      .create_surface(SurfaceTarget::Canvas(canvas.clone()))
      .map_err(|err| RendererBuildError::CreateSurfaceError(err))?;

    let adapter = instance
      .request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      })
      .await
      .ok_or(RendererBuildError::NoAdapter)?;

    let mut required_limits = if backend == Backend::WebGL {
      Limits::downlevel_webgl2_defaults()
    } else {
      Limits::downlevel_defaults()
    };
    required_limits.max_bind_groups = 8;
    // required_limits.max_storage_buffers_per_shader_stage = 1;

    let (device, queue) = adapter
      .request_device(
        &DeviceDescriptor {
          required_features: Features::empty(),
          required_limits,
          memory_hints: MemoryHints::default(),
          label: None,
        },
        None,
      )
      .await
      .map_err(|err| RendererBuildError::RequestDeviceError(err))?;

    let capabilities = surface.get_capabilities(&adapter);
    let supported_format = capabilities
      .formats
      .iter()
      .find(|format| format.is_srgb())
      .or_else(|| capabilities.formats.first())
      .copied();

    if let (Some(present_mode), Some(alpha_mode), Some(format)) = (
      capabilities.present_modes.first().cloned(),
      capabilities.alpha_modes.first().cloned(),
      supported_format.clone(),
    ) {
      let width = canvas.client_width() as u32;
      let height = canvas.client_height() as u32;

      if width != 0 && height != 0 {
        surface.configure(
          &device,
          &SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            present_mode,
            alpha_mode,
            width,
            height,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
          },
        );
      }
    };

    Ok(Renderer {
      backend,
      canvas,
      surface,
      adapter,
      device,
      queue,
      surface_capabilities: capabilities,
      supported_format,
    })
  }
}

impl Renderer {
  pub fn builder() -> RendererBuilder {
    RendererBuilder::default()
  }

  pub fn resize(&self) {
    let (Some(present_mode), Some(alpha_mode), Some(format)) = (
      self.surface_capabilities().present_modes.first().cloned(),
      self.surface_capabilities().alpha_modes.first().cloned(),
      self.supported_format().clone(),
    ) else {
      return;
    };

    let width = self.canvas.client_width() as u32;
    let height = self.canvas.client_height() as u32;

    if width == 0 || height == 0 {
      return;
    }

    self.surface().configure(
      self.device(),
      &SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format,
        present_mode,
        alpha_mode,
        width,
        height,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
      },
    );
  }
}
