use leptos::*;

use sand_castle::object::mesh;

#[component]
pub fn Mesh(
  // node_ref: ,
  children: Children,
) -> impl IntoView {
  let (mesh, set_mesh) = create_signal::<Option<mesh::Mesh>>(None);

  view! {}
}
