use collision_rs::{check_collisions_circle, Circle, smooth_step};
use image::{ ImageBuffer, Rgb };

fn main() {
  let width = 1280 as f32;
  let height = 720 as f32;

  let max_radius = 200.0;
  let circles_count = 10;

  let mut circles: Vec<Circle> = Vec::new();
  for i in 0..circles_count {
    let i_float = i as f32;
    let circle_float = circles_count as f32;
    // position is from [0, 1)
    let position = i_float/circle_float;
    let inverse_position = 1.0 - position;

    // make sure circles are smaller than the next
    let radius = position * max_radius;
    
    circles.push(
      Circle::new(
        // Random position vector
        (
          position*(width-2.0*radius)+radius,
          height*smooth_step(max_radius, height-max_radius, position*(height-2.0*radius)+radius)
        ),
        // Random radius
        radius,
        // Random color
        (
          0u8,
          (100.0 + position * 155.0) as u8,
          (100.0 + inverse_position * 155.0) as u8,
        )
      )
    );
  }

  // DEBUG OUTPUT
  let collided_circles = check_collisions_circle(
    &circles
  );
  circles_image(&circles, width as u32, height as u32);
  println!("{:?}", collided_circles);
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