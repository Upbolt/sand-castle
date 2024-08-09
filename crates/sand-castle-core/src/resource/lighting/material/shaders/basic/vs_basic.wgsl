struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: mat4x4<f32>;

@group(1) @binding(0)
var<uniform> transformation: mat4x4<f32>;

@vertex
fn vs_main(
  @location(0) position: vec3<f32>,
) -> VertexOutput {
  var out: VertexOutput;

  out.clip_position = camera * transformation * vec4<f32>(position, 1.0);

  return out;
}
