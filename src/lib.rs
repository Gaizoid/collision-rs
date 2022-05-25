#[derive(Debug)]
pub struct Circle {
  pos: (f32, f32),
  r: f32,
}

impl Circle {
  pub fn new(pos: (f32, f32), r: f32) -> Circle {
    Circle {
      pos,
      r
    }
  }
}

fn fast_dist(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
  (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)
}

#[allow(dead_code)]
fn smooth_step(edge0: f32, edge1: f32, x: f32) -> f32 {
  if x < edge0 { return 0f32; }
  if x > edge1 { return 1f32; }

  let x = (x - edge0) / (edge1 - edge0);

  x * x * (3.0 - 2.0 * x)
}

fn check_col_c_c(c1: &Circle, c2: &Circle) -> bool {
  fast_dist(c1.pos.0, c1.pos.1, c2.pos.0, c2.pos.1) < (c1.r + c2.r) * (c1.r + c2.r)
}

pub fn check_collisions_circle(circles: Vec<Circle>) -> Vec<(usize, usize)> {
  let mut collisions: Vec<(usize, usize)> = Vec::new();
  for (i, c1) in circles.iter().enumerate() {
    for (j, c2) in circles.iter().enumerate() {
      if i == j { continue; }
      if check_col_c_c(c1, c2) && i < j {
        collisions.push((i, j))
      }
    }
  }
  collisions
}