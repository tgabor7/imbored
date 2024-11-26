use std::arch::x86_64;

use bevy::{math::Vec2, prelude::Transform};

use crate::components::velocity::Velocity;

pub fn check_collision(a: &Transform, b: &Transform) -> bool {

  let left_a = a.translation.x - a.scale.x * 0.5;
  let left_b = b.translation.x - b.scale.x * 0.5;
  let right_a = a.translation.x + a.scale.x * 0.5;
  let right_b = b.translation.x + b.scale.x * 0.5;
  let top_a = a.translation.y + a.scale.y * 0.5;
  let top_b = b.translation.y + b.scale.y * 0.5;
  let bottom_a = a.translation.y - a.scale.y * 0.5;
  let bottom_b = b.translation.y - b.scale.y * 0.5;

  let collision = left_a < right_b && right_a > left_b && top_a > bottom_b && bottom_a < top_b;
  
  collision
}

pub fn check_collision_with_velocity(
  a: &Transform,
  b: &Transform,
  a_velocity: &Velocity,
  b_velocity: &Velocity,
  delta: f32) -> bool {

  let left_a_next = a.translation.x + a_velocity.x * delta - a.scale.x * 0.5;
  let left_b_next = b.translation.x + b_velocity.x * delta - b.scale.x * 0.5;
  let right_a_next = a.translation.x + a_velocity.x * delta + a.scale.x * 0.5;
  let right_b_next = b.translation.x + b_velocity.x * delta + b.scale.x * 0.5;
  let top_a_next = a.translation.y + a_velocity.y * delta + a.scale.y * 0.5;
  let top_b_next = b.translation.y + b_velocity.y * delta + b.scale.y * 0.5;
  let bottom_a_next = a.translation.y + a_velocity.y * delta - a.scale.y * 0.5;
  let bottom_b_next = b.translation.y + b_velocity.y * delta - b.scale.y * 0.5;

  let collision = left_a_next < right_b_next && right_a_next > left_b_next && top_a_next > bottom_b_next && bottom_a_next < top_b_next;

  collision
}

pub fn check_swept_aabb_collision(
  a: &Transform,
  b: &Transform,
  a_velocity: &Velocity,
  b_velocity: &Velocity,
  delta: f32) -> (bool, f32, f32, f32, f32) {

    let x_inv_entry;
    let y_inv_entry;
    let x_inv_exit;
    let y_inv_exit;

    if a_velocity.x > 0.0 {
      x_inv_entry = b.translation.x - b.scale.x * 0.5 - (a.translation.x + a.scale.x * 0.5);
      x_inv_exit = b.translation.x + b.scale.x * 0.5 - (a.translation.x - a.scale.x * 0.5);
    } else {
      x_inv_entry = b.translation.x + b.scale.x * 0.5 - (a.translation.x - a.scale.x * 0.5);
      x_inv_exit = b.translation.x - b.scale.x * 0.5 - (a.translation.x + a.scale.x * 0.5);
    }

    if a_velocity.y > 0.0 {
      y_inv_entry = b.translation.y - b.scale.y * 0.5 - (a.translation.y + a.scale.y * 0.5);
      y_inv_exit = b.translation.y + b.scale.y * 0.5 - (a.translation.y - a.scale.y * 0.5);
    } else {
      y_inv_entry = b.translation.y + b.scale.y * 0.5 - (a.translation.y - a.scale.y * 0.5);
      y_inv_exit = b.translation.y - b.scale.y * 0.5 - (a.translation.y + a.scale.y * 0.5);
    }

    let x_entry;
    let y_entry;
    let x_exit;
    let y_exit;

    if a_velocity.x == 0.0 {
      x_entry = std::f32::NEG_INFINITY;
      x_exit = std::f32::INFINITY;
    } else {
      x_entry = x_inv_entry / a_velocity.x;
      x_exit = x_inv_exit / a_velocity.x;
    }

    if a_velocity.y == 0.0 {
      y_entry = std::f32::NEG_INFINITY;
      y_exit = std::f32::INFINITY;
    } else {
      y_entry = y_inv_entry / a_velocity.y;
      y_exit = y_inv_exit / a_velocity.y;
    }

    let entry_time = x_entry.max(y_entry);
    let exit_time = x_exit.min(y_exit);

    if entry_time > exit_time || x_entry < 0.0 && y_entry < 0.0 || x_entry > 1.0 || y_entry > 1.0 {
      return (false, 0.0, 0.0, 0.0, 0.0);
    }else {
      let normalx;
      let normaly;

      if x_entry > y_entry {
        if x_inv_entry < 0.0 {
          normalx = 1.0;
          normaly = 0.0;
        } else {
          normalx = -1.0;
          normaly = 0.0;
        }
      } else {
        if y_inv_entry < 0.0 {
          normalx = 0.0;
          normaly = 1.0;
        } else {
          normalx = 0.0;
          normaly = -1.0;
        }
      }

      return (true, entry_time, exit_time, normalx, normaly);
    }
}

pub fn get_ray_intersection_fraction_first_ray(
  origin_a: &Vec2,
  end_a: &Vec2,
  origin_b: &Vec2,
  end_b: &Vec2) -> f32 {

    let delta_a = Vec2::new(end_a.x - origin_a.x, end_a.y - origin_a.y);
    let delta_b = Vec2::new(end_b.x - origin_b.x, end_b.y - origin_b.y);

    let determinant = delta_a.x * delta_b.y - delta_a.y * delta_b.x;

    if determinant == 0.0 {
      return 0.0;
    }

    let delta_c = Vec2::new(origin_b.x - origin_a.x, origin_b.y - origin_a.y);
    let t = (delta_c.x * delta_b.y - delta_c.y * delta_b.x) / determinant;

    if t < 0.0 || t > 1.0 {
      return 0.0;
    }

    let u = (delta_c.x * delta_a.y - delta_c.y * delta_a.x) / determinant;

    if u < 0.0 || u > 1.0 {
      return 0.0;
    }

    t
  }

pub fn get_ray_intersection_fraction(
  origin: &Vec2,
  direction: &Vec2
) -> f32 {

  let mut t = 0.0;

  let mut current = origin.clone();

  while current.x >= 0.0 && current.x <= 1.0 && current.y >= 0.0 && current.y <= 1.0 {
    current.x = current.x + direction.x;
    current.y = current.y + direction.y;
    t += 1.0;
  }

  t
}

pub fn minkowski_difference(
  a: &Transform,
  b: &Transform
) -> Transform {

  let mut result = Transform::default();

  result.translation.x = a.translation.x - b.translation.x;
  result.translation.y = a.translation.y - b.translation.y;
  result.scale.x = a.scale.x + b.scale.x;
  result.scale.y = a.scale.y + b.scale.y;

  result
}

pub fn closest_point_on_bounds_to_point(
  bounds: &Transform,
  point: &Vec2
) -> Vec2 {

  let mut closest = Vec2::default();

  let half_width = bounds.scale.x * 0.5;
  let half_height = bounds.scale.y * 0.5;

  let dx = point.x - bounds.translation.x;
  let dy = point.y - bounds.translation.y;

  let x = dx.max(-half_width).min(half_width);
  let y = dy.max(-half_height).min(half_height);

  closest.x = bounds.translation.x + x;
  closest.y = bounds.translation.y + y;

  closest
}
