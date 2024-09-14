use std::sync::Arc;

use leptos::prelude::*;

use sand_castle_core::{
  renderer::Renderer,
  resource::{
    lighting::light::{
      directional_light::DirectionalLight as CoreDirectionalLight,
      point_light::PointLight as CorePointLight, spot_light::SpotLight as CoreSpotLight,
    },
    loader::{geometry::GeometryLoader, material::MaterialLoader, textures::TextureLoader},
  },
  scene::Scene as CoreScene,
  Vec4,
};

use crate::canvas::RendererContextValue;

#[derive(Clone)]
pub struct SceneContextValue {
  pub renderer: Signal<Option<Arc<Renderer>>, LocalStorage>,
  pub scene: RwSignal<Option<CoreScene>, LocalStorage>,
  pub texture_loader: RwSignal<Option<TextureLoader>, LocalStorage>,
  pub material_loader: RwSignal<Option<MaterialLoader>, LocalStorage>,
  pub geometry_loader: RwSignal<Option<GeometryLoader>, LocalStorage>,
  pub directional_lights: RwSignal<Vec<RwSignal<Option<CoreDirectionalLight>>>, LocalStorage>,
  pub spot_lights: RwSignal<Vec<RwSignal<Option<CoreSpotLight>>>, LocalStorage>,
  pub point_lights: RwSignal<Vec<RwSignal<Option<CorePointLight>>>, LocalStorage>,
}

#[component]
pub fn Scene(
  #[prop(optional, into)] color: MaybeProp<Vec4>,
  #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
  let scene = RwSignal::new_local(None);

  let texture_loader = RwSignal::new_local(None);
  let geometry_loader = RwSignal::new_local(None);
  let material_loader = RwSignal::new_local(None);

  let directional_lights = RwSignal::new_local(vec![]);
  let point_lights = RwSignal::new_local(vec![]);
  let spot_lights = RwSignal::new_local(vec![]);

  let RendererContextValue { renderer, scenes } =
    use_context().expect("`Scene` must be used in a `Canvas` component");

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    scene.update(|scene| {
      let scene_builder = CoreScene::builder(&renderer);

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
    texture_loader.set(Some(TextureLoader::new()));
    material_loader.set(Some(MaterialLoader::new()));
    geometry_loader.set(Some(GeometryLoader::new()));
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
        .filter_map(|light: &RwSignal<Option<CorePointLight>>| light.get())
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
        .filter_map(|light: &RwSignal<Option<CoreSpotLight>>| light.get())
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
        .filter_map(|light: &RwSignal<Option<CoreDirectionalLight>>| light.get())
        .collect::<Vec<_>>();

      scene.update(|scene| {
        if let Some(scene) = scene {
          scene.bind_directional_lights(&renderer, &lights);
        }
      });
    });
  });

  provide_context(SceneContextValue {
    scene,
    renderer,
    texture_loader,
    material_loader,
    geometry_loader,
    directional_lights,
    point_lights,
    spot_lights,
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
