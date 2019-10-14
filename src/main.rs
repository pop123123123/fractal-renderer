extern crate image;
extern crate itertools;
extern crate palette;
extern crate pbr;
extern crate rand;
extern crate rayon;
use palette::{Hsv, Srgb};
use std::sync::RwLock;

use crate::rayon::iter::*;
use pbr::ProgressBar;
use std::iter;

mod img;
use crate::img::*;
mod fractale;
use crate::fractale::*;

const COUNT: u64 = 90;

fn main() {
  let mut pb = ProgressBar::new(COUNT);
  pb.format("╢▌▌░╟");

  let mut scale: f64 = 0.25;
  let (center_x, center_y): (f64, f64) = get_center(scale);
  let mut frame: Vec<Color> = iter::repeat_with(|| [255, 255, 255])
    .take(HEIGHT * WIDTH)
    .collect();

  for i in 0..COUNT {
    frame = (0..WIDTH * HEIGHT)
      .into_par_iter()
      .map(|a| {
        if black_around(&frame, scale, a) {
          [0, 0, 0]
        } else {
          fractale(
            a_to_x(a, scale) + center_x,
            a_to_y(a, scale) + center_y,
            scale,
          )
        }
      })
      .collect::<Vec<Color>>();
    save_image(&frame, format!("images/{:03}.png", i));
    scale *= 1.1;
    pb.inc();
  }
  pb.finish_print("done");
}

fn get_center(scale: f64) -> (f64, f64) {
  let frame = (0..WIDTH * HEIGHT)
    .into_par_iter()
    .map(|a| fractale(a_to_x(a, scale), a_to_y(a, scale), scale))
    .collect::<Vec<Color>>();
  find_spot(&frame, scale, 0.0, 0.0)
}

fn black_around(screen: &Vec<Color>, scale: f64, a: usize) -> bool {
  let mut indices: Vec<usize> = Vec::new();
  for a in [a - WIDTH, a, a + WIDTH].iter() {
    let a = *a;
    if a < HEIGHT * WIDTH {
      indices.push(a);
      if a % WIDTH > 0 {
        indices.push(a - 1);
      }
      if a % WIDTH < WIDTH - 1 {
        indices.push(a + 1);
      }
    }
  }
  indices
    .iter()
    .map(|a| {
      let c = screen[xy_to_a(a_to_x(*a, scale), a_to_y(*a, scale), scale / 1.1)];
      c[0] as u16 + c[1] as u16 + c[2] as u16
    })
    .sum::<u16>()
    == 0
}

fn find_spot(screen: &Vec<Color>, scale: f64, center_x: f64, center_y: f64) -> (f64, f64) {
  let (a, max) = screen
    .iter()
    .enumerate()
    .fold((0, 0), |(pi, pmax): (usize, u8), (i, col)| {
      let max = *col.iter().max().unwrap();
      if max > pmax {
        (i, max)
      } else {
        (pi, pmax)
      }
    });
  ((a_to_x(a, scale) + center_x), (a_to_y(a, scale) + center_y))
}

fn a_to_x(a: usize, scale: f64) -> f64 {
  (((a % WIDTH) as isize - (WIDTH as isize) / 2) as f64) / (WIDTH_F * scale)
}

fn a_to_y(a: usize, scale: f64) -> f64 {
  -(((a / WIDTH) as isize - (HEIGHT as isize) / 2) as f64) / (HEIGHT_F * scale)
}

fn xy_to_a(x: f64, y: f64, scale: f64) -> usize {
  ((x * scale * WIDTH_F) as isize + (WIDTH as isize / 2)) as usize
    + WIDTH * (((-y) * scale * HEIGHT_F) as isize + (HEIGHT as isize / 2)) as usize
}
