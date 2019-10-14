use crate::img;
use palette::*;

const LIMIT: f64 = 2.0;

pub fn fractale(x: f64, y: f64, scale: f64) -> img::Color {
  let depth: u16 = (((1 << 10) as f64) + scale) as u16;
  //(((x.powf(2.0) + y.powf(2.0)).sqrt().sin() + 1.0) / 2.0).round()
  //(((x.abs().max(y.abs())).sin() + 1.0) / 2.0).round()
  //(((x.abs().max(y.abs())).sin() + 1.0) / 2.0).round()
  let a = mand([x, y], [x, y], depth, LIMIT);
  /*if a < 0 {
    0.0
  } else {
    (depth as i32 - a) as f64
  }*/
  let col = Hsv::new(
    palette::RgbHue::<f32>::from_degrees(a as f32 * 360.0 / depth as f32),
    1.0,
    if a < 0 {
      0.0
    } else {
      (((depth as i32 - a) as f32 / (depth as f32))/*+1.0*/).powf(0.1) //.log(2.0)
    },
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

fn mand(c: Complex, previous: Complex, depth: u16, limit: f64) -> i32 {
  let q = (previous[0] - 0.25).powf(2.0) + previous[1].powf(2.0);
  if q * (q + (previous[0] - 0.25)) <= 0.25 * previous[1].powf(2.0) {
    return -1;
  }

  let mut previous = previous;
  let mut res = -1;
  for depth in (0..(depth + 1)).rev() {
    //println!("{}", depth);
    if previous[0] + previous[1] > limit {
      res = depth as i32;
      break;
    } else {
      previous = add(square(previous), c);
    }
  }
  res
}
