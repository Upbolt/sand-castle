extern crate leptos;
extern crate leptos_use;
extern crate reqwasm;
extern crate sand_castle_leptos;

use leptos::{
  ev::{
    beforeunload, click, keydown, keyup, mousedown, mousemove, mouseup, BeforeUnloadEvent, Event,
    KeyboardEvent, MouseEvent, ProgressEvent,
  },
  html::Canvas as HtmlCanvas,
  prelude::*,
};

use sand_castle_leptos::{
  canvas::{Backend, Canvas},
  resource::{
    camera::perspective::PerspectiveCamera,
    lighting::{
      light::{ambient_light::AmbientLight, point_light::PointLight},
      material::phong::PhongMaterial,
    },
    loader::{
      gltf::{use_gltf_loader_from_source, Gltf, LoadGltfError, LoadedGeometry, LoadedTransform},
      textures::TextureId,
    },
    mesh::Mesh,
    Id,
  },
  scene::Scene,
  Quat, Vec2, Vec3, Vec4,
};

use leptos_use::{use_document, use_event_listener, use_interval_fn, use_window};

use wasm_bindgen::{prelude::*, JsCast};

use js_sys::{ArrayBuffer, Uint8Array};
use web_sys::{FileReader, HtmlInputElement};

fn main() {
  console_error_panic_hook::set_once();

  mount_to_body(|| {
    view! {
      <App />
    }
  });
}

#[component]
fn GltfModel(#[prop(into)] source: MaybeProp<Vec<u8>>) -> impl IntoView {
  let model = use_gltf_loader_from_source(source);

  view! {
    <>
      {move || model
        .get()
        .map(|model: Result<Vec<(LoadedTransform, Id, Vec4, Option<TextureId>)>, LoadGltfError>| {
          let geometry_wrapper = std::iter::once(model);

          view! {
            <ErrorBoundary fallback=move |_| view! { <p>"failed to load model"</p> }.into_any()>
              {move || geometry_wrapper.clone().next().unwrap().map(|geometries| {
                let geometries = geometries.into_iter().enumerate();

                view! {
                  <For
                    each=move || geometries.clone()
                    key=|(index, _)| *index
                    children=move |(_, (LoadedTransform { translation, rotation, .. }, geometry_id, color, texture_id))| {
                      view! {
                        <Mesh
                          geometry_id=geometry_id
                          position=translation
                          rotation=rotation
                        >
                          <PhongMaterial
                            color=color
                            diffuse_map_texture_id=texture_id
                          />
                        </Mesh>
                      }
                    }
                  />
                }
              })}
            </ErrorBoundary>
          }
        })}
    </>
  }
}

