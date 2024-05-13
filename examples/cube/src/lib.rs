use sand_castle::{
  camera::{perspective::PerspectiveCamera, ViewFrustum},
  geometry::{torus::Torus, cuboid::Cuboid, WithGeometry},
  material::{mesh_basic::MeshBasic, WithMaterial},
  object::mesh::Mesh,
  renderer::{Driver, Renderer},
  scene::Scene,
  units::*,
};

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, Element, HtmlCanvasElement};
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[wasm_bindgen(start)]
async fn main() {
  console_error_panic_hook::set_once();

  let web_window = window().expect("could not get window");
  let document = web_window.document().expect("could not get document");
  let canvas = document
    .get_element_by_id("cube-example")
    .expect("could not find canvas")
    .dyn_into::<HtmlCanvasElement>()
    .expect("could not convert into canvas");

  let mut scene = Scene::new();

  let camera = PerspectiveCamera::builder()
    .field_of_view(70.)
    .aspect_ratio(canvas.client_width() as f64 / canvas.client_height() as f64)
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

  let renderer = Renderer::builder()
    .driver(Driver::WebGL)
    .size((canvas.client_width() as i32, canvas.client_height() as i32))
    .pixel_ratio(web_window.device_pixel_ratio())
    .canvas(canvas)
    .build()
    .await
    .expect("could not build renderer");

  let cube = Mesh::new(
    &renderer,
    Cuboid::builder()
      .build()
      .expect("could not build cuboid"),
    MeshBasic::builder()
      .color(0xFF6347)
      .wireframe(true)
      .build()
      .expect("could not build material")
  );

  let torus = Mesh::new(
    &renderer,
    Torus::builder()
      .radius(10.)
      .tube(3.)
      .radial_segments(16)
      .arc(100.)
      .build()
      .expect("could not build torus"),
    MeshBasic::builder()
      .color(0xFF6347)
      .wireframe(true)
      .build()
      .expect("could not build material"),
  );

  scene.push(cube);
  scene.push(torus);

  animation_loop(move || {
    renderer.render(&scene);
  });
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
  window()
    .expect("could not get window")
    .request_animation_frame(f.as_ref().unchecked_ref())
    .expect("should register `requestAnimationFrame` OK");
}

fn animation_loop(mut callback: impl FnMut() + 'static) {
  let f = Rc::new(RefCell::new(None));
  let g = f.clone();

  *g.borrow_mut() = Some(Closure::new(move || {
    callback();
    
    request_animation_frame(f.borrow().as_ref().unwrap());
  }));

  request_animation_frame(g.borrow().as_ref().unwrap());
}
