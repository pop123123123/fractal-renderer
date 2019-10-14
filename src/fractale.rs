
use palette::*;
use crate::img;


const DEPTH: u8 = 255;
const LIMIT: f64 = 1.0e8;

pub fn fractale(x: f64, y: f64) -> img::Color {
  //(((x.powf(2.0) + y.powf(2.0)).sqrt().sin() + 1.0) / 2.0).round()
  //(((x.abs().max(y.abs())).sin() + 1.0) / 2.0).round()
  //(((x.abs().max(y.abs())).sin() + 1.0) / 2.0).round()
  let a = mand([x, y], [x, y], DEPTH, LIMIT);
  /*if a < 0 {
    0.0
  } else {
    (DEPTH as i32 - a) as f64
  }*/
  let col = Hsv::new(
    palette::RgbHue::<f32>::from_degrees(a as f32 * 4.0),
    1.0,
    (a >= 0) as u8 as f32,
  );

  let rgb: palette::rgb::LinSrgb = col.into();
  rgb.into_format().into_raw::<[u8; 3]>()
}

type Complex = [f64; 2];

fn square(a: Complex) -> Complex {
  [a[0].powf(2.0) - a[1].powf(2.0), 2.0 * a[0] * a[1]]
}

fn add(a: Complex, b: Complex) -> Complex {
  [a[0] + b[0], a[1] + b[1]]
}

fn mand(c: Complex, previous: Complex, depth: u8, limit: f64) -> i32 {
  if previous[0] + previous[1] > limit {
    depth as i32
  } else if depth == 0 {
    -1
  } else {
    mand(c, add(square(previous), c), depth - 1, limit)
  }
}
