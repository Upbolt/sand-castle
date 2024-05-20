use sand_castle::{
  camera::{perspective::PerspectiveCamera, ViewFrustum, WithCamera},
  geometry::{torus::Torus, cuboid::Cuboid, WithGeometry},
  material::{mesh_basic::MeshBasic, WithMaterial},
  object::mesh::Mesh,
  renderer::{Driver, Renderer},
  scene::Scene,
  units::*,
};

use std::ops::Deref;

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, Element, HtmlCanvasElement, KeyboardEvent, MouseEvent};
use std::cell::RefCell;
use std::rc::Rc;

use web_time::{Instant, Duration};
use cgmath::{Deg, Rad};

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

  let renderer = Rc::new(RefCell::new(Renderer::builder()
    .driver(Driver::WebGL)
    .size((canvas.client_width() as i32, canvas.client_height() as i32))
    .pixel_ratio(web_window.device_pixel_ratio())
    .canvas(canvas.clone())
    .build()
    .await
    .expect("could not build renderer")));

  let perspective_camera = Rc::new(RefCell::new(PerspectiveCamera::builder()
    .field_of_view(70.)
    .aspect_ratio(canvas.client_width() as f32 / canvas.client_height() as f32)
    .view_frustum(ViewFrustum {
      near: 0.1,
      far: 1000.,
    })
    // .position(Vector3 {
    //   z: 30.,
    //   ..Default::default()
    // })
    .build()
    .expect("could not build perspective camera")));

  let cube = Mesh::new(
    &renderer.borrow(),
    Cuboid::builder()
      .width(6.)
      .height(6.)
      .depth(6.)
      .build()
      .expect("could not build cuboid"),
    MeshBasic::builder()
      .color(0xFF6347)
      .wireframe(true)
      .build()
      .expect("could not build material")
  );

  // let torus = Mesh::new(
  //   &renderer,
  //   Torus::builder()
  //     .radius(10.)
  //     .tube(3.)
  //     .radial_segments(16)
  //     .arc(100.)
  //     .build()
  //     .expect("could not build torus"),
  //   MeshBasic::builder()
  //     .color(0xFF6347)
  //     .wireframe(true)
  //     .build()
  //     .expect("could not build material"),
  // );

  scene.push(cube);
  // scene.push(torus);

  let camera_renderer = Rc::clone(&renderer);
  let camera = Rc::new(Rc::clone(&perspective_camera)
    .borrow()
    .to_camera(&camera_renderer.borrow()));
  // let cloned_perspective_camera = Rc::clone(&perspective_camera);

  let render_time = Rc::new(RefCell::new(Instant::now()));
  let render_delta = Rc::new(RefCell::new(Duration::from_secs(0)));

  // let cloned_renderer = Rc::clone(&renderer);
  // let cloned_camera = Rc::clone(&camera);
  // let keydown = Closure::<dyn FnMut(_)>::new(move |ev: KeyboardEvent| {
  //   match ev.key().as_str() {
  //     "w" => {
  //       *cloned_perspective_camera.borrow_mut().position_mut() += (0.0, 0.0, -0.5).into();
  //     },
  //     "a" => {
  //       *cloned_perspective_camera.borrow_mut().position_mut() += (-0.5, 0.0, 0.0).into();
  //     },
  //     "s" => {
  //       *cloned_perspective_camera.borrow_mut().position_mut() += (0.0, 0.0, 0.5).into();
  //     },
  //     "d" => {
  //       *cloned_perspective_camera.borrow_mut().position_mut() += (0.5, 0.0, 0.0).into();
  //     },
  //     _ => {}
  //   }

  //   let renderer = &cloned_renderer.borrow();
  //   let perspective_camera: &PerspectiveCamera = &cloned_perspective_camera.borrow();

  //   cloned_camera.update(renderer, perspective_camera);
  // });

  // let cloned_renderer = Rc::clone(&renderer);
  // let cloned_camera = Rc::clone(&camera);
  // let cloned_perspective_camera = Rc::clone(&perspective_camera);

  // let delta_clone = Rc::clone(&render_delta);
  // let mousemove = Closure::<dyn FnMut(_)>::new(move |ev: MouseEvent| {
  //   let delta_time = delta_clone.borrow().as_secs_f32();

  //   if ev.movement_x() > 0 {
  //     *cloned_perspective_camera.borrow_mut().yaw_mut() -= Rad(20.0) * delta_time;
  //   } else if ev.movement_x() < 1 {
  //     *cloned_perspective_camera.borrow_mut().yaw_mut() += Rad(20.0) * delta_time;
  //   }

  //   if ev.movement_y() > 0 {
  //     *cloned_perspective_camera.borrow_mut().pitch_mut() -= Rad(20.0) * delta_time;
  //   } else if ev.movement_y() < 1 {
  //     *cloned_perspective_camera.borrow_mut().pitch_mut() += Rad(20.0) * delta_time;
  //   }

  //   use std::f32::consts::FRAC_PI_2;
  //   const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

  //   if *cloned_perspective_camera.borrow().pitch() < -Rad(SAFE_FRAC_PI_2) {
  //     *cloned_perspective_camera.borrow_mut().pitch_mut() = -Rad(SAFE_FRAC_PI_2);
  //   } else if *cloned_perspective_camera.borrow().pitch() > Rad(SAFE_FRAC_PI_2) {
  //     *cloned_perspective_camera.borrow_mut().pitch_mut() = Rad(SAFE_FRAC_PI_2);
  //   }

  //   let renderer = &cloned_renderer.borrow();
  //   let perspective_camera: &PerspectiveCamera = &cloned_perspective_camera.borrow();

  //   cloned_camera.update(renderer, perspective_camera);
  // });

  let focus_canvas = Closure::<dyn FnMut(_)>::new(move |ev: MouseEvent| {
    let Some(target) = ev.target() else {
      return;
    };

    let Some(target_el) = target.dyn_ref::<HtmlCanvasElement>() else {
      return;
    };

    target_el.request_pointer_lock();
  });

  _ = canvas.add_event_listener_with_callback("click", focus_canvas.as_ref().unchecked_ref());
  // _ = document.add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref());
  // _ = document.add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref());

  // keydown.forget();
  // mousemove.forget();
  focus_canvas.forget();
  
  animation_loop(move || {
    let now = Instant::now();
    let delta = now - *render_time.borrow();

    *render_delta.borrow_mut() = delta;
    *render_time.borrow_mut() = now;
    
    {
      let pcamera: &PerspectiveCamera = &perspective_camera.borrow();
      let prev_pitch: Deg<f32> = (*pcamera.pitch()).into();

      *perspective_camera.borrow_mut().pitch_mut() = ((prev_pitch + Deg(1.0)) % Deg(360.0)).into();
      camera.update(&Rc::clone(&renderer).borrow(), pcamera);
    }

    Rc::clone(&renderer).borrow().render(&scene, &*camera);
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
