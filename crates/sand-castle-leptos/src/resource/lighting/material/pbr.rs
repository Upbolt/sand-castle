use leptos::prelude::*;

use sand_castle_core::{
  resource::{
    lighting::material::{pbr::PbrMaterial as CorePbrMaterial, ToMaterial},
    object_3d::Object3D,
    texture::TextureId,
  },
  Vec4,
};

use crate::{resource::mesh::MeshContextValue, scene::SceneContextValue};

#[component]
pub fn PbrMaterial(
  #[prop(default = Vec4::new(0.0, 0.0, 0.0, 1.0).into(), into)] color: MaybeSignal<Vec4>,
  #[prop(default = 1.0.into(), into)] roughness: MaybeSignal<f32>,
  #[prop(default = 0.0.into(), into)] metalness: MaybeSignal<f32>,
  #[prop(optional, into)] diffuse_map_texture_id: MaybeProp<TextureId>,
) -> impl IntoView {
  let MeshContextValue {
    mesh, material_id, ..
  } = use_context().expect("`PbrMaterial` must be used in a `Mesh` component");

  let SceneContextValue {
    scene,
    renderer,
    material_loader,
    ..
  } = use_context().expect("`PbrMaterial` must be used in a Scene component");

  Effect::new(move |_| {
    leptos::logging::log!("hello");

    if material_loader.with(|loader| loader.is_none()) {
      return;
    }

    let pbr_material = CorePbrMaterial::builder()
      .color(color.get_untracked())
      .metalness(metalness.get_untracked())
      .roughness(roughness.get_untracked())
      .diffuse_map_texture_id(diffuse_map_texture_id.get_untracked())
      .build();

    let material = pbr_material.to_material();

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
    let roughness = roughness.get();
    let metalness = metalness.get();

    mesh.with(|mesh| {
      scene.with(|scene| {
        if let (Some(scene), Some(mesh)) = (scene, mesh) {
          scene.update_material_data(
            &renderer,
            mesh,
            bytemuck::cast_slice(&[color, Vec4::new(roughness, metalness, 0.0, 0.0)]),
          );
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
