use leptos::*;

use sand_castle_core::resource::{
  geometry::{cuboid::Cuboid as CoreCuboid, ToGeometry},
  object_3d::Object3D,
};

use crate::{resource::mesh::MeshContextValue, scene::SceneContextValue};

#[component]
pub fn Cuboid(
  #[prop(default=1.0.into())] width: MaybeProp<f32>,
  #[prop(default=1.0.into())] height: MaybeProp<f32>,
  #[prop(default=1.0.into())] depth: MaybeProp<f32>,
  #[prop(default=1.into())] width_segments: MaybeProp<u32>,
  #[prop(default=1.into())] height_segments: MaybeProp<u32>,
  #[prop(default=1.into())] depth_segments: MaybeProp<u32>,
) -> impl IntoView {
  let cuboid = RwSignal::<Option<CoreCuboid>>::new(None);

  let SceneContextValue { scene, renderer } =
    use_context().expect("`Cuboid` must be used in a `Scene` component");

  let MeshContextValue { geometry, .. } =
    use_context().expect("`Cuboid` must be used in a `Mesh` component");

  Effect::new(move |_| {
    cuboid.set(Some(
      CoreCuboid::builder()
        .width(width.get_untracked().unwrap())
        .height(height.get_untracked().unwrap())
        .depth(depth.get_untracked().unwrap())
        .width_segments(width_segments.get_untracked().unwrap())
        .height_segments(height_segments.get_untracked().unwrap())
        .depth_segments(depth_segments.get_untracked().unwrap())
        .build(),
    ));
  });

  Effect::new(move |_| {
    let (Some(cuboid_geometry), Some(renderer)) = (
      cuboid.with(|cuboid| cuboid.as_ref().map(|cuboid| cuboid.to_geometry())),
      renderer.get(),
    ) else {
      return;
    };

    geometry.set(Some(cuboid_geometry));
  });

  view! {}
}
