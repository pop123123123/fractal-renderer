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

fn main() {
  let mut scale: f64 = 0.25;
  let (mut center_x, mut center_y): (f64, f64) = (0.0, 0.0);
  for i in 0..300 {
    let frame = (0..WIDTH * HEIGHT)
      .into_par_iter()
      .map(|a| {
        fractale(
          a_to_x(a, scale) + center_x,
          a_to_y(a, scale) + center_y,
          scale,
        )
      })
      .collect::<Vec<Color>>();
    if i == 0 {
      let (center_x_, center_y_) = find_spot(&frame, scale, center_x, center_y);
      center_x = center_x_;
      center_y = center_y_;
    }
    scale *= 1.1;
    save_image(&frame, format!("images/{:03}.png", i));
  }
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
