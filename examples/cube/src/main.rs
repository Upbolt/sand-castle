use sand_castle::scene::Scene;
use web_sys::{window, Element};

fn main() {
  console_error_panic_hook::set_once();

  let window = window().expect("could not get window");
  let document = window.document().expect("could not get document");
  let body = document.body().expect("could not get body");

  let mut scene = Scene::new();
  let camera = PerspectiveCamera::builder()
    .field_of_view(70.)
    .aspect_ratio(window.inner_width() / window.inner_height())
    .view_frustum(ViewFrustum {
      near: 0.1,
      far: 1000.,
    })
    .position(Vector3 {
      z: 30.,
      ..Default::default()
    })
    .build();

  let canvas = Element::from(window.canvas().expect("could not get canvas"));

  body
    .append_child(&canvas)
    .expect("could not append canvas to body");

  let renderer = Renderer::builder()
    .driver(Driver::WebGL)
    .canvas(canvas)
    .pixel_ratio(window.device_pixel_ratio())
    .size((window.inner_width(), window.inner_height()))
    .build();

  let geometry = Torus::builder()
    .radius(10.)
    .tube(3.)
    .radial_segments(16)
    .arc(100)
    .build();

  let material = materials::MeshBasic::builder()
    .color(0xFF6347)
    .wireframe(true)
    .build();

  let torus = Mesh::from_geometry(geometry, material);

  scene.push(torus);
}
