use leptos::*;

use sand_castle_core::{
  resource::{
    object_3d::{mesh::Mesh as CoreMesh, Scale},
    Resource,
  },
  Quat, Vec3,
};

use crate::scene::SceneContextValue;

#[derive(Clone)]
pub struct MeshContextValue {
  pub mesh: RwSignal<Option<CoreMesh>>,
}

#[component]
pub fn Mesh(
  #[prop(optional, into)] position: MaybeProp<Vec3>,
  #[prop(optional, into)] rotation: MaybeProp<Quat>,
  #[prop(optional, into)] scale: MaybeProp<Scale>,

  children: Children,
) -> impl IntoView {
  let mesh = RwSignal::<Option<CoreMesh>>::new(None);

  let SceneContextValue { scene, renderer } =
    use_context().expect("`Mesh` must be used in a `Scene` component");

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
    mesh.with(|mesh| {
      mesh.as_ref().map(|mesh| {
        logging::log!("{:#?}", mesh.geometry());
      })
    });
  });

  Effect::new(move |_| {
    let (Some(position), Some(renderer)) = (position.get(), renderer.get()) else {
      return;
    };

    mesh.update_untracked(|mesh| {
      let Some(mesh) = mesh else {
        return;
      };

      scene.with(|scene| {
        if let Some(scene) = scene {
          scene.transform_pos(&renderer, mesh, position);
        }
      });
    });
  });

  Effect::new(move |_| {
    let (Some(rotation), Some(renderer)) = (rotation.get(), renderer.get()) else {
      return;
    };

    mesh.update_untracked(|mesh| {
      let Some(mesh) = mesh else {
        return;
      };

      // scene.with(|scene| {
      //   if let Some(scene) = scene {
      //     scene.transform_rot(&renderer, mesh, rotation);
      //   }
      // });
    });
  });

  Effect::new(move |_| {
    let (Some(scale), Some(renderer)) = (scale.get(), renderer.get()) else {
      return;
    };

    mesh.update_untracked(|mesh| {
      let Some(mesh) = mesh else {
        return;
      };

      scene.with(|scene| {
        if let Some(scene) = scene {
          scene.transform_scale(&renderer, mesh, scale);
        }
      });
    });
  });

  on_cleanup(move || {
    mesh.with(|mesh| {
      let Some(mesh) = mesh else {
        return;
      };

      scene.update(|scene| {
        if let Some(scene) = scene {
          scene.remove(mesh);
        }
      });
    });
  });

  provide_context(MeshContextValue { mesh });

  children().into_view()
}
