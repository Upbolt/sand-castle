use std::sync::Arc;

use leptos::prelude::*;

use sand_castle_core::resource::{
  geometry::{cuboid::Cuboid as CoreCuboid, ToGeometry},
  object_3d::Object3D,
};

use crate::{resource::mesh::MeshContextValue, scene::SceneContextValue};

#[component]
pub fn Cuboid(
  #[prop(default=1.0.into())] width: MaybeSignal<f32>,
  #[prop(default=1.0.into())] height: MaybeSignal<f32>,
  #[prop(default=1.0.into())] depth: MaybeSignal<f32>,
  #[prop(default=1.into())] width_segments: MaybeSignal<u32>,
  #[prop(default=1.into())] height_segments: MaybeSignal<u32>,
  #[prop(default=1.into())] depth_segments: MaybeSignal<u32>,
) -> impl IntoView {
  let cuboid = RwSignal::<Option<CoreCuboid>>::new(None);

  let SceneContextValue {
    scene,
    renderer,
    geometry_loader,
    ..
  } = use_context().expect("`Cuboid` must be used in a `Scene` component");

  let MeshContextValue { geometry_id, .. } =
    use_context().expect("`Cuboid` must be used in a `Mesh` component");

  Effect::new(move |_| {
    if geometry_loader.with(|loader| loader.is_none()) {
      return;
    }

    let cuboid_descriptor = CoreCuboid::builder()
      .width(width.get_untracked())
      .height(height.get_untracked())
      .depth(depth.get_untracked())
      .width_segments(width_segments.get_untracked())
      .height_segments(height_segments.get_untracked())
      .depth_segments(depth_segments.get_untracked())
      .build();

    let geometry = cuboid_descriptor.to_geometry();

    let id = *geometry.id();

    geometry_loader.update_untracked(|loader| {
      if let Some(loader) = loader {
        loader.insert(geometry);
      }
    });

    geometry_id.set(Some(id));
    cuboid.set(Some(cuboid_descriptor));
  });

  ()
}
