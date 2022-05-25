use collision_rs::{check_collisions_circle, Circle, fast_rand_range};
use image::{ ImageBuffer, Rgb };

fn main() {
  let width = 1280u32;
  let height = 720u32;

  let mut circles: Vec<Circle> = Vec::new();
  for i in 0..5 {
      circles.push(
        Circle::new(
            (
                fast_rand_range(90, width-90) as f32,
                fast_rand_range(90, height-90) as f32
            ),
            fast_rand_range(20, 90) as f32
        )
      );
  }
  circles_image(&circles, width, height);
  println!("{:?}", check_collisions_circle(
    circles
  ));
}

fn circles_image(circles: &Vec<Circle>, width: u32, height: u32) {
  let mut img_buf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
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