extern crate leptos;
extern crate leptos_use;
extern crate sand_castle_leptos;

use leptos::{
  ev::{
    beforeunload, click, keydown, keyup, mousedown, mousemove, mouseup, BeforeUnloadEvent,
    KeyboardEvent, MouseEvent,
  },
  html::Canvas,
  *,
};

use sand_castle_leptos::{
  canvas::{Backend, Canvas},
  resource::{
    camera::{orthographic::OrthographicCamera, perspective::PerspectiveCamera},
    geometry::cuboid::Cuboid,
    lighting::material::basic::BasicMaterial,
    mesh::Mesh,
  },
  scene::Scene,
  Quat, Vec3, Vec4,
};

use wasm_bindgen::prelude::*;

use leptos_use::{use_document, use_event_listener, use_window};

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
  let yaw = RwSignal::new(-89.555);
  let pitch = RwSignal::new(-19.25);

  let camera_pos = RwSignal::new(Vec3::new(0.0, 1.5, 4.0));

  let up = RwSignal::new(false);
  let down = RwSignal::new(false);
  let forward = RwSignal::new(false);
  let backward = RwSignal::new(false);
  let left = RwSignal::new(false);
  let right = RwSignal::new(false);

  let cube_rotation = RwSignal::new(Quat::default());

  let canvas = NodeRef::<Canvas>::new();

  let dragging = RwSignal::new(false);
  let coords = RwSignal::new((0, 0));

  use_event_listener(use_window(), beforeunload, move |ev: BeforeUnloadEvent| {
    // ev.prevent_default();
    // ev.set_return_value("what the fart");
  });

  use_event_listener(use_document(), mousemove, move |ev: MouseEvent| {
    if dragging.get_untracked() {
      coords.set((ev.client_x(), ev.client_y()));
    }
  });

  use_event_listener(canvas, mousedown, move |ev: MouseEvent| {
    dragging.set(true);
  });

  use_event_listener(use_document(), mouseup, move |ev: MouseEvent| {
    dragging.set(false);
  });

  use_event_listener(use_document(), keydown, move |ev: KeyboardEvent| {
    match ev.key().as_str() {
      " " => up.set(true),
      "Shift" => down.set(true),
      "w" => forward.set(true),
      "a" => left.set(true),
      "s" => backward.set(true),
      "d" => right.set(true),
      _ => {}
    }
  });

  use_event_listener(use_document(), keyup, move |ev: KeyboardEvent| {
    match ev.key().as_str() {
      " " => up.set(false),
      "Shift" => down.set(false),
      "w" => forward.set(false),
      "a" => left.set(false),
      "s" => backward.set(false),
      "d" => right.set(false),
      _ => {}
    }
  });

  Effect::new(move |_| {
    if up.get() {
      camera_pos.update(|pos| {
        if !down.get_untracked() {
          pos.y += 0.1;
        }

        if forward.get_untracked() {
          pos.z -= 0.1;
        } else if backward.get_untracked() {
          pos.z += 0.1;
        }

        if left.get_untracked() {
          pos.x -= 0.1;
        } else if right.get_untracked() {
          pos.x += 0.1;
        }
      });
    }
  });

  Effect::new(move |_| {
    if down.get() {
      camera_pos.update(|pos| {
        if !up.get_untracked() {
          pos.y -= 0.1;
        }

        if forward.get_untracked() {
          pos.z -= 0.1;
        } else if backward.get_untracked() {
          pos.z += 0.1;
        }

        if left.get_untracked() {
          pos.x -= 0.1;
        } else if right.get_untracked() {
          pos.x += 0.1;
        }
      });
    }
  });

  Effect::new(move |_| {
    if forward.get() {
      camera_pos.update(|pos| {
        if !backward.get_untracked() {
          pos.z -= 0.1;
        }

        if left.get_untracked() {
          pos.x -= 0.1;
        } else if right.get_untracked() {
          pos.x += 0.1;
        }

        if down.get_untracked() {
          pos.y -= 0.1;
        } else if up.get_untracked() {
          pos.y += 0.1;
        }
      });
    }
  });

  Effect::new(move |_| {
    if backward.get() {
      camera_pos.update(|pos| {
        if !forward.get_untracked() {
          pos.z += 0.1;
        }

        if left.get_untracked() {
          pos.x -= 0.1;
        } else if right.get_untracked() {
          pos.x += 0.1;
        }

        if down.get_untracked() {
          pos.y -= 0.1;
        } else if up.get_untracked() {
          pos.y += 0.1;
        }
      });
    }
  });

  Effect::new(move |_| {
    if left.get() {
      camera_pos.update(|pos| {
        if !right.get_untracked() {
          pos.x -= 0.1;
        }

        if forward.get_untracked() {
          pos.z -= 0.1;
        } else if backward.get_untracked() {
          pos.z += 0.1;
        }

        if down.get_untracked() {
          pos.y -= 0.1;
        } else if up.get_untracked() {
          pos.y += 0.1;
        }
      });
    }
  });

  Effect::new(move |_| {
    if right.get() {
      camera_pos.update(|pos| {
        if !left.get_untracked() {
          pos.x += 0.1;
        }

        if forward.get_untracked() {
          pos.z -= 0.1;
        } else if backward.get_untracked() {
          pos.z += 0.1;
        }

        if down.get_untracked() {
          pos.y -= 0.1;
        } else if up.get_untracked() {
          pos.y += 0.1;
        }
      });
    }
  });

  Effect::new(move |old_coords| {
    let (old_x, old_y) = old_coords.unwrap_or((0, 0));

    let coords = coords.get();
    let (x, y) = &coords;

    if old_x != *x {
      if old_x > *x {
        yaw.update(|yaw| {
          *yaw -= 0.025;
        });
      } else {
        yaw.update(|yaw| {
          *yaw += 0.025;
        });
      }
    }

    if old_y != *y {
      if old_y > *y {
        pitch.update(|pitch| {
          *pitch += 0.025;
        });
      } else {
        pitch.update(|pitch| {
          *pitch -= 0.025;
        });
      }
    }

    return coords;
  });

  view! {
      <h1 attr:style="margin-bottom: 0.125rem">"sand-castle leptos"</h1>

      <Canvas
        attr:style="display: block"
        node_ref=canvas
      >
        <Scene color=Vec4::new(0.1, 0.1, 0.1, 1.0)>
          <PerspectiveCamera
            aspect_ratio=300.0/150.0
            position=camera_pos
            yaw=yaw
            pitch=pitch
          />

          <Mesh
            rotation=cube_rotation
          >
            <Cuboid />
            <BasicMaterial />
          </Mesh>
        </Scene>
      </Canvas
  >
      <div>
        <h2 style="margin-bottom: 0.25rem">"camera"</h2>

        <div>
          <button on:click=move |_| yaw.update(|yaw| { *yaw += 0.1; })>"+"</button>
          <button on:click=move |_| yaw.update(|yaw| { *yaw -= 0.1; })>"-"</button>
          <span>{move || format!("yaw: {}", yaw.get())}</span>
        </div>

        <div>
          <button on:click=move |_| pitch.update(|pitch| { *pitch += 0.1; })>"+"</button>
          <button on:click=move |_| pitch.update(|pitch| { *pitch -= 0.1; })>"-"</button>
          <span>{move || format!("pitch: {}", pitch.get())}</span>
        </div>
      </div>
    }
}
