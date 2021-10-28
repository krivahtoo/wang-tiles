use std::fs::File;
use std::io::{Result, Write, BufWriter};
use rand::seq::SliceRandom;

#[derive(Clone, Copy)]
enum TileType {
  North = 1,
  East = 2,
  South = 4,
  West = 8
}

struct Tile(Option<TileType>,Option<TileType>,Option<TileType>,Option<TileType>);

use TileType::*;

impl Tile {
  fn new(num: usize) -> Self {
    let mut n: Option<TileType> = None;
    let mut e: Option<TileType> = None;
    let mut s: Option<TileType> = None;
    let mut w: Option<TileType> = None;
    if get_bit(num as u32, 0) {
      n = Some(North);
    }
    if get_bit(num as u32, 1) {
      e = Some(East);
    }
    if get_bit(num as u32, 2) {
      s = Some(South);
    }
    if get_bit(num as u32, 3) {
      w = Some(West);
    }
    Self(n,e,s,w)
  }
  fn get_possible(&self,edge: u8) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    if edge >= 4 { panic!("Edge out of range") }
    let n = if edge <= 1 { edge + 2 } else { edge - 2 };
    for x in 0..16 {
      let tile = Self::new(x);
      match n {
        0 if type_eq(&self.0, &tile.0) => {
            res.push(x as u8);
        },
        1 if type_eq(&self.1, &tile.1) => {
            res.push(x as u8);
        },
        2 if type_eq(&self.2, &tile.2) => {
            res.push(x as u8);
        },
        3 if type_eq(&self.3, &tile.3) => {
            res.push(x as u8);
        },
        _ => (),
      }
    }
    println!("{} -> {} => {:?}", &edge, &n, &res);
    res
  }
  fn to_u8(&self) -> u8 {
    let mut num = 0;
    if let Some(North) = self.0 {
        num = num + 1;
    }
    if let Some(East) = self.1 {
        num = num + 2;
    }
    if let Some(South) = self.2 {
        num = num + 4;
    }
    if let Some(West) = self.3 {
        num = num + 8;
    }
    num
  }
}

fn type_eq(a: &Option<TileType>, b: &Option<TileType>) -> bool {
  match (a, b) {
    (&Option::Some(..), &Option::Some(..)) => true,
    (&Option::None, &Option::None) => true,
    _ => false,
  }
}


struct RGB(u8,u8,u8);

impl RGB {
  fn to_u32(&self) -> u32 {
    ((self.0 as u32) << 4) + ((self.1 as u32) << 2) + (self.2 as u32)
  }
}

fn get_rand(vs: &[u8]) -> u8 {
  *(vs.choose(&mut rand::thread_rng()).unwrap())
}

// Returns true if the i'th bit of x is set to 1.
fn get_bit(x: u32, i: i32) -> bool {
  (x >> i) & 1 != 0
}

fn save_as_ppm(file_path: &str, pixels: &[u32], width: usize, height: usize) -> Result<()> {
  let mut buffer: Vec<u8> = Vec::new();

  for y in 0..height {
    for x in 0..width {
      let px = pixels[y*width+x];
      buffer.push(((px >> 8 * 2) & 0xff) as u8);
      buffer.push(((px >> 8 * 1) & 0xff) as u8);
      buffer.push(((px >> 8 * 0) & 0xff) as u8);
    }
  }

  let mut file = File::create(file_path)?;
  write!(file, "P6\n{} {} 255\n", width, height)?;
  file.write(&buffer)?;
  Ok(())
}

fn draw_circle(u: f32, v: f32) -> RGB {
  let c = (0.5, 0.5);
  let r = 0.25;
  let dx = c.0 - u;
  let dy = c.1 - v;
  if dx*dx + dy*dy <= r*r {
    RGB(0x00,0x0a,0x00)
  } else {
    RGB(0xff, 0xff, 0xff)
  }
}

