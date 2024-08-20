use leptos::*;

use sand_castle_core::{
  resource::{
    geometry::Geometry,
    lighting::material::Material,
    object_3d::{mesh::Mesh as CoreMesh, Scale, SceneTransform},
    Resource,
  },
  Quat, Vec3,
};

use crate::scene::SceneContextValue;

#[derive(Clone)]
pub struct MeshContextValue {
  pub mesh: RwSignal<Option<CoreMesh>>,
  pub geometry: RwSignal<Option<Geometry>>,
  pub material: RwSignal<Option<Material>>,
}

#[component]
pub fn Mesh(
  #[prop(optional, into)] position: MaybeProp<Vec3>,
  #[prop(optional, into)] rotation: MaybeProp<Quat>,
  #[prop(optional, into)] scale: MaybeProp<Scale>,

  children: Children,
) -> impl IntoView {
  let mesh = RwSignal::<Option<CoreMesh>>::new(None);

  let geometry = RwSignal::<Option<Geometry>>::new(None);
  let material = RwSignal::<Option<Material>>::new(None);

  let SceneContextValue {
    scene, renderer, ..
  } = use_context().expect("`Mesh` must be used in a `Scene` component");

  let position = Memo::new(move |_| position.get());
  let rotation = Memo::new(move |_| rotation.get());
  let scale = Memo::new(move |_| scale.get());

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    let mesh_desc = CoreMesh::builder()
      .position(position.get_untracked().unwrap_or_default())
      .rotation(rotation.get_untracked().unwrap_or_default())
      .scale(scale.get_untracked().unwrap_or_default())
      .build();

    scene.update(|scene| {
      if let Some(scene) = scene {
        scene.insert(&renderer, &mesh_desc);
      }
    });

    mesh.set(Some(mesh_desc));
  });

  Effect::new(move |_| {
    let (Some(geometry), Some(renderer)) = (geometry.get(), renderer.get()) else {
      return;
    };

    scene.update(|scene| {
      mesh.update(|mesh| {
        if let (Some(scene), Some(mesh)) = (scene, mesh) {
          scene.update_geometry(&renderer, mesh, geometry);
        }
      });
    });
  });

  Effect::new(move |_| {
    let (Some(material), Some(renderer)) = (material.get(), renderer.get()) else {
      return;
    };

    scene.update(|scene| {
      mesh.update(|mesh| {
        if let (Some(scene), Some(mesh)) = (scene, mesh) {
          scene.update_material(&renderer, mesh, material);
        }
      });
    });
  });

  Effect::new(move |_| {
    let (Some(position), Some(renderer)) = (position.get(), renderer.get()) else {
      return;
    };

    mesh.update(|mesh| {
      scene.with(|scene| {
        if let (Some(scene), Some(mesh)) = (scene, mesh) {
          mesh.update_pos(scene, &renderer, position);
        }
      });
    });
  });

  Effect::new(move |_| {
    let (Some(rotation), Some(renderer)) = (rotation.get(), renderer.get()) else {
      return;
    };

    mesh.update(|mesh| {
      scene.with(|scene| {
        if let (Some(scene), Some(mesh)) = (scene, mesh) {
          mesh.update_rot(scene, &renderer, rotation);
        }
      });
    });
  });

  Effect::new(move |_| {
    let (Some(scale), Some(renderer)) = (scale.get(), renderer.get()) else {
      return;
    };

    mesh.update(|mesh| {
      scene.with(|scene| {
        if let (Some(scene), Some(mesh)) = (scene, mesh) {
          mesh.update_scale(scene, &renderer, scale);
        }
      });
    });
  });

  on_cleanup(move || {
    mesh.with(|mesh| {
      scene.update(|scene| {
        if let (Some(scene), Some(mesh)) = (scene, mesh) {
          scene.remove(mesh);
        }
      });
    });
  });

  provide_context(MeshContextValue {
    mesh,
    geometry,
    material,
  });

  children().into_view()
}
