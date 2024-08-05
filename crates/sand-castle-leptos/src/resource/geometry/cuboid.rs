use leptos::*;

use sand_castle_core::resource::{
  geometry::{cuboid::Cuboid as CoreCuboid, ToGeometry},
  object_3d::Object3D,
};

use crate::resource::mesh::MeshContextValue;

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

  let MeshContextValue { mesh } =
    use_context().expect("`Cuboid` must be used in a `Mesh` component");

  Effect::new(move |_| {
    if cuboid.with(|cuboid| cuboid.is_none()) {
      cuboid.set(Some(
        CoreCuboid::builder()
          .width(width.get().unwrap())
          .height(height.get().unwrap())
          .depth(depth.get().unwrap())
          .width_segments(width_segments.get().unwrap())
          .height_segments(height_segments.get().unwrap())
          .depth_segments(depth_segments.get().unwrap())
          .build(),
      ));
    }
  });

  Effect::new(move |_| {
    let Some(geometry) = cuboid.with(|cuboid| cuboid.as_ref().map(|cuboid| cuboid.to_geometry()))
    else {
      return;
    };

    mesh.update(|mesh| {
      if let Some(mesh) = mesh {
        mesh.set_geometry(geometry);
      }
    });
  });

  view! {}
}