// u for x axis, v for y axis
fn draw(u: f32,v: f32,n: u8) -> RGB {
  let c = 0.5;
  let res: bool = match n {
    0 => false,
    1 => (u >= v && v <= c && u <= 1.0 - v),
    2 => (v <= u && u >= 1.0 - v),
    3 => (u >= v),
    4 => (v >= u && v >= 1.0 - u),
    5 => (u >= v && v <= c && u <= 1.0 - v) || (v >= u && v >= 1.0 - u),
    6 => (v >= u && v >= 1.0 - u) || (v <= u && u >= 1.0 - v),
    7 => (u >= v) || (v >= u && v >= 1.0 - u),
    8 => (v >= u && v <= 1.0 - u),
    9 => (v <= 1.0 - u),
    10 => (v >= u && v <= 1.0 - u) || (v <= u && u >= 1.0 - v),
    11 => (v >= u && v <= 1.0 - u) || (u >= v),
    12 => (v >= u),
    13 => (v >= u) || (u >= v && v <= c && u <= 1.0 - v),
    14 => (v >= u) || (v <= u && u >= 1.0 - v),
    15 => true,
    _ => panic!("Value out of range"),
  };
  if res {
    RGB(0x00,0x0a,0x00)
  } else {
    RGB(0xff, 0xff, 0xff)
  }
}

fn main() {
  const HEIGHT: usize = 512;
  const WIDTH: usize = 512;
  const TILES_H: usize = 16;
  const TILES_W: usize = 16;
  const OUTPUT_PATH: &str = "output.ppm";
  let mut pixels = [0u32; WIDTH*HEIGHT];
  let tiles: usize = TILES_H*TILES_W;
  let tileset: [u8; 16] = [
    0b0000, // 0
    0b0001, // 1
    0b0010, // 2
    0b0011, // 3
    0b0100, // 4
    0b0101, // 5
    0b0110, // 6
    0b0111, // 7
    0b1000, // 8
    0b1001, // 9
    0b1010, // 10
    0b1011, // 11
    0b1100, // 12
    0b1101, // 13
    0b1110, // 14
    0b1111  // 15
  ];
  let mut next_tile: usize = get_rand(&tileset) as usize;
  let mut last_x_start_tile = Tile::new(next_tile);
  let mut last_x_tiles: Vec<Tile> = Vec::new();
  for y in 0..TILES_H {
    let mut x_buffer: Vec<Tile> = Vec::new();
    for x in 0..TILES_W {
      let mut num: u8 = 0;
      let mut vs: Vec<u8> = Vec::new();
      let mut tile: Tile = Tile::new(next_tile);
      if x == 0 && y > 0 {
        vs = last_x_start_tile.get_possible(2);
        tile = Tile::new(get_rand(&vs) as usize);
      } else if y > 0 {
      } else {
        vs = tile.get_possible(1);
      }
      num = tile.to_u8();
      for y1 in 0..(HEIGHT/TILES_H) {
        for x1 in 0..(WIDTH/TILES_W) {
          let u = (x1 as f32)/((WIDTH/TILES_W) as f32);
          let v = (y1 as f32)/((HEIGHT/TILES_H) as f32);
          pixels[
            ((y*HEIGHT/TILES_H)+y1)*WIDTH+((x*WIDTH/TILES_W)+x1)
          // ] = draw_circle(u, v);
          ] = draw(u, v, num).to_u32();
        }
      }
      if x == 0 {
        last_x_start_tile = Tile::new(num as usize);
      }
      vs = tile.get_possible(1);
      x_buffer.push(Tile::new(num as usize));
      println!("{:?} -> {}", &vs, &next_tile);
      next_tile = get_rand(&vs) as usize;
    }
    last_x_tiles = x_buffer;
  }
  println!("{}", WIDTH / TILES_W);
  println!("{}", West as u8);
  save_as_ppm(OUTPUT_PATH, &pixels, WIDTH, HEIGHT).unwrap();
}
