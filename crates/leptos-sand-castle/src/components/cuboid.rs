use leptos::{leptos_dom::Transparent, *};

use sand_castle::geometry::{cuboid, WithGeometry};

#[component(transparent)]
pub fn Cuboid() -> impl IntoView {
  View::Transparent(Transparent::new(
    cuboid::Cuboid::builder()
      .build()
      .expect("failed to create cuboid")
      .into_geometry(),
  ))
}
