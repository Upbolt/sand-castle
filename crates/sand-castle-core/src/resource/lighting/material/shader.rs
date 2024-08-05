use std::borrow::Cow;

pub struct ShaderMaterial {
  vertex_shader: Cow<'static, str>,
  fragment_shader: Cow<'static, str>,
}
