use leptos::prelude::*;

pub use sand_castle_core::{
  resource::{
    camera::{orthographic::OrthographicCamera as CoreOrthographicCamera, ViewFrustum},
    object_3d::Scale,
  },
  Quat, Vec2, Vec3,
};

use crate::scene::SceneContextValue;

#[component]
pub fn OrthographicCamera(
  #[prop(optional, into)] yaw: MaybeSignal<f32>,
  #[prop(optional, into)] pitch: MaybeSignal<f32>,
  #[prop(into)] screen_size: MaybeSignal<Vec2>,
  #[prop(optional, into)] position: MaybeSignal<Vec3>,
  #[prop(optional, into)] rotation: MaybeSignal<Quat>,
  #[prop(optional, into)] scale: MaybeSignal<Scale>,
) -> impl IntoView {
  let SceneContextValue {
    scene, renderer, ..
  } = use_context().expect("`OrthographicCamera must be used in a `Scene` component");

  let camera = RwSignal::<Option<CoreOrthographicCamera>>::new(None);

  Effect::new(move |_| {
    let orthographic_camera = CoreOrthographicCamera::builder()
      .yaw(yaw.get_untracked())
      .pitch(pitch.get_untracked())
      .screen_size(screen_size.get_untracked())
      .position(position.get_untracked())
      .rotation(rotation.get_untracked())
      .scale(scale.get_untracked())
      .build();

    scene.update(|scene| {
      let Some(renderer) = renderer.get() else {
        return;
      };

      if let Some(scene) = scene {
        scene.set_camera(&renderer, &orthographic_camera);
      }
    });

    camera.set(Some(orthographic_camera));
  });

  ()
}
