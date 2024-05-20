use std::borrow::Borrow;

use leptos::{leptos_dom::Transparent, *};

use sand_castle::{geometry::Geometry, material::Material, object::mesh};

use crate::components::canvas::CanvasContextValue;

#[component]
pub fn Mesh(
  // node_ref: ,
  children: Children,
) -> impl IntoView {
  let CanvasContextValue { renderer, .. } =
    use_context().expect("Mesh must be used in a Canvas component");

  let (mesh, set_mesh) = create_signal::<Option<mesh::Mesh>>(None);

  let children = children();
  let children = children.as_children();

  let material = children
    .iter()
    .filter_map(|child| {
      child
        .as_transparent()
        .and_then(|view| view.downcast_ref::<Material>())
    })
    .next();

  let geometry = children
    .iter()
    .filter_map(|child| {
      child
        .as_transparent()
        .and_then(|view| view.downcast_ref::<Geometry>())
    })
    .next();

  let renderer = renderer.get().unwrap().clone();

  View::Transparent(Transparent::new(mesh::Mesh::from_parts(
    &renderer.as_ref().borrow(),
    geometry.unwrap().clone(),
    material.unwrap().clone(),
  )))
}
