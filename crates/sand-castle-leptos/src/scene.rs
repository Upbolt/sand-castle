use std::sync::Arc;

use leptos::*;

use sand_castle_core::{
  renderer::Renderer,
  resource::lighting::light::{
    directional_light::DirectionalLight as CoreDirectionalLight,
    point_light::PointLight as CorePointLight, spot_light::SpotLight as CoreSpotLight,
  },
  scene::Scene as CoreScene,
  Vec4,
};

use crate::canvas::RendererContextValue;

#[derive(Clone)]
pub struct SceneContextValue {
  pub renderer: Signal<Option<Arc<Renderer>>>,
  pub scene: RwSignal<Option<CoreScene>>,
  pub directional_lights: RwSignal<Vec<RwSignal<Option<CoreDirectionalLight>>>>,
  pub spot_lights: RwSignal<Vec<RwSignal<Option<CoreSpotLight>>>>,
  pub point_lights: RwSignal<Vec<RwSignal<Option<CorePointLight>>>>,
}

#[component]
pub fn Scene(
  #[prop(optional, into)] color: MaybeProp<Vec4>,
  #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
  let scene = RwSignal::<Option<CoreScene>>::new(None);

  let directional_lights = RwSignal::<Vec<RwSignal<Option<CoreDirectionalLight>>>>::new(vec![]);
  let point_lights = RwSignal::<Vec<RwSignal<Option<CorePointLight>>>>::new(vec![]);
  let spot_lights = RwSignal::<Vec<RwSignal<Option<CoreSpotLight>>>>::new(vec![]);

  let RendererContextValue { renderer, scenes } =
    use_context().expect("`Scene` must be used in a `Canvas` component");

  provide_context(SceneContextValue {
    scene,
    renderer,
    directional_lights,
    point_lights,
    spot_lights,
  });

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    scene.update(|scene| {
      let scene_builder = CoreScene::builder()
        .init_camera(&renderer)
        .init_ambient_light(&renderer)
        .init_dynamic_lights(&renderer);

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

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    point_lights.with(|lights| {
      let lights = lights
        .iter()
        .filter_map(|light| light.get())
        .collect::<Vec<_>>();

      scene.update(|scene| {
        if let Some(scene) = scene {
          scene.bind_point_lights(&renderer, &lights);
        }
      });
    });
  });

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    spot_lights.with(|lights| {
      let lights = lights
        .iter()
        .filter_map(|light| light.get())
        .collect::<Vec<_>>();

      scene.update(|scene| {
        if let Some(scene) = scene {
          scene.bind_spot_lights(&renderer, &lights);
        }
      });
    });
  });

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    directional_lights.with(|lights| {
      let lights = lights
        .iter()
        .filter_map(|light| light.get())
        .collect::<Vec<_>>();

      scene.update(|scene| {
        if let Some(scene) = scene {
          scene.bind_directional_lights(&renderer, &lights);
        }
      });
    });
  });

  on_cleanup(move || {
    scene.with_untracked(|current_scene| {
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
