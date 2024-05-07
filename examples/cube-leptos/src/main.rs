use leptos::*;

use leptos_sand_castle::components::canvas::Canvas;

fn main() {
  console_error_panic_hook::set_once();

  mount_to_body(|| {
    view! {
      <Canvas>
        // <span>"hi"</span>
      </Canvas>
    }
  });
}
