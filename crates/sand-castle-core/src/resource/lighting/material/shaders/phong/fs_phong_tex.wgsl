struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) normal: vec3<f32>,
  @location(1) world_position: vec3<f32>,
  @location(2) tex_coords: vec2<f32>,
};

@group(2) @binding(0)
var<uniform> material_color: vec4<f32>;

@group(4) @binding(0)
var<uniform> ambient_light: vec4<f32>;

@group(5) @binding(0)
var<uniform> directional_lights: array<DirectionalLight, 16>;
@group(5) @binding(1)
var<uniform> point_lights: array<PointLight, 16>;
@group(5) @binding(2)
var<uniform> spot_lights: array<SpotLight, 16>;
@group(5) @binding(3)
var<uniform> directional_light_count: LightCount;
@group(5) @binding(4)
var<uniform> point_light_count: LightCount;
@group(5) @binding(5)
var<uniform> spot_light_count: LightCount;

@group(6) @binding(0)
var diffuse_map: texture_2d<f32>;
@group(6) @binding(1)
var diffuse_sampler: sampler;

struct SpotLight {
  point_light: PointLight,
  direction: vec3<f32>,
  cutoff_angle: f32,
}

struct PointLight {
  pos: vec3<f32>,
  color: vec3<f32>,
}

struct DirectionalLight {
  direction: vec3<f32>,
  color: vec3<f32>,
}

struct LightCount {
  value: u32,
  padding0: u32,
  padding1: u32,
  padding2: u32,
}

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
  var light_influence = ambient_light;

  for (var index: u32 = 0; index < point_light_count.value; index++) {
    light_influence += emission_from_point_light(point_lights[index], vertex);
  }

  for (var index: u32 = 0; index < spot_light_count.value; index++) {
    light_influence += emission_from_spot_light(spot_lights[index], vertex);
  }

  return light_influence * textureSample(diffuse_map, diffuse_sampler, vertex.tex_coords);
}

const attenuation_const: f32 = 1.0;
const attenuation_linear: f32 = 0.045;
const attenuation_quad: f32 = 0.0075;

fn emission_from_point_light(
  light: PointLight,
  vertex: VertexOutput,
) -> vec4<f32> {
  let light_to_pixel = normalize(light.pos - vertex.world_position);
  let light_distance = length(light_to_pixel);
  let light_direction = light_to_pixel / light_distance;

  let attenuation = 1.0 / (attenuation_const + attenuation_linear * light_distance + attenuation_quad * (light_distance * light_distance));
  let diffuse = light.color * attenuation * max(dot(vertex.normal, light_direction), 0.0);

  return vec4<f32>(diffuse, 1.0);
}

fn emission_from_spot_light(
  light: SpotLight,
  vertex: VertexOutput,
) -> vec4<f32> {
  let light_to_pixel = normalize(light.point_light.pos - vertex.world_position);
  let spot_factor = dot(light_to_pixel, light.direction);

  if spot_factor > light.cutoff_angle {
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
  }

  let color = emission_from_point_light(light.point_light, vertex);
  let intensity = (1.0 - (1.0 - spot_factor) / (1.0 - light.cutoff_angle));

  return color * intensity;
}

fn emission_from_directional_light(
  light: DirectionalLight,
  vertex: VertexOutput,
) -> vec4<f32> {
  return vec4<f32>(light.color, 1.0);
}
