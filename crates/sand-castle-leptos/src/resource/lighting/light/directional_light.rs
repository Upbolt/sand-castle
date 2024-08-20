use leptos::*;

use sand_castle_core::{
  resource::{
    lighting::light::directional_light::DirectionalLight as CoreDirectionalLight, Resource,
  },
  Vec3,
};

use crate::scene::SceneContextValue;

#[component]
pub fn DirectionalLight(
  #[prop(default = Vec3::new(1.0, 1.0, 1.0).into(), into)] color: MaybeSignal<Vec3>,
  #[prop(optional, into)] direction: MaybeSignal<Vec3>,
) -> impl IntoView {
  let directional_light = RwSignal::<Option<CoreDirectionalLight>>::new(None);

  let SceneContextValue {
    directional_lights,
    scene,
    renderer,
    ..
  } = use_context().expect("`DirectionalLight` must be used in a `Scene` component");

  let index_in_storage = Signal::derive(move || {
    directional_lights.with(|lights| {
      lights.iter().position(|light| {
        light.with_untracked(|light| light.as_ref().map(|light| light.id()))
          == directional_light.with_untracked(|light| light.as_ref().map(|light| light.id()))
      })
    })
  });

  Effect::new(move |_| {
    let light = CoreDirectionalLight::builder()
      .direction(direction.get_untracked())
      .color(color.get_untracked())
      .build();

    directional_lights.update(|directional_lights| {
      directional_lights.push(directional_light);
    });

    directional_light.set(Some(light));
  });

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    let direction = direction.get();

    let Some(index) = index_in_storage.get() else {
      return;
    };

    directional_light.update_untracked(|light| {
      let Some(light) = light else {
        return;
      };

      light.set_direction(direction);

      scene.update_untracked(|scene| {
        if let Some(scene) = scene {
          scene.update_directional_light(&renderer, index, light);
        }
      });
    });
  });

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    let color = color.get();

    let Some(index) = index_in_storage.get() else {
      return;
    };

    directional_light.update_untracked(|light| {
      let Some(light) = light else {
        return;
      };

      light.set_color(color);

      scene.update_untracked(|scene| {
        if let Some(scene) = scene {
          scene.update_directional_light(&renderer, index, light);
        }
      });
    });
  });

  on_cleanup(move || {
    directional_lights.update(|directional_lights| {
      if let Some(index_in_storage) = index_in_storage.get() {
        directional_lights.remove(index_in_storage);
      };
    });
  });

  ()
}
