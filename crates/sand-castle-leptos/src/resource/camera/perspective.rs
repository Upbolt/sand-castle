use leptos::*;

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
  #[prop(optional, into)] yaw: MaybeProp<f32>,
  #[prop(optional, into)] pitch: MaybeProp<f32>,
  #[prop(default=70.0.into(), into)] fov: MaybeSignal<f32>,
  #[prop(into)] aspect_ratio: MaybeSignal<f32>,
  #[prop(optional, into)] view_frustum: MaybeProp<ViewFrustum>,
  #[prop(optional, into)] position: MaybeProp<Vec3>,
  #[prop(optional, into)] rotation: MaybeProp<Quat>,
  #[prop(optional, into)] scale: MaybeProp<Scale>,
) -> impl IntoView {
  let SceneContextValue {
    scene, renderer, ..
  } = use_context().expect("`PerspectiveCamera must be used in a `Scene` component");

  let camera = RwSignal::<Option<CorePerspectiveCamera>>::new(None);

  let yaw = Memo::new(move |_| yaw.get());
  let pitch = Memo::new(move |_| pitch.get());
  let position = Memo::new(move |_| position.get());

  Effect::new(move |_| {
    if camera.with(|camera| camera.is_none()) {
      let perspective_camera = CorePerspectiveCamera::builder()
        .yaw(yaw.get().unwrap_or_default())
        .pitch(pitch.get().unwrap_or_default())
        .fov(fov.get())
        .aspect_ratio(aspect_ratio.get())
        .view_frustum(view_frustum.get().unwrap_or_default())
        .position(position.get().unwrap_or_default())
        .rotation(rotation.get().unwrap_or_default())
        .scale(scale.get().unwrap_or_default())
        .build();

      scene.update(|scene| {
        let Some(renderer) = renderer.get() else {
          return;
        };

        if let Some(scene) = scene {
          scene.set_camera(&renderer, &perspective_camera);
        }
      });

      camera.set(Some(perspective_camera));
    }
  });

  Effect::new(move |_| {
    let (Some(yaw), Some(renderer)) = (yaw.get(), renderer.get()) else {
      return;
    };

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
    let (Some(pitch), Some(renderer)) = (pitch.get(), renderer.get()) else {
      return;
    };

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
    let (Some(position), Some(renderer)) = (position.get(), renderer.get()) else {
      return;
    };

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
