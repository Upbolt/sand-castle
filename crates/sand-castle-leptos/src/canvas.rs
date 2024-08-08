use std::sync::Arc;

use leptos::*;
use leptos_use::{use_raf_fn, use_resize_observer, utils::Pausable};
use sand_castle_core::{renderer::Renderer, scene::Scene as CoreScene};

pub use sand_castle_core::renderer::Backend;

use std::ops::Deref;

#[derive(Clone)]
pub struct RendererContextValue {
  pub renderer: Signal<Option<Arc<Renderer>>>,
  pub scenes: RwSignal<Vec<RwSignal<Option<CoreScene>>>>,
}

#[component]
pub fn Canvas(
  #[prop(optional, into)] backend: MaybeProp<Backend>,

  #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
  #[prop(optional)] node_ref: NodeRef<html::Canvas>,
  #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
  let scenes = RwSignal::<Vec<RwSignal<Option<CoreScene>>>>::new(vec![]);

  let renderer_rsc = create_local_resource(
    || {},
    move |_| {
      let backend = backend.get();
      let canvas = node_ref.get();

      async move {
        let canvas = canvas?;
        let renderer = Renderer::builder();

        let renderer = if let Some(backend) = backend {
          renderer.backend(backend)
        } else {
          renderer
        };

        Some(Arc::new(
          renderer
            .canvas(canvas.deref().clone())
            .build()
            .await
            .expect("Failed to build `Renderer`"),
        ))
      }
    },
  );

  let renderer = Signal::derive(move || renderer_rsc.get().flatten());

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    let Pausable { pause, .. } = use_raf_fn(move |_| {
      scenes.with(|scenes| {
        for scene in scenes {
          scene.with(|scene| {
            if let Some(scene) = scene {
              scene.render(&renderer);
            }
          });
        }
      });
    });

    on_cleanup(move || {
      pause();
    });
  });

  use_resize_observer(node_ref, move |_, _| {
    renderer.with(|renderer| {
      if let Some(renderer) = renderer {
        renderer.resize();
      }
    });
  });

  provide_context(RendererContextValue { renderer, scenes });

  view! {
    <canvas {..attrs} node_ref=node_ref>
      {children.map(|children| children())}
    </canvas>
  }
}
