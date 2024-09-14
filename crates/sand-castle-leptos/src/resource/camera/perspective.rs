use leptos::prelude::*;

pub use sand_castle_core::{
  resource::{
    camera::{perspective::PerspectiveCamera as CorePerspectiveCamera, ViewFrustum},
    object_3d::{Scale, Transform},
  },
  Quat, Vec3,
};

use crate::scene::SceneContextValue;

#[component]
pub fn PerspectiveCamera(
  #[prop(optional, into)] yaw: MaybeSignal<f32>,
  #[prop(optional, into)] pitch: MaybeSignal<f32>,
  #[prop(default=70.0.into(), into)] fov: MaybeSignal<f32>,
  #[prop(into)] aspect_ratio: MaybeSignal<f32>,
  #[prop(optional, into)] view_frustum: MaybeSignal<ViewFrustum>,
  #[prop(optional, into)] position: MaybeSignal<Vec3>,
  #[prop(optional, into)] rotation: MaybeSignal<Quat>,
  #[prop(optional, into)] scale: MaybeSignal<Scale>,
) -> impl IntoView {
  let SceneContextValue {
    scene, renderer, ..
  } = use_context().expect("`PerspectiveCamera must be used in a `Scene` component");

  let camera = RwSignal::<Option<CorePerspectiveCamera>>::new(None);

  let yaw = Memo::new(move |_| yaw.get());
  let pitch = Memo::new(move |_| pitch.get());
  let position = Memo::new(move |_| position.get());

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    let perspective_camera = CorePerspectiveCamera::builder()
      .yaw(yaw.get_untracked())
      .pitch(pitch.get_untracked())
      .fov(fov.get_untracked())
      .aspect_ratio(aspect_ratio.get_untracked())
      .view_frustum(view_frustum.get_untracked())
      .position(position.get_untracked())
      .rotation(rotation.get_untracked())
      .scale(scale.get_untracked())
      .build();

    scene.update(|scene| {
      if let Some(scene) = scene {
        scene.set_camera(&renderer, &perspective_camera);
      }
    });

    camera.set(Some(perspective_camera));
  });

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    let yaw = yaw.get();

    camera.update(|camera| {
      let Some(camera) = camera else {
        return;
      };

      camera.set_yaw(yaw);

      scene.with(|scene| {
        if let Some(scene) = scene {
          scene.update_camera(&renderer, camera);
        }
      });
    });
  });

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    let pitch = pitch.get();

    camera.update(|camera| {
      let Some(camera) = camera else {
        return;
      };

      camera.set_pitch(pitch);

      scene.with(|scene| {
        if let Some(scene) = scene {
          scene.update_camera(&renderer, camera);
        }
      });
    });
  });

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    let position = position.get();

    camera.update(|camera| {
      let Some(camera) = camera else {
        return;
      };

      camera.set_pos(position);

      scene.with(|scene| {
        if let Some(scene) = scene {
          scene.update_camera(&renderer, camera);
        }
      });
    });
  });

  ()
}
