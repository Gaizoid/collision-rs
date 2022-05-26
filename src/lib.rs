use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Circle {
    pub pos: (f32, f32),
    pub r: f32,
    pub color: (u8, u8, u8),
}

pub struct SurfaceCoordinates {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub inverted: bool,
}

pub struct Surface {
    pub pos: SurfaceCoordinates,
    pub color: (u8, u8, u8),
}

impl Circle {
    pub fn new(pos: (f32, f32), r: f32, color: (u8, u8, u8)) -> Circle {
        Circle { pos, r, color }
    }
}

impl Surface {
    pub fn new(pos: SurfaceCoordinates, color: (u8, u8, u8)) -> Surface {
        Surface { pos, color }
    }
}

fn fast_dist(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)
}

pub fn smooth_step(edge0: f32, edge1: f32, x: f32) -> f32 {
    if x < edge0 {
        return 0f32;
    }
    if x > edge1 {
        return 1f32;
    }

    let x = (x - edge0) / (edge1 - edge0);

    x * x * (3.0 - 2.0 * x)
}

fn check_col_c_c(c1: &Circle, c2: &Circle) -> bool {
    fast_dist(c1.pos.0, c1.pos.1, c2.pos.0, c2.pos.1) < (c1.r + c2.r) * (c1.r + c2.r)
}

pub fn check_collisions_circle(circles: &Vec<Circle>) -> Vec<(usize, usize)> {
    let mut collisions: Vec<(usize, usize)> = Vec::new();
    for (i, c1) in circles.iter().enumerate() {
        for (j, c2) in circles.iter().enumerate() {
            if i < j && check_col_c_c(c1, c2) {
                collisions.push((i, j));
            }
        }
    }
    collisions
}

/// ### Checking a collision
pub fn check_collision_circle_surface(
    circles: &Vec<Circle>,
    surfaces: &Vec<Surface>,
) -> Vec<(usize, usize)> {
    todo!();
}

/// ### Fast rand Randomly
/// generates a value between 0 and 1 where $$v \in [0, 1)$$
pub fn fast_rand() -> f32 {
    let sqrt2: f32 = 1.41421356237;
    let time = SystemTime::now();

    let since_the_epoch = time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let time = since_the_epoch.subsec_nanos() as u64;

    let rand = (time as f64) / (sqrt2 as f64);
    let int_portion = rand.floor();

    return (rand - int_portion) as f32;
}

pub fn fast_rand_range(min: i32, max: i32) -> i32 {
    let rand = fast_rand();
    let integer_portion = (((max - min) as f32) * rand) as i32;
    integer_portion + min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fast_dist_test() {
        assert_eq!(fast_dist(0.0, 0.0, 0.0, 0.0), 0.0);
        assert_eq!(fast_dist(0.0, 0.0, 0.0, 1.0), 1.0);
        assert_eq!(fast_dist(0.0, 0.0, 1.0, 0.0), 1.0);
        assert_eq!(fast_dist(0.0, 0.0, 1.0, 1.0), 2.0);
        assert_eq!(fast_dist(0.0, 1.0, 0.0, 0.0), 1.0);
        assert_eq!(fast_dist(0.0, 1.0, 0.0, 1.0), 0.0);
        assert_eq!(fast_dist(0.0, 1.0, 1.0, 0.0), 2.0);
        assert_eq!(fast_dist(0.0, 1.0, 1.0, 1.0), 1.0);
        assert_eq!(fast_dist(1.0, 0.0, 0.0, 0.0), 1.0);
        assert_eq!(fast_dist(1.0, 0.0, 0.0, 1.0), 2.0);
        assert_eq!(fast_dist(1.0, 0.0, 1.0, 0.0), 0.0);
        assert_eq!(fast_dist(1.0, 0.0, 1.0, 1.0), 1.0);
        assert_eq!(fast_dist(1.0, 1.0, 0.0, 0.0), 2.0);
        assert_eq!(fast_dist(1.0, 1.0, 0.0, 1.0), 1.0);
        assert_eq!(fast_dist(1.0, 1.0, 1.0, 0.0), 1.0);
        assert_eq!(fast_dist(1.0, 1.0, 1.0, 1.0), 0.0);
    }

    #[test]
    fn check_collisions_circle_test() {
        // expected values and indices
        let ans: Vec<(usize, usize)> = vec![(5, 6), (6, 7), (6, 8), (7, 8), (7, 9), (8, 9)];

      let width = 1280 as f32;
        let height = 720 as f32;

        let max_radius = 200.0;
        let circles_count = 10;

        let mut circles: Vec<Circle> = Vec::new();
        for i in 0..circles_count {
            let i_float = i as f32;
            let circle_float = circles_count as f32;
            // position is from [0, 1)
            let position = i_float / circle_float;
            let inverse_position = 1.0 - position;

            // make sure circles are smaller than the next
            let radius = position * max_radius;

            circles.push(Circle::new(
                // Random position vector
                (
                    position * (width - 2.0 * radius) + radius,
                    height
                        * smooth_step(
                            max_radius,
                            height - max_radius,
                            position * (height - 2.0 * radius) + radius,
                        ),
                ),
                // Random radius
                radius,
                // Random color
                (
                    0u8,
                    (100.0 + position * 155.0) as u8,
                    (100.0 + inverse_position * 155.0) as u8,
                ),
            ));
        }

        // DEBUG OUTPUT
        let collided_circles = check_collisions_circle(&circles);

        assert_eq!(collided_circles, ans)
    }

    #[test]
    fn fast_rand_range_test() {
        let min = 0;
        let max = 10;
        for _ in 0..100 {
            let rand = fast_rand_range(min, max);
            assert!(rand >= min && rand < max);
        }

        let min = -10;
        let max = 20;
        for _ in 0..100 {
            let rand = fast_rand_range(min, max);
            assert!(rand >= min && rand < max);
        }

        let min = 50;
        let max = 51;
        for _ in 0..100 {
            let rand = fast_rand_range(min, max);
            assert!(rand >= min && rand < max);
        }
    }

    #[test]
    fn check_col_c_c_test() {
        let circle_xy: Vec<(i8, i8, i8, i8)> =
            vec![(10, 10, 20, 20), (5, 5, 2, 2), (-10, -10, -15, -15)];
        let circle_r: Vec<(i8, i8)> = vec![(5, 5), (4, 2), (30, 5)];
        let should_collide: Vec<bool> = vec![false, true, true];

        assert!(circle_r.len() == circle_xy.len() && circle_r.len() == should_collide.len());

        for (i, (x1, y1, x2, y2)) in circle_xy.iter().enumerate() {
            let (r1, r2) = circle_r[i];

            let c1 = Circle::new((*x1 as f32, *y1 as f32), r1 as f32, (0, 0, 0));
            let c2 = Circle::new((*x2 as f32, *y2 as f32), r2 as f32, (0, 0, 0));
            assert_eq!(check_col_c_c(&c1, &c2), should_collide[i]);
        }
    }
}