#[component]
fn App() -> impl IntoView {
  let yaw = RwSignal::new(230.0_f32.to_radians());
  let pitch = RwSignal::new(-12.0_f32.to_radians());
  let source = RwSignal::new(None);

  let camera_pos = RwSignal::new(Vec3::new(3.5, 3.5, 3.5));

  let up = RwSignal::new(false);
  let down = RwSignal::new(false);
  let forward = RwSignal::new(false);
  let backward = RwSignal::new(false);
  let left = RwSignal::new(false);
  let right = RwSignal::new(false);

  let dragging = RwSignal::new(false);
  let coords = RwSignal::new(((0, 0), (0, 0)));

  let canvas = NodeRef::<HtmlCanvas>::new();

  _ = use_event_listener(use_document(), mousemove, move |ev: MouseEvent| {
    if dragging.get_untracked() {
      coords.update(|(coords, old_coords)| {
        *old_coords = *coords;
        *coords = (ev.client_x(), ev.client_y());
      });
    }
  });

  _ = use_event_listener(canvas, mousedown, move |ev: MouseEvent| {
    ev.prevent_default();

    dragging.set(true);

    coords.update(|(coords, old_coords)| {
      let current_pos = (ev.client_x(), ev.client_y());
      *coords = current_pos;
      *old_coords = current_pos;
    });
  });

  _ = use_event_listener(use_document(), mouseup, move |ev: MouseEvent| {
    dragging.set(false);
  });

  _ = use_event_listener(use_document(), keydown, move |ev: KeyboardEvent| {
    let key = ev.key();
    let key = key.as_str();

    let Some(c) = key.chars().next() else {
      return;
    };

    if key.len() == 1 && c.is_alphanumeric() {
      match c.to_lowercase().next() {
        Some('w') => forward.set(true),
        Some('a') => left.set(true),
        Some('s') => backward.set(true),
        Some('d') => right.set(true),
        _ => {}
      }

      return;
    }

    if key == " " {
      ev.prevent_default();
    }

    match key {
      " " => up.set(true),
      "Shift" => down.set(true),
      _ => {}
    }
  });

  _ = use_event_listener(use_document(), keyup, move |ev: KeyboardEvent| {
    let key = ev.key();
    let key = key.as_str();

    let Some(c) = key.chars().next() else {
      return;
    };

    if key.len() == 1 && c.is_alphanumeric() {
      match c.to_lowercase().next() {
        Some('w') => forward.set(false),
        Some('a') => left.set(false),
        Some('s') => backward.set(false),
        Some('d') => right.set(false),
        _ => {}
      }

      return;
    }

    if key == " " {
      ev.prevent_default();
    }

    match key {
      " " => up.set(false),
      "Shift" => down.set(false),
      _ => {}
    }
  });

  use_interval_fn(
    move || {
      camera_pos.update(|pos| {
        let up = up.get_untracked();
        let down = down.get_untracked();
        let left = left.get_untracked();
        let right = right.get_untracked();
        let forth = forward.get_untracked();
        let back = backward.get_untracked();

        const MOVEMENT_SPEED: f32 = 0.05;

        let pitch = pitch.get_untracked();
        let yaw = yaw.get_untracked();

        let forward_dir = Vec3::new(
          yaw.cos() * pitch.cos(),
          pitch.sin(),
          yaw.sin() * pitch.cos(),
        )
        .normalize();

        let right_dir = Vec3::new(yaw.sin(), 0.0, -yaw.cos()).normalize();
        let up_dir = Vec3::Y;

        if up && !down {
          *pos += up_dir * MOVEMENT_SPEED;
        } else if down && !up {
          *pos -= up_dir * MOVEMENT_SPEED;
        }

        if left && !right {
          *pos += right_dir * MOVEMENT_SPEED;
        } else if right && !left {
          *pos -= right_dir * MOVEMENT_SPEED;
        }

        if forth && !back {
          *pos += forward_dir * MOVEMENT_SPEED;
        } else if back && !forth {
          *pos -= forward_dir * MOVEMENT_SPEED;
        }
      });

      coords.update(|(coords, old_coords)| {
        let ((x, y), (old_x, old_y)) = (&coords, &old_coords);

        const CAMERA_SENS: f32 = 0.2;

        let delta_x = *x as f32 - *old_x as f32;
        let delta_y = *old_y as f32 - *y as f32;

        yaw.update(|yaw| {
          let new_yaw = (*yaw).to_degrees() + delta_x * CAMERA_SENS;
          *yaw = new_yaw.rem_euclid(360.0).to_radians();
        });

        pitch.update(|pitch| {
          let new_pitch = (*pitch).to_degrees() + delta_y * CAMERA_SENS;
          *pitch = new_pitch.clamp(-89.9, 89.9).to_radians();
        });

        *old_coords = (*x, *y);
      });
    },
    1,
  );


  view! {
    <span>"(W, A, S, D, Space, Shift) to move around, Hold left click to pan camera"</span>

    <h1 style="margin-bottom: 0.125rem">"sand-castle leptos"</h1>

    <Canvas
      attr:style="display: block"
      attr:width=1080
      attr:height=720
      node_ref=canvas
    >
      <Scene color=Vec4::new(0.1, 0.1, 0.1, 1.0)>
        <AmbientLight color=Vec3::new(0.1, 0.1, 0.1)/>
        <PointLight position=Vec3::new(100.0, 100.0, 100.0)/>

        <PerspectiveCamera
          aspect_ratio=1080.0/720.0
          position=camera_pos
          yaw=yaw
          pitch=pitch
        />

        <GltfModel source=source />
      </Scene>
    </Canvas>

    <input
      type="file"
      accept=".glb,.gltf"
      on:change=move |ev: Event| {
        let Some(target) = ev.target() else {
          return;
        };

        let Some(target) = target.dyn_ref::<HtmlInputElement>() else {
          return;
        };

        let Some(item) = target.files().and_then(|files| files.item(0)) else {
          return;
        };

        let Ok(reader) = FileReader::new() else {
          return;
        };

        _ = reader.read_as_array_buffer(&item);

        let onload = Closure::<dyn FnMut(_)>::new(move |ev: ProgressEvent| {
          let Some(target) = ev.target() else {
            return;
          };

          let Some(reader) = target.dyn_ref::<FileReader>() else {
            return;
          };

          let Ok(result) = reader.result() else {
            return;
          };

          let Some(buffer) = result.dyn_ref::<ArrayBuffer>() else {
            return;
          };

          let bytes = Uint8Array::new(&buffer);

          source.set(Some(bytes.to_vec()));
        });

        reader.set_onload(Some(onload.as_ref().unchecked_ref()));

        onload.forget();
      }
    />

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
  }
}
