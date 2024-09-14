use std::sync::Arc;

use leptos::{html, prelude::*};
use leptos_use::{use_raf_fn, use_resize_observer, utils::Pausable};
use sand_castle_core::{renderer::Renderer, scene::Scene as CoreScene};

pub use sand_castle_core::renderer::Backend;

use std::ops::Deref;

#[derive(Clone)]
pub struct RendererContextValue {
  pub renderer: Signal<Option<Arc<Renderer>>, LocalStorage>,
  pub scenes: RwSignal<Vec<RwSignal<Option<CoreScene>, LocalStorage>>, LocalStorage>,
}

#[component]
pub fn Canvas(
  #[prop(optional, into)] backend: MaybeProp<Backend>,

  #[prop(optional)] node_ref: NodeRef<html::Canvas>,
  #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
  let scenes = RwSignal::new_local(vec![]);

  let renderer = AsyncDerived::new_unsync(move || {
    let backend = backend.clone();
    async move {
      let backend = backend.get();
      let canvas = node_ref.get();

      let canvas = canvas?;
      let renderer = Renderer::builder();

      let renderer = if let Some(backend) = backend {
        renderer.backend(backend)
      } else {
        renderer
      };

      Some(Arc::new(
        renderer
          .canvas(canvas.clone())
          .build()
          .await
          .expect("Failed to build `Renderer`"),
      ))
    }
  });

  let renderer = Signal::derive_local(move || renderer.get().flatten());

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    _ = use_raf_fn(move |_| {
      scenes.with(|scenes: &Vec<RwSignal<Option<CoreScene>, LocalStorage>>| {
        for scene in scenes {
          scene.with(|scene| {
            if let Some(scene) = scene {
              scene.render(&renderer);
            }
          });
        }
      });
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
    <canvas node_ref=node_ref>
      {children.map(|children| children())}
    </canvas>
  }
}
