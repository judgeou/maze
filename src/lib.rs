extern crate image;

use wasm_bindgen::prelude::*;
use web_sys::console;
use image::GenericImageView;

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

#[wasm_bindgen]
pub fn gen_map_array (buffer: Box<[u8]>, walk_color: Box<[u8]>, stop_color: Box<[u8]>) -> Vec<u8> {
  let max_diff = 2;
  // let startend_diff = color_diff(&walk_color, &stop_color);
  
  let img = image::load_from_memory(&buffer).unwrap();
  let (width, height) = img.dimensions();

  let mut result = Vec::with_capacity((width * height) as usize);
  for h in 0..height {
    for w in 0..width {
      let pixel = img.get_pixel(w, h);
      let red = pixel[0];
      let green = pixel[1];
      let blue = pixel[2];

      let color_distance = color_diff(&walk_color, &[ red, green, blue ]);
      if color_distance < max_diff {
        result.push(1)
      } else {
        result.push(0);
      }
    } 
  }

  return result;
}

fn color_diff (walk_color: &[u8], stop_color: &[u8]) -> i32 {
   let r = stop_color[0] as i32 - walk_color[0] as i32;
   r.abs()
}
