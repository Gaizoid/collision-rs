use collision_rs::{check_collisions_circle, Circle};

fn main() {
  let mut circles: Vec<Circle> = Vec::new();
  circles.push(Circle::new((1.0, 2.0), 3.0));
  circles.push(Circle::new((10.0, 2.0), 3.0));
  circles.push(Circle::new((-4.0, 2.0), 3.0));
  circles.push(Circle::new((-6.5, 2.0), 3.0));
  println!("{:?}", check_collisions_circle(
    circles
  ));
}