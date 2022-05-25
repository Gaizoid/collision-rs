use collision_rs::{check_collisions_circle, Circle};
use image::{ ImageBuffer, Rgb };

fn main() {
  let mut circles: Vec<Circle> = Vec::new();
  circles.push(Circle::new((500.0, 200.0), 30.0));
  circles.push(Circle::new((200.0, 300.0), 69.0));
  circles.push(Circle::new((900.0, 400.0), 30.0));
  circles.push(Circle::new((900.0, 430.0), 30.0));
  circles_image(&circles);
  println!("{:?}", check_collisions_circle(
    circles
  ));
}

fn circles_image(circles: &Vec<Circle>) {
  let mut img_buf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(1280, 720);
  for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
    for circle in circles {
      if
        (x as f32 -circle.pos.0)*(x as f32 -circle.pos.0)
        + (y as f32 -circle.pos.1)*(y as f32 -circle.pos.1)
        < circle.r*circle.r {
        *pixel = Rgb([255, 0, 0]);
      }
    }
  }
  img_buf.save("circles.png").unwrap();
}