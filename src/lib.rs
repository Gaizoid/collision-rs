use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Circle {
  pub pos: (f32, f32),
  pub r: f32,
  pub color: (u8, u8, u8),
}

impl Circle {
  pub fn new(pos: (f32, f32), r: f32, color: (u8, u8, u8)) -> Circle {
    Circle {
      pos,
      r,
      color,
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

  pow(x, 2.0) * (3.0 - 2.0 * x)
}

fn check_col_c_c(c1: &Circle, c2: &Circle) -> bool {
  fast_dist(
      c1.pos.0, 
      c1.pos.1, 
      c2.pos.0, 
      c2.pos.1
    ) < pow(c1.r + c2.r, 2.0)
}

pub fn check_collisions_circle(circles: Vec<Circle>) -> Vec<(usize, usize)> {
  let mut collisions: Vec<(usize, usize)> = Vec::new();
  for (i, c1) in circles.iter().enumerate() {
    for (j, c2) in circles[i..circles.len()].iter().enumerate() {
      if i == j { continue; }
      if check_col_c_c(c1, c2) && i < j {
        collisions.push((i, j))
      }
    }
  }
  collisions
}

/// ### Fast rand Randomly generates a value between 0 and 1 where [0, 1)
pub fn fast_rand() -> f32 {
    let sqrt2: f32 = 1.41421356237; 
    let time = SystemTime::now();

    let since_the_epoch = time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let time = since_the_epoch.subsec_nanos() as u64;
    
    let rand = (time as f64)/(sqrt2 as f64);
    let int_portion = rand.floor();

    return (rand-int_portion) as f32;
}

pub fn fast_rand_range(min: i32, max: i32) -> i32 {
    let rand = fast_rand();
    let integer_portion = (((max-min) as f32)*rand) as i32;
    integer_portion + min
}

fn pow(x: f32, y: f32) -> f32 {
  x.powf(y)
}