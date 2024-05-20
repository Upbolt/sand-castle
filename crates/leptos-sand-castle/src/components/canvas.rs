use std::{cell::RefCell, rc::Rc};

use leptos::*;
use leptos_use::{use_raf_fn, use_resize_observer, utils::Pausable};
use sand_castle::{
  camera::Camera,
  renderer::{Driver, Renderer},
  scene::Scene,
};
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement};

#[derive(Clone)]
pub struct CanvasContextValue {
  pub(crate) renderer: Signal<Option<Rc<RefCell<Renderer>>>>,
  pub(crate) scene: Signal<Rc<Scene>>,
}

async fn create_renderer(
  ctx: Option<(HtmlCanvasElement, Driver)>,
) -> Option<Rc<RefCell<Renderer>>> {
  let (node, driver) = ctx?;
  let canvas = node.dyn_ref::<HtmlCanvasElement>()?;
  let window = window()?;

  let new_renderer = Renderer::builder()
    .canvas(canvas.clone())
    .driver(driver)
    .pixel_ratio(window.device_pixel_ratio())
    .size((canvas.client_width(), canvas.client_height()))
    .build()
    .await
    .expect("could not build renderer");

  Some(Rc::new(RefCell::new(new_renderer)))
}

#[component]
pub fn Canvas(
  #[prop(optional)] driver: MaybeSignal<sand_castle::renderer::Driver>,
  #[prop(optional)] node_ref: NodeRef<html::Canvas>,
  children: ChildrenFn,
  #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
  let (scene, set_scene) = create_signal(Rc::new(Scene::new()));
  let (camera, set_camera) = create_signal::<Option<Rc<Camera>>>(None);
  let renderer_resource = create_local_resource(
    move || {
      let node = node_ref.get()?;
      let node: &HtmlCanvasElement = node.as_ref();

      Some((node.clone(), driver.get()))
    },
    create_renderer,
  );

  let renderer = Signal::derive(move || renderer_resource.get().flatten());

  Effect::new(move |_| {
    let Pausable { pause, .. } = use_raf_fn(move |_| {
      let Some(renderer) = renderer.get() else {
        return;
      };

      renderer.borrow_mut().render(
        scene.get().as_ref(),
        camera.get().as_ref().map(|camera| &**camera),
      );
    });

    on_cleanup(move || {
      pause();
    });
  });

  use_resize_observer(node_ref, move |entries, _| {
    let (Some(renderer), Some(entry)) = (renderer.get(), entries.get(0)) else {
      return;
    };

    let rect = entry.content_rect();

    renderer
      .borrow_mut()
      .resize((rect.width() as u32, rect.height() as u32));
  });

  provide_context(CanvasContextValue {
    renderer,
    scene: scene.into(),
  });

  let children = StoredValue::new(children);

  view! {
    <canvas
      {..attrs}
      node_ref=node_ref
    />

    <Show when=move || renderer.get().is_some()>
      {children.with_value(|children| children())}
    </Show>
  }
}
