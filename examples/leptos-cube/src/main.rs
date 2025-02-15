extern crate leptos;
extern crate leptos_use;
extern crate sand_castle_leptos;

use leptos::prelude::*;

use sand_castle_leptos::{
  canvas::Canvas,
  resource::{
    camera::perspective::PerspectiveCamera,
    geometry::{cuboid::Cuboid, Geometry},
    lighting::{
      light::{ambient_light::AmbientLight, point_light::PointLight},
      material::phong::PhongMaterial,
    },
    mesh::Mesh,
  },
  scene::Scene,
  Quat, Vec3, Vec4,
};

use leptos_use::use_interval_fn;

fn main() {
  console_error_panic_hook::set_once();

  mount_to_body(|| {
    view! {
      <App />
    }
  });
}

#[component]
fn App() -> impl IntoView {
  let camera_pos = RwSignal::new(Vec3::new(0.0, 2.5, 5.0));

  let yaw = RwSignal::new(270.0f32.to_radians());
  let pitch = RwSignal::new(-25.0f32.to_radians());

  let cube_rot_angle = RwSignal::new(0.0f32);
  let cube_rot =
    Signal::derive(move || Quat::from_axis_angle(Vec3::Y, cube_rot_angle.get().to_radians()));

  use_interval_fn(
    move || {
      cube_rot_angle.update(|angle| {
        *angle = (*angle + 0.25) % 360.0;
      });
    },
    1,
  );

  view! {
    <h1 style="margin-bottom: 0.125rem">"sand-castle leptos"</h1>

    <Canvas
      attr:style="display: block"
      attr:width=1080
      attr:height=720
    >
      <Scene color=Vec4::new(0.1, 0.1, 0.1, 0.0)>
        <AmbientLight color=Vec3::new(0.1, 0.1, 0.1)/>
        <PointLight position=Vec3::new(10.0, 10.0, 10.0)/>

        <PerspectiveCamera
          aspect_ratio=1080.0/720.0
          position=camera_pos
          yaw=yaw
          pitch=pitch
        />

        <Mesh
          rotation=cube_rot
          position=Vec3::new(0.0, 0.0, 0.0)
        >
          <Cuboid />
          <PhongMaterial color=Vec4::new(0.5, 0.5, 0.5, 1.0)/>
        </Mesh>
      </Scene>
    </Canvas>

    <div>
      <h2 style="margin-bottom: 0.25rem">"camera"</h2>

      <div>
        <span>{move || format!("yaw: {}°", yaw.get().to_degrees())}</span>
      </div>

      <div>
        <span>{move || format!("pitch: {}°", pitch.get().to_degrees())}</span>
      </div>

      <div>
        <span>{move || format!("position: {:?}", camera_pos.get())}</span>
      </div>
    </div>

    <div>
      <h2 style="margin-bottom: 0.25rem">"cube"</h2>

      <div>
        <span>{move || format!("rotation (y): {}°", cube_rot_angle.get())}</span>
      </div>
    </div>
  }
}
