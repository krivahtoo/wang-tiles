use std::fs::File;
use std::io::{Result,Write};
use rand::seq::SliceRandom;

struct RGB(u8,u8,u8);

impl RGB {
  fn to_u32(&self) -> u32 {
    ((self.0 as u32) << 4) + ((self.1 as u32) << 2) + (self.2 as u32)
  }
}

fn get_rand(vs: &[u8]) -> u8 {
  *(vs.choose(&mut rand::thread_rng()).unwrap())
}

fn save_as_ppm(file_path: &str, pixels: &[u32], width: usize, height: usize) -> Result<()> {
  let mut file = File::create(file_path)?;
  write!(file, "P6\n{} {} 255\n", width, height)?;
  for y in 0..height {
    for x in 0..width {
      let px = pixels[y*width+x];
      let color = [
        ((px >> 8 * 2) & 0xff) as u8,
        ((px >> 8 * 1) & 0xff) as u8,
        ((px >> 8 * 0) & 0xff) as u8
      ];
      file.write(&color).unwrap();
    }
  }
  Ok(())
}

fn draw_circle(u: f32, v: f32) -> RGB {
  let c = (0.5, 0.5);
  let r = 0.25;
  let dx = c.0 - u;
  let dy = c.1 - v;
  if dx*dx + dy*dy <= r*r {
    RGB(0,10,0)
  } else {
    RGB(255, 255, 255)
  }
}

fn draw_triangle_top(u: f32, v: f32) -> RGB {
  let c = 0.5;
  if u >= v && v <= c && u <= 1.0 - v {
    RGB(0,10,0)
  } else {
    RGB(255, 255, 255)
  }
}

fn draw_triangle_left(u: f32, v: f32) -> RGB {
  if v >= u && v <= 1.0 - u {
    RGB(0,10,0)
  } else {
    RGB(255, 255, 255)
  }
}

fn draw_triangle_bottom(u: f32, v: f32) -> RGB {
  if v >= u && v >= 1.0 - u {
    RGB(0,10,0)
  } else {
    RGB(0xff, 0xff, 0xff)
  }
}

fn draw_triangle_right(u: f32, v: f32) -> RGB {
  if v <= u && u >= 1.0 - v {
    RGB(0,10,0)
  } else {
    RGB(0xff, 0xff, 0xff)
  }
}

fn main() {
  const HEIGHT: usize = 512;
  const WIDTH: usize = 512;
  const OUTPUT_PATH: &str = "output.ppm";
  let mut pixels = [0u32; WIDTH * HEIGHT];
  for y in 0..HEIGHT {
    for x in 0..WIDTH {
      let u = (x as f32) / (WIDTH as f32);
      let v = (y as f32) / (HEIGHT as f32);
      // pixels[y*WIDTH+x] = draw_circle(u, v);
      // pixels[y*WIDTH+x] = draw_triangle_top(u, v);
      // pixels[y*WIDTH+x] = draw_triangle_left(u, v).to_u32();
      // pixels[y*WIDTH+x] = draw_triangle_bottom(u, v).to_u32();
      pixels[y*WIDTH+x] = draw_triangle_right(u, v).to_u32();
    }
  }
  save_as_ppm(OUTPUT_PATH, &pixels, WIDTH, HEIGHT).unwrap();
}
