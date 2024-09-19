struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) normal: vec3<f32>,
  @location(1) world_position: vec3<f32>,
  @location(2) tex_coords: vec2<f32>,
  @location(3) camera_pos: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(1) @binding(0)
var<uniform> transformation: mat4x4<f32>;

@group(3) @binding(0)
var<uniform> matrix_normal: mat4x4<f32>;

struct Camera {
  view_matrix: mat4x4<f32>,
  position: vec3<f32>,
  pad0: f32,
}

@vertex
fn vs_main(
  @location(0) position: vec3<f32>,
  @location(1) normal: vec3<f32>,
  @location(2) tex_coords: vec2<f32>,
) -> VertexOutput {
  var out: VertexOutput;

  let view_matrix_normal = mat3x3<f32>(matrix_normal[0].xyz, matrix_normal[1].xyz, matrix_normal[2].xyz);
  let world_position: vec4<f32> = transformation * vec4<f32>(position, 1.0);

  out.world_position = world_position.xyz;
  out.normal = view_matrix_normal * normal;
  out.clip_position = camera.view_matrix * world_position;
  out.tex_coords = tex_coords;
  out.camera_pos = camera.position;

  return out;
}
