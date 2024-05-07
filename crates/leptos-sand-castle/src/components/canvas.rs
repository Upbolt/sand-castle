use std::rc::Rc;

use leptos::*;
use leptos_use::{use_raf_fn, use_resize_observer, utils::Pausable};
use sand_castle::{
  renderer::{Driver, Renderer},
  scene::Scene,
};
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement};

#[derive(Clone)]
pub struct CanvasContextValue {
  renderer: Signal<Option<Rc<Renderer>>>,
  scene: Signal<Rc<Scene>>,
}

async fn create_renderer(node_ref: Option<HtmlCanvasElement>) -> Option<Rc<Renderer>> {
  let node = node_ref?;
  let canvas = node.dyn_ref::<HtmlCanvasElement>()?;
  let window = window()?;

  let new_renderer = Renderer::builder()
    .canvas(canvas.clone())
    .driver(Driver::WebGL)
    .pixel_ratio(window.device_pixel_ratio())
    .size((canvas.client_width(), canvas.client_height()))
    .build()
    .await
    .expect("could not build renderer");

  Some(Rc::new(new_renderer))
}

#[component]
pub fn Canvas(
  #[prop(optional)] driver: MaybeSignal<sand_castle::renderer::Driver>,
  #[prop(optional)] node_ref: NodeRef<html::Canvas>,
  #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
  let (scene, set_scene) = create_signal(Rc::new(Scene::new()));
  let renderer_resource = create_local_resource(
    move || {
      let node = node_ref.get()?;
      let node: &HtmlCanvasElement = node.as_ref();

      Some(node.clone())
    },
    create_renderer,
  );
  let renderer = Signal::derive(move || renderer_resource.get().flatten());

  Effect::new(move |_| {
    let Pausable { pause, .. } = use_raf_fn(move |_| {
      let Some(renderer) = renderer.get() else {
        return;
      };

      renderer.render(scene.get().as_ref());
    });

    on_cleanup(move || {
      pause();
    });
  });

  use_resize_observer(node_ref, move |entries, _| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    let Some(entry) = entries.get(0) else {
      return;
    };

    let rect = entry.content_rect();

    renderer.resize((rect.width() as u32, rect.height() as u32));
  });

  provide_context(CanvasContextValue {
    renderer,
    scene: scene.into(),
  });

  view! {
    <canvas node_ref=node_ref />
    {children.map(|children| children())}
  }
}
