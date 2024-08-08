use leptos::*;

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
  #[prop(optional, into)] yaw: MaybeProp<f32>,
  #[prop(optional, into)] pitch: MaybeProp<f32>,
  #[prop(into)] screen_size: MaybeSignal<Vec2>,
  #[prop(optional, into)] position: MaybeProp<Vec3>,
  #[prop(optional, into)] rotation: MaybeProp<Quat>,
  #[prop(optional, into)] scale: MaybeProp<Scale>,
) -> impl IntoView {
  let SceneContextValue { scene, renderer } =
    use_context().expect("`OrthographicCamera must be used in a `Scene` component");

  let camera = RwSignal::<Option<CoreOrthographicCamera>>::new(None);

  Effect::new(move |_| {
    let orthographic_camera = CoreOrthographicCamera::builder()
      .yaw(yaw.get_untracked().unwrap_or_default())
      .pitch(pitch.get_untracked().unwrap_or_default())
      .screen_size(screen_size.get_untracked())
      .position(position.get_untracked().unwrap_or_default())
      .rotation(rotation.get_untracked().unwrap_or_default())
      .scale(scale.get_untracked().unwrap_or_default())
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

  view! {}
}
