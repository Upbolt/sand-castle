use sand_castle::{
  camera::{perspective::PerspectiveCamera, ViewFrustum},
  geometry::torus::Torus,
  material,
  object::mesh::Mesh,
  renderer::{Driver, Renderer},
  scene::Scene,
  units::*,
};

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, Element, HtmlCanvasElement};

#[wasm_bindgen(start)]
async fn run() {
  console_error_panic_hook::set_once();

  let web_window = window().expect("could not get window");
  let document = web_window.document().expect("could not get document");
  let canvas = document
    .get_element_by_id("cube-example")
    .expect("could not find canvas")
    .dyn_into::<HtmlCanvasElement>()
    .expect("could not convert into canvas");

  setup_scene(canvas).await;
}

async fn setup_scene(canvas: HtmlCanvasElement) -> (Scene, Renderer) {
  let web_window = window().expect("could not get window");
  let document = web_window.document().expect("could not get document");
  let body = document.body().expect("could not get body");

  let mut scene = Scene::new();

  let inner_width = web_window
    .inner_width()
    .expect("could not get inner window width")
    .as_f64()
    .expect("could not get inner window width as f64");

  let inner_height = web_window
    .inner_width()
    .expect("could not get inner window width")
    .as_f64()
    .expect("could not get inner window width as f64");

  let camera = PerspectiveCamera::builder()
    .field_of_view(70.)
    .aspect_ratio(inner_width / inner_height)
    .view_frustum(ViewFrustum {
      near: 0.1,
      far: 1000.,
    })
    .position(Vector3 {
      z: 30.,
      ..Default::default()
    })
    .build()
    .expect("could not build camera");

  body
    .append_child(&canvas)
    .expect("could not append canvas to body");

  let renderer = Renderer::builder()
    .driver(Driver::WebGL)
    .canvas(canvas)
    .pixel_ratio(web_window.device_pixel_ratio())
    .size((inner_width, inner_height))
    .build()
    .await
    .expect("could not build renderer");

  let torus = Mesh::from_geometry(
    Torus::builder()
      .radius(10.)
      .tube(3.)
      .radial_segments(16)
      .arc(100)
      .build()
      .expect("could not build torus"),
    material::mesh_basic::MeshBasic::builder()
      .color(0xFF6347)
      .wireframe(true)
      .build()
      .expect("could not build material"),
  );

  scene.push(torus);

  (scene, renderer)
}
