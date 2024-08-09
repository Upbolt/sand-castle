struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
};

@group(2) @binding(0)
var<uniform> material_color: vec4<f32>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  return material_color;
}
