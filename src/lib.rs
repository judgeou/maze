extern crate image;
extern crate lab;

use std::collections::BinaryHeap;
use std::cmp::Ordering;
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

fn log (s: &str) {
  console::log_1(&JsValue::from_str(s));
}

fn log_i32 (s: i32) {
  console::log_1(&JsValue::from(s));
} 

#[derive(Eq)]
struct Node {
  pub x: usize,
  pub y: usize,
  pub end_distance: u32,
  pub start_distance: u32,
  pub distance: u32,
  pub passed: bool,
  pub is_queue: bool,
  pub parent: Option<usize>, // 存储Node集合的下标
  pub is_path: bool,
  pub next_nodes: Vec<usize> // 存储Node集合的下标的集合
}

impl Node {
  pub fn new () -> Self {
    Node {
      x: 0,
      y: 0,
      end_distance: 0,
      start_distance: 0,
      distance: 0,
      passed: false,
      is_queue: false,
      parent: None,
      is_path: true,
      next_nodes: vec![]
    }
  }
}

impl Ord for Node {
  fn cmp (&self, other: &Self) -> Ordering {
    other.distance.cmp(&self.distance)
  }
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Node {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}

unsafe fn r<T>(rf: &T) -> &mut T {
  let const_ptr = rf as *const T;
  let mut_ptr = const_ptr as *mut T;
  &mut *mut_ptr
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
pub fn solve_maze (matrix: Box<[u8]>, start: Box<[usize]>, end: Box<[usize]>, width: usize, height: usize) -> Vec<usize> {
  let nodes = build_nodes(&matrix, &end, width, height);
  let start_node = get_node(&nodes, start[0], start[1], width);
  let end_node = get_node(&nodes, end[0], end[1], width);

  if start_node.is_some() && end_node.is_some() {
    let check_count = build_path(&nodes, start_node.unwrap(), end_node.unwrap(), width);
    let paths = backtrack_path(&nodes, end_node.unwrap());
    paths
  } else {
    // 不可到达
    vec![]
  }
}

fn backtrack_path (nodes: &Vec<Option<Node>>, end_node: &Node) -> Vec<usize> {
  let mut paths = vec![end_node.x, end_node.y];
  let mut parent = end_node.parent;

  while parent.is_some() {
    let index = parent.unwrap();
    let node = nodes[index].as_ref().unwrap();
    paths.push(node.x);
    paths.push(node.y);
    parent = node.parent;
  }

  paths
}

fn build_path (nodes: &Vec<Option<Node>>, start_node: &Node, end_node: &Node, width: usize) -> i32 {
  let mut check_count = 0;
  
  let mut queue = BinaryHeap::new();
  queue.push(start_node);

  loop {
    let node = queue.pop();
    match node {
      Some (node) => {
        check_count += 1;
        unsafe {
          r(node).passed = true;
        }

        for &n in &node.next_nodes {
          let next = nodes[n].as_ref().unwrap();
          if next.passed == false {
            unsafe {
              let next_r = r(next);
              next_r.parent = Some(get_node_index(node.x, node.y, width));
              next_r.start_distance = node.start_distance + 1;
              next_r.distance = next_r.start_distance + next_r.end_distance;

              if !next.is_queue {
                queue.push(next);
                next_r.is_queue = true;
              }

              if next == end_node {
                log("wasm solve!");
                return check_count;
              }
            }
          }
        }

      },
      None => {
        break;
      }
    }
  }

  check_count
}

/// 构建图结构
fn build_nodes (matrix: &Box<[u8]>, end: &Box<[usize]>, width: usize, height: usize) -> Vec<Option<Node>> {
  let mut nodes = vec![];
  
  for y in 0..height {
    for x in 0..width {
      let v = matrix[y * width + x];
      if v > 0 {
        let mut node = Node::new();
        node.x = x;
        node.y = y;
        node.end_distance = get_distance(&[x, y], end);
        nodes.push(Some(node));
      } else {
        nodes.push(None);
      }
    }
  }

  for i in 0..nodes.len() {
    let node = &nodes[i];
    match node {
      Some(node_v) => {
        let next = get_next_nodes(&nodes, node_v.x, node_v.y, width);
        let node_mut = &mut nodes[i];
        node_mut.as_mut().unwrap().next_nodes = next;
      },
      None => {}
    }
  }

  nodes
}

fn get_next_nodes (nodes: &Vec<Option<Node>>, x: usize, y: usize, width: usize) -> Vec<usize> {
  let mut result = vec![];

  let mut set_node = |x: usize, y: usize| {
    let node = get_node(nodes, x, y, width);
    if node.is_some() {
      result.push(get_node_index(x, y, width));
    }
  };

  if x > 0 { set_node(x - 1, y); }
  set_node(x + 1, y);
  if y > 0 { set_node(x, y - 1); }
  set_node(x, y + 1);

  result
}

fn get_node (nodes: &Vec<Option<Node>>, x: usize, y: usize, width: usize) -> Option<&Node> {
  let index = get_node_index(x, y, width);
  if index < nodes.len() {
    match &nodes[index] {
      Some (node) => Some(node),
      None => None
    }
  } else {
    None
  }
}

fn get_node_index (x: usize, y: usize, width: usize) -> usize {
  return y * width + x
}

fn get_distance (node_a: &[usize], node_b: &[usize]) -> u32 {
  let x = (node_b[0] as i32 - node_a[0] as i32).abs();
  let y = (node_b[1] as i32 - node_a[1] as i32).abs();
  return (x + y) as u32
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
