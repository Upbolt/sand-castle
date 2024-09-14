use leptos::prelude::*;
use sand_castle_core::{
  resource::lighting::light::ambient_light::AmbientLight as CoreAmbientLight, Vec3,
};

use crate::scene::SceneContextValue;

#[component]
pub fn AmbientLight(
  #[prop(default = Vec3::default().into(), into)] color: MaybeSignal<Vec3>,
) -> impl IntoView {
  let ambient_light = RwSignal::<Option<CoreAmbientLight>>::new(None);

  let SceneContextValue {
    scene, renderer, ..
  } = use_context().expect("`AmbientLight` must be used in a `Scene` component");

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    let light = CoreAmbientLight::builder()
      .color(color.get_untracked())
      .build();

    scene.update(|scene| {
      if let Some(scene) = scene {
        scene.update_ambient_light(&renderer, &light);
      }
    });

    ambient_light.set(Some(light));
  });

  Effect::new(move |_| {
    let color = color.get();

    let Some(renderer) = renderer.get() else {
      return;
    };

    if ambient_light.with(|light| light.is_none()) {
      return;
    }

    ambient_light.update_untracked(move |light| {
      let Some(light) = light else {
        return;
      };

      light.set_color(color);

      scene.update_untracked(|scene| {
        if let Some(scene) = scene {
          scene.update_ambient_light(&renderer, light);
        }
      });
    });
  });

  on_cleanup(move || {
    let Some(renderer) = renderer.get_untracked() else {
      return;
    };

    scene.update(|scene| {
      if let Some(scene) = scene {
        scene.update_ambient_light(&renderer, &CoreAmbientLight::default());
      }
    });
  });

  ()
}
