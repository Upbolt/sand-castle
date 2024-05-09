pub mod cube;
pub mod torus;

use crate::units::Vector3;

pub trait Geometry {
  // fn attributes(&self);
  // fn bounding_box(&self);
  // fn bounding_sphere(&self);
  // fn draw_range(&self);
  // fn groups(&self);
  // fn id(&self);
  // fn index(&self);
  // fn is_buffer_geometry(&self);
  // fn morph_attributes(&self);
  // fn morph_targets_relative(&self);
  // fn name(&self);
  // fn user_data(&self);
  // fn uuid(&self);

  // fn get_attribute(&self, name: &str);
  // fn has_attribute(&self, name: &str);

  // fn add_group(&mut self);
  // fn apply_matrix4(&mut self);
  // fn apply_quaternion(&mut self);
  // fn center(&mut self);
  // fn clear_groups(&mut self);
  // fn compute_bounding_box(&mut self);
  // fn compute_bounding_sphere(&mut self);
  // fn compute_tangents(&mut self);
  // fn compute_vertex_normals(&mut self);
  // fn remove_attribute(&mut self, name: &str);
  // fn look_at(&mut self, position: Vector3);

  // fn normalize_normals(&mut self);
  // fn rotate_x(&mut self, radians: f64);
  // fn rotate_y(&mut self, radians: f64);
  // fn rotate_z(&mut self, radians: f64);
  // fn scale(&mut self, x: f64, y: f64, z: f64);

  // fn set_attribute(&mut self, name: &str);
}
