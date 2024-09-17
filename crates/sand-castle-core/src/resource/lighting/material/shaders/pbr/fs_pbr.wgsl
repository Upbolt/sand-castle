struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) normal: vec3<f32>,
  @location(1) world_position: vec3<f32>,
  @location(3) camera_pos: vec3<f32>,
};

@group(2) @binding(0)
var<uniform> material: Material;

struct Material {
  color: vec4<f32>,
  roughness: f32,
  metalness: f32,
  pad0: f32,
  pad1: f32,
}

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

  light_influence = light_influence / (light_influence + vec4<f32>(1.0, 1.0, 1.0, 0.0));

  let gamma_corrected = vec4<f32>(pow(light_influence.xyz, vec3<f32>(1.0 / 2.2, 1.0 / 2.2, 1.0 / 2.2)), 1.0);

  return gamma_corrected * material.color;
}

const PI = radians(180.0);

const attenuation_const: f32 = 1.0;
const attenuation_linear: f32 = 0.045;
const attenuation_quad: f32 = 0.0075;

fn emission_from_point_light(
  light: PointLight,
  vertex: VertexOutput,
) -> vec4<f32> {
  var light_to_pixel = light.pos - vertex.world_position;
  let light_distance = length(light_to_pixel);

  light_to_pixel = normalize(light_to_pixel);

  var light_intensity = light.color;
  light_intensity = light_intensity / (light_distance * light_distance);

  // let attenuation = 1.0 / (attenuation_const + attenuation_linear * light_distance + attenuation_quad * (light_distance * light_distance));

  let view_normal = normalize(vertex.camera_pos - vertex.world_position);
  let half_vector = normalize(view_normal + light_to_pixel);

  let f = schlick(max(dot(view_normal, half_vector), 0.0));

  let ks = f;
  let kd = 1.0 - ks;

  let light_dot = max(dot(vertex.normal, light_to_pixel), 0.0);
  let view_dot = max(dot(vertex.normal, view_normal), 0.0);

  let specular_brdf_nom = ggx(max(dot(vertex.normal, half_vector), 0.0))
    * f
    * geom_smith(light_dot)
    * geom_smith(view_dot);

  let specular_brdf = specular_brdf_nom / ( 4.0 * view_dot * light_dot + 0.0001 );

  let lambert = mix(material.color.xyz, vec3<f32>(0.0, 0.0, 0.0), material.metalness);
  let specular_color = mix(vec3<f32>(1.0, 1.0, 1.0), material.color.xyz, material.metalness);

  let diffuse_brdf = kd * lambert / PI;
  // let diffuse = (diffuse_brdf + (specular_brdf * specular_color)) * attenuation * light_dot;
  let diffuse = (diffuse_brdf + (specular_brdf * specular_color)) * light_intensity * light_dot;

  return vec4<f32>(diffuse, 1.0);
}

fn ggx(n_dot_h: f32) -> f32 {
  let alpha2 = material.roughness * material.roughness * material.roughness * material.roughness;
  let d = n_dot_h * n_dot_h * (alpha2 - 1) + 1;

  return alpha2 / (PI * d * d);
}

fn geom_smith(dp: f32) -> f32 {
  let k = (material.roughness + 1.0) * (material.roughness * 1.0) / 8.0;
  let denom = dp * (1 - k) + k;

  return dp / denom;
}

fn schlick(v_dot_h: f32) -> vec3<f32> {
  let dielectric_f0 = vec3<f32>(0.04, 0.04, 0.04);

  let f0 = mix(dielectric_f0, material.color.xyz, material.metalness);

  return f0 + (1 - f0) * pow(clamp(1.0 - v_dot_h, 0.0, 1.0), 5.0);
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
