use itertools::Itertools;
use itertools::MinMaxResult::{MinMax, NoElements, OneElement};

const POW: u8 = 10;

pub const WIDTH: usize = 1 << POW;
pub const HEIGHT: usize = 1 << POW;

pub const WIDTH_F: f64 = WIDTH as f64;
pub const HEIGHT_F: f64 = HEIGHT as f64;

pub type Color = [u8; 3];

pub fn save_image(screen: &Vec<Color>, name: String) {
  //let min_max = screen.iter().minmax().into_option();
  //let (min, max) = min_max.unwrap();

  let mut imgbuf = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

  for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    let c = screen[x as usize + (y as usize) * WIDTH];
    *pixel = image::Rgb(c);
    //*pixel = image::Rgb(float_to_color(c, *min, *max));
  }
  imgbuf.save(name).unwrap();
}

fn float_to_color(x: f64, min: f64, max: f64) -> Color {
  let value = ((x - min) * 255.0 / (max - min)) as u8;
  [value, value, value]
}
