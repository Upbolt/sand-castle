use leptos::prelude::*;

use sand_castle_core::{
  resource::{lighting::light::point_light::PointLight as CorePointLight, Resource},
  Vec3,
};

use crate::scene::SceneContextValue;

#[component]
pub fn PointLight(
  #[prop(default = Vec3::new(0.0, 0.0, 0.0).into(), into)] position: MaybeSignal<Vec3>,
  #[prop(default = Vec3::new(1.0, 1.0, 1.0).into(), into)] color: MaybeSignal<Vec3>,
) -> impl IntoView {
  let point_light = RwSignal::<Option<CorePointLight>>::new(None);

  let SceneContextValue {
    point_lights,
    scene,
    renderer,
    ..
  } = use_context().expect("`PointLight` must be used in a `Scene` component");

  let index_in_storage = Signal::derive(move || {
    point_lights.with(|lights| {
      lights.iter().position(|light| {
        light.with_untracked(|light| light.as_ref().map(|light| light.id()))
          == point_light.with_untracked(|light| light.as_ref().map(|light| light.id()))
      })
    })
  });

  Effect::new(move |_| {
    let light = CorePointLight::builder()
      .position(position.get_untracked())
      .color(color.get_untracked())
      .build();

    point_lights.update(|point_lights| {
      point_lights.push(point_light);
    });

    point_light.set(Some(light));
  });

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    let position = position.get();

    let Some(index) = index_in_storage.get() else {
      return;
    };

    point_light.update_untracked(|light| {
      let Some(light) = light else {
        return;
      };

      light.set_position(position);

      scene.update_untracked(|scene| {
        if let Some(scene) = scene {
          scene.update_point_light(&renderer, index, light);
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

    point_light.update_untracked(|light| {
      let Some(light) = light else {
        return;
      };

      light.set_color(color);

      scene.update_untracked(|scene| {
        if let Some(scene) = scene {
          scene.update_point_light(&renderer, index, light);
        }
      });
    });
  });

  on_cleanup(move || {
    point_lights.update(|point_lights| {
      if let Some(index_in_storage) = index_in_storage.get() {
        point_lights.remove(index_in_storage);
      };
    });
  });

  ()
}
