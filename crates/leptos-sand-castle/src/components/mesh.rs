use leptos::*;

use sand_castle::object::mesh;

#[component]
pub fn Mesh(
  // node_ref: ,
  children: Children,
) -> impl IntoView {
  let (mesh, set_mesh) = create_signal::<Option<mesh::Mesh>>(None);

  let children = children().as_children();

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

  View::Transparent()
}
