use leptos::*;

use sand_castle_core::resource::{
  lighting::material::{basic::BasicMaterial as CoreBasicMaterial, ToMaterial},
  object_3d::Object3D,
};

use crate::{resource::mesh::MeshContextValue, scene::SceneContextValue};

#[component]
pub fn BasicMaterial(#[prop(default = 0x000000FF.into())] color: MaybeProp<u32>) -> impl IntoView {
  let material = RwSignal::<Option<CoreBasicMaterial>>::new(None);

  let MeshContextValue { mesh, .. } =
    use_context().expect("`BasicMaterial` must be used in a `Mesh` component");

  let SceneContextValue { scene, .. } =
    use_context().expect("`BasicMaterial` must be used in a Scene component");

  Effect::new(move |_| {
    let basic_material = CoreBasicMaterial::with_color(color.get().unwrap());

    // mesh.update(|mesh| {
    //   if let Some(mesh) = mesh {
    //     scene.update(|scene| {
    //       if let Some(scene) = scene {
    //         scene.update_material(mesh, basic_material.to_material());
    //       }
    //     });
    //   }
    // });

    material.set(Some(basic_material));
  });

  view! {}
}
