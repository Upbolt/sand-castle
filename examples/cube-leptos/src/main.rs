use leptos::*;

use leptos_sand_castle::components::{canvas::Canvas, cuboid::Cuboid, mesh::Mesh};

fn main() {
  console_error_panic_hook::set_once();

  mount_to_body(|| {
    view! {
      <div style="display: flex; height: 100vh; justify-content: center; align-items: center">
        <Canvas
          attr:width="500"
          attr:height="281"
          attr:style="overflow: hidden; border-radius: 8px; filter: drop-shadow(0px 5px 10px #00000044);"
        >
          <Mesh>
            <Cuboid />
          </Mesh>
        </Canvas>
      </div>
    }
  });
}
