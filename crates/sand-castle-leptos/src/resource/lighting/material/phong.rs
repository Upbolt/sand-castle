use leptos::*;

use sand_castle_core::{
  resource::{
    lighting::material::{phong::PhongMaterial as CorePhongMaterial, ToMaterial},
    object_3d::Object3D,
  },
  Vec4,
};

use crate::{resource::mesh::MeshContextValue, scene::SceneContextValue};

#[component]
pub fn PhongMaterial(
  #[prop(default = Vec4::new(0.0, 0.0, 0.0, 1.0).into(), into)] color: MaybeProp<Vec4>,
) -> impl IntoView {
  let MeshContextValue { mesh, material, .. } =
    use_context().expect("`PhongMaterial` must be used in a `Mesh` component");

  let SceneContextValue {
    scene, renderer, ..
  } = use_context().expect("`PhongMaterial` must be used in a Scene component");

  Effect::new(move |_| {
    let phong_material = CorePhongMaterial::with_color(
      color
        .get_untracked()
        .unwrap_or(Vec4::new(0.0, 0.0, 0.0, 1.0)),
    );

    material.set(Some(phong_material.to_material()));
  });

  Effect::new(move |_| {
    let (Some(color), Some(renderer)) = (color.get(), renderer.get()) else {
      return;
    };

    mesh.with(|mesh| {
      scene.with(|scene| {
        if let (Some(scene), Some(mesh)) = (scene, mesh) {
          scene.update_material_data(&renderer, mesh, bytemuck::cast_slice(&[color]));
        }
      });
    });
  });

  view! {}
}
