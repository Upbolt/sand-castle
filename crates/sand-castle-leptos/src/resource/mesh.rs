use std::sync::Arc;

use leptos::prelude::*;

use sand_castle_core::{
  resource::{
    geometry::{Geometry, ToGeometry},
    lighting::material::Material,
    object_3d::{mesh::Mesh as CoreMesh, Scale, SceneTransform},
    Id, Resource,
  },
  Quat, Vec3,
};

use crate::scene::SceneContextValue;

#[derive(Clone)]
pub struct MeshContextValue {
  pub mesh: RwSignal<Option<CoreMesh>, LocalStorage>,
  pub geometry_id: RwSignal<Option<Id>, LocalStorage>,
  pub material_id: RwSignal<Option<Id>, LocalStorage>,
}

#[component]
pub fn Mesh(
  #[prop(optional, into)] position: MaybeSignal<Vec3>,
  #[prop(optional, into)] rotation: MaybeSignal<Quat>,
  #[prop(optional, into)] scale: MaybeSignal<Scale>,

  #[prop(optional, into)] geometry_id: MaybeProp<Id>,
  #[prop(optional, into)] material_id: MaybeProp<Id>,

  children: Children,
) -> impl IntoView {
  let mesh = RwSignal::new_local(None);

  let inner_geometry_id = RwSignal::new_local(None);
  let inner_material_id = RwSignal::new_local(None);

  let SceneContextValue {
    scene,
    renderer,
    geometry_loader,
    material_loader,
    texture_loader,
    ..
  } = use_context().expect("`Mesh` must be used in a `Scene` component");

  let position = Memo::new(move |_| position.get());
  let rotation = Memo::new(move |_| rotation.get());
  let scale = Memo::new(move |_| scale.get());

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    let mesh_desc = CoreMesh::builder()
      .position(position.get_untracked())
      .rotation(rotation.get_untracked())
      .scale(scale.get_untracked())
      .build();

    geometry_loader.with_untracked(|geometry_loader| {
      material_loader.with_untracked(|material_loader| {
        texture_loader.with_untracked(|texture_loader| {
          scene.update(|scene| {
            if let (
              Some(scene),
              Some(geometry_loader),
              Some(material_loader),
              Some(texture_loader),
            ) = (scene, geometry_loader, material_loader, texture_loader)
            {
              scene.insert(
                &renderer,
                geometry_loader,
                material_loader,
                texture_loader,
                &mesh_desc,
              );
            }
          });
        });
      });
    });

    mesh.set(Some(mesh_desc));
  });

  Effect::new(move |_| {
    let geometry_id = geometry_id.get();

    if geometry_id.is_some() {
      inner_geometry_id.set(geometry_id);
    }
  });

  Effect::new(move |_| {
    let material_id = material_id.get();

    if material_id.is_some() {
      inner_material_id.set(material_id);
    }
  });

  Effect::new(move |_| {
    let (Some(inner_geometry_id), Some(renderer)) = (inner_geometry_id.get(), renderer.get())
    else {
      return;
    };

    geometry_loader.with(|loader| {
      scene.update(|scene| {
        mesh.update(|mesh| {
          if let (Some(scene), Some(mesh), Some(loader)) = (scene, mesh, loader) {
            scene.update_geometry(&renderer, mesh, loader, inner_geometry_id);
          }
        });
      });
    });
  });

  Effect::new(move |_| {
    let (Some(inner_material_id), Some(renderer)) = (inner_material_id.get(), renderer.get())
    else {
      return;
    };

    material_loader.with(|material_loader| {
      texture_loader.with(|texture_loader| {
        scene.update(|scene| {
          mesh.update(|mesh| {
            if let (Some(scene), Some(mesh), Some(material_loader), Some(texture_loader)) =
              (scene, mesh, material_loader, texture_loader)
            {
              scene.update_material(
                &renderer,
                mesh,
                texture_loader,
                material_loader,
                inner_material_id,
              );
            }
          });
        });
      });
    });
  });

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    let position = position.get();

    mesh.update(|mesh| {
      scene.with(|scene| {
        if let (Some(scene), Some(mesh)) = (scene, mesh) {
          mesh.update_pos(scene, &renderer, position);
        }
      });
    });
  });

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    let rotation = rotation.get();

    mesh.update(|mesh| {
      scene.with(|scene| {
        if let (Some(scene), Some(mesh)) = (scene, mesh) {
          mesh.update_rot(scene, &renderer, rotation);
        }
      });
    });
  });

  Effect::new(move |_| {
    let Some(renderer) = renderer.get() else {
      return;
    };

    let scale = scale.get();

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
    geometry_id: inner_geometry_id,
    material_id: inner_material_id,
  });

  children().into_view()
}
