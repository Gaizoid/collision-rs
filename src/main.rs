use collision_rs::{check_collisions_circle, Circle, fast_rand_range};
use image::{ ImageBuffer, Rgb };

fn main() {
  let width = 1280u32;
  let height = 720u32;

  let max_radius = 90;
  let circles_count = 5;

  let mut circles: Vec<Circle> = Vec::new();
  for _i in 0..circles_count {
      circles.push(
        Circle::new(
            // Random position vector
            (
                fast_rand_range(max_radius, (width as i32)-max_radius) as f32,
                fast_rand_range(max_radius, (height as i32)-max_radius) as f32
            ),
            // Random radius
            fast_rand_range(20, max_radius) as f32,
            // Random color
            (
                fast_rand_range(0, 255) as u8,
                fast_rand_range(0, 255) as u8,
                fast_rand_range(0, 255) as u8
            )
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
    // Rendering could be optimised here
    for circle in circles {
      if
        (x as f32 -circle.pos.0)*(x as f32 -circle.pos.0)
        + (y as f32 -circle.pos.1)*(y as f32 -circle.pos.1)
        < circle.r*circle.r {
        *pixel = Rgb([circle.color.0, circle.color.1, circle.color.2]);
      }
    }
  }
  img_buf.save("circles.png").unwrap();
}