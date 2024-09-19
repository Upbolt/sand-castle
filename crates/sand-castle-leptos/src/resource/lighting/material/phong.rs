use leptos::prelude::*;

use sand_castle_core::{
  resource::{
    lighting::material::{phong::PhongMaterial as CorePhongMaterial, ToMaterial},
    object_3d::Object3D,
    texture::TextureId,
  },
  Vec4,
};

use crate::{resource::mesh::MeshContextValue, scene::SceneContextValue};

#[component]
pub fn PhongMaterial(
  #[prop(default = Vec4::new(0.0, 0.0, 0.0, 1.0).into(), into)] color: MaybeSignal<Vec4>,
  #[prop(optional, into)] diffuse_map_texture_id: MaybeProp<TextureId>,
) -> impl IntoView {
  let MeshContextValue {
    mesh, material_id, ..
  } = use_context().expect("`PhongMaterial` must be used in a `Mesh` component");

  let SceneContextValue {
    scene,
    renderer,
    material_loader,
    ..
  } = use_context().expect("`PhongMaterial` must be used in a Scene component");

  Effect::new(move |_| {
    if material_loader.with(|loader| loader.is_none()) {
      return;
    }

    let phong_material = CorePhongMaterial::builder()
      .color(color.get_untracked())
      .diffuse_map_texture_id(diffuse_map_texture_id.get_untracked())
      .build();

    let material = phong_material.to_material();

    let id = *material.id();

    material_loader.update_untracked(|loader| {
      if let Some(loader) = loader {
        loader.insert(material);
      }
    });

    material_id.set(Some(id));
  });

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    let color = color.get();

    mesh.with(|mesh| {
      scene.with(|scene| {
        if let (Some(scene), Some(mesh)) = (scene, mesh) {
          scene.update_material_data(&renderer, mesh, bytemuck::cast_slice(&[color]));
        }
      });
    });
  });

  on_cleanup(move || {
    mesh.with(|mesh| {
      scene.update(|scene| {
        if let (Some(scene), Some(mesh)) = (scene, mesh) {
          scene.remove_material(mesh);
        }
      });
    });
  });

  view! {}
}
