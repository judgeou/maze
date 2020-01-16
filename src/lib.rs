extern crate image;
extern crate lab;

use wasm_bindgen::prelude::*;
use web_sys::console;
use image::GenericImageView;
use lab::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console::log_1(&JsValue::from_str("我好了"));

    Ok(())
}

pub struct Node {
  pub x: i32,
  pub y: i32,
  pub end_distance: i32,
  pub start_distance: i32,
  pub distance: i32,
  pub passed: bool,
  pub is_queue: bool,
  pub parent: Box<Node>,
  pub is_path: bool,
  pub next_nodes: Vec<Box<Node>>
}

#[wasm_bindgen]
pub fn gen_map_array (buffer: Box<[u8]>, start_point: Box<[u32]>) -> Vec<u8> {
  let img = image::load_from_memory(&buffer).unwrap();
  let (width, height) = img.dimensions();

  let start_pixel = img.get_pixel(start_point[0], start_point[1]);
  let walk_color = [start_pixel[0], start_pixel[1], start_pixel[2]];

  let mut result = Vec::with_capacity((width * height) as usize);
  for h in 0..height {
    for w in 0..width {
      let pixel = img.get_pixel(w, h);
      let red = pixel[0];
      let green = pixel[1];
      let blue = pixel[2];

      let color_distance = color_diff_lab(&walk_color, &[ red, green, blue ]);
      let max_diff = 81.0;
      if color_distance < max_diff {
        result.push(1)
      } else {
        result.push(0);
      }
    } 
  }

  return result;
}

#[wasm_bindgen]
pub fn solve_maze (matrix: Box<[u8]>, start: Box<[u8]>, end: Box<[u8]>, wdith: usize, height: usize) -> i32 {
  1
}

fn build_nodes (matrix: Box<[u8]>, end: Box<[u8]>, start: Box<[u8]>, width: usize, height: usize) {
  
}

fn color_diff_lab (walk_color: &[u8], stop_color: &[u8]) -> f32 {
  let walk_lab = Lab::from_rgb(&[ walk_color[0], walk_color[1], walk_color[2] ]);
  let stop_lab = Lab::from_rgb(&[ stop_color[0], stop_color[1], stop_color[2] ]);

  delta_e(&walk_lab, &stop_lab)
}

fn color_diff_rgb (walk_color: &[u8], stop_color: &[u8]) -> f32 {
  let dr = (walk_color[0] as i16 - stop_color[0] as i16).abs();
  let dg = (walk_color[1] as i16 - stop_color[1] as i16).abs();
  let db = (walk_color[2] as i16 - stop_color[2] as i16).abs();

  ((dr * dr) + (dg * dg) + (db * db)).into()
}

fn delta_e (a: &Lab, b: &Lab) -> f32 {
  let delta_l = a.l - b.l;
  let delta_a = a.a - b.a;
  let delta_b = a.b - b.b;

  ((delta_l * delta_l) + (delta_a * delta_a) + (delta_b * delta_b))
}
