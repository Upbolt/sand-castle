use std::sync::Arc;

use leptos::*;

use sand_castle_core::{renderer::Renderer, scene::Scene as CoreScene, Vec4};

use crate::canvas::RendererContextValue;

#[derive(Clone)]
pub struct SceneContextValue {
  pub renderer: Signal<Option<Arc<Renderer>>>,
  pub scene: RwSignal<Option<CoreScene>>,
}

#[component]
pub fn Scene(
  #[prop(optional, into)] color: MaybeProp<Vec4>,
  #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
  let scene = RwSignal::<Option<CoreScene>>::new(None);

  let RendererContextValue { renderer, scenes } =
    use_context().expect("`Scene` must be used in a `Canvas` component");

  provide_context(SceneContextValue { scene, renderer });

  Effect::new(move |_| {
    logging::log!(
      "{:?}",
      scene.with(|scene| scene.as_ref().map(|scene| scene.subject_count()))
    );
  });

  Effect::new(move |_| {
    scene.update(|scene| {
      let scene_builder = CoreScene::builder();

      let scene_builder = if let Some(color) = color.get_untracked() {
        scene_builder.color(color)
      } else {
        scene_builder
      };

      *scene = Some(scene_builder.build());
    });

    scenes.update(|scenes| scenes.push(scene));
  });

  Effect::new(move |_| {
    let Some(color) = color.get() else {
      return;
    };

    scene.update(|scene| {
      if let Some(scene) = scene {
        scene.set_color(color);
      }
    });
  });

  on_cleanup(move || {
    scene.with(|current_scene| {
      let Some(current_scene) = current_scene else {
        return;
      };

      scenes.update(|scenes| {
        if let Some(pos) = scenes.iter().position(|scene| {
          scene.with(|scene| {
            scene
              .as_ref()
              .map(|scene| scene == current_scene)
              .unwrap_or_default()
          })
        }) {
          scenes.remove(pos);
        };
      });
    });
  });

  children.map(|children| children().into_view())
}
