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
  let scale: f64 = 0.25;
  let (center_x, center_y): (f64, f64) = (0.0, 0.0);
  let frame = (0..WIDTH * HEIGHT)
    .into_par_iter()
    .map(|a| {
      fractale(
        (a_to_x(a) + center_x) / scale,
        (a_to_y(a) + center_y) / scale,
      )
    })
    .collect::<Vec<Color>>();
  save_image(&frame, "render.png");
}

fn a_to_x(a: usize) -> f64 {
  (((a % WIDTH) as isize - (WIDTH as isize) / 2) as f64) / WIDTH_F
}

fn a_to_y(a: usize) -> f64 {
  (((a / WIDTH) as isize - (HEIGHT as isize) / 2) as f64) / HEIGHT_F
}
