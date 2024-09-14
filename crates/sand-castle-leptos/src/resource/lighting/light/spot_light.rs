use leptos::prelude::*;

use sand_castle_core::{
  resource::{lighting::light::spot_light::SpotLight as CoreSpotLight, Resource},
  Vec3,
};

use crate::scene::SceneContextValue;

#[component]
pub fn SpotLight(
  #[prop(default = Vec3::new(1.0, 1.0, 1.0).into(), into)] color: MaybeSignal<Vec3>,
  #[prop(optional, into)] position: MaybeSignal<Vec3>,
  #[prop(optional, into)] direction: MaybeSignal<Vec3>,
  #[prop(default = 45.0_f32.into(), into)] cutoff_angle: MaybeSignal<f32>,
) -> impl IntoView {
  let spot_light = RwSignal::<Option<CoreSpotLight>>::new(None);

  let SceneContextValue {
    spot_lights,
    scene,
    renderer,
    ..
  } = use_context().expect("`SpotLight` must be used in a `Scene` component");

  let index_in_storage = Signal::derive(move || {
    spot_lights.with(|lights| {
      lights.iter().position(|light| {
        light.with_untracked(|light| light.as_ref().map(|light| light.id()))
          == spot_light.with_untracked(|light| light.as_ref().map(|light| light.id()))
      })
    })
  });

  Effect::new(move |_| {
    let light = CoreSpotLight::builder()
      .direction(direction.get_untracked())
      .position(position.get_untracked())
      .color(color.get_untracked())
      .cutoff_angle(cutoff_angle.get_untracked())
      .build();

    spot_lights.update(|spot_lights| {
      spot_lights.push(spot_light);
    });

    spot_light.set(Some(light));
  });

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    let position = position.get();

    let Some(index) = index_in_storage.get() else {
      return;
    };

    spot_light.update_untracked(|light| {
      let Some(light) = light else {
        return;
      };

      light.set_position(position);

      scene.update_untracked(|scene| {
        if let Some(scene) = scene {
          scene.update_spot_light(&renderer, index, light);
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

    spot_light.update_untracked(|light| {
      let Some(light) = light else {
        return;
      };

      light.set_color(color);

      scene.update_untracked(|scene| {
        if let Some(scene) = scene {
          scene.update_spot_light(&renderer, index, light);
        }
      });
    });
  });

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    let direction = direction.get();

    let Some(index) = index_in_storage.get() else {
      return;
    };

    spot_light.update_untracked(|light| {
      let Some(light) = light else {
        return;
      };

      light.set_direction(direction);

      scene.update_untracked(|scene| {
        if let Some(scene) = scene {
          scene.update_spot_light(&renderer, index, light);
        }
      });
    });
  });

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    let cutoff_angle = cutoff_angle.get();

    let Some(index) = index_in_storage.get() else {
      return;
    };

    spot_light.update_untracked(|light| {
      let Some(light) = light else {
        return;
      };

      light.set_cutoff_angle(cutoff_angle);

      scene.update_untracked(|scene| {
        if let Some(scene) = scene {
          scene.update_spot_light(&renderer, index, light);
        }
      });
    });
  });

  on_cleanup(move || {
    spot_lights.update(|spot_lights| {
      if let Some(index_in_storage) = index_in_storage.get() {
        spot_lights.remove(index_in_storage);
      };
    });
  });

  ()
}
