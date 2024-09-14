struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) normal: vec3<f32>,
  @location(1) world_position: vec3<f32>,
  @location(2) tex_coords: vec2<f32>,
};

@group(6) @binding(0)
var diffuse_map: texture_2d<f32>;
@group(6) @binding(1)
var diffuse_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  return textureSample(diffuse_map, diffuse_sampler, in.tex_coords);
}
