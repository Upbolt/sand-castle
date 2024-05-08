use leptos::*;

use leptos_sand_castle::components::canvas::Canvas;

fn main() {
  console_error_panic_hook::set_once();

  mount_to_body(|| {
    view! {
      <Canvas
        attr:width="500"
        attr:height="281"
      >
        // <span>"hi"</span>
      </Canvas>
    }
  });
}
