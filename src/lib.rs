extern crate js_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, FocusEvent, HtmlCanvasElement, KeyboardEvent};

use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
extern {
  fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_f64(a: f64);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tetromino {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}
use self::Tetromino::*;

type TetroValues = [[[i32; 4]; 4]; 4];
fn tetro_values(t: Tetromino) -> TetroValues {
  match t {
    I => [
      [
        [0, 0, 0, 0],
        [1, 1, 1, 1],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
      ],
      [
        [0, 0, 1, 0],
        [0, 0, 1, 0],
        [0, 0, 1, 0],
        [0, 0, 1, 0],
      ],
      [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [1, 1, 1, 1],
        [0, 0, 0, 0],
      ],
      [
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 1, 0, 0],
      ]
    ],
    J => [
      [
        [1, 0, 0, 0],
        [1, 1, 1, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
      ],
      [
        [0, 1, 1, 0],
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 0, 0, 0],
      ],
      [
        [0, 0, 0, 0],
        [1, 1, 1, 0],
        [0, 0, 1, 0],
        [0, 0, 0, 0],
      ],
      [
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [1, 1, 0, 0],
        [0, 0, 0, 0],
      ]
    ],
    L => [
      [
        [0, 0, 1, 0],
        [1, 1, 1, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
      ],
      [
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
      ],
      [
        [0, 0, 0, 0],
        [1, 1, 1, 0],
        [1, 0, 0, 0],
        [0, 0, 0, 0],
      ],
      [
        [1, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 0, 0, 0],
      ]
    ],
    O => [
      [
        [0, 0, 0, 0],
        [0, 1, 1, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
      ],
      [
        [0, 0, 0, 0],
        [0, 1, 1, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
      ],
      [
        [0, 0, 0, 0],
        [0, 1, 1, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
      ],
      [
        [0, 0, 0, 0],
        [0, 1, 1, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
      ]
    ],
    S => [
      [
        [0, 1, 1, 0],
        [1, 1, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
      ],
      [
        [0, 1, 0, 0],
        [0, 1, 1, 0],
        [0, 0, 1, 0],
        [0, 0, 0, 0],
      ],
      [
        [0, 0, 0, 0],
        [0, 1, 1, 0],
        [1, 1, 0, 0],
        [0, 0, 0, 0],
      ],
      [
        [1, 0, 0, 0],
        [1, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 0, 0, 0],
      ]
    ],
    T => [
      [
        [0, 1, 0, 0],
        [1, 1, 1, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
      ],
      [
        [0, 1, 0, 0],
        [0, 1, 1, 0],
        [0, 1, 0, 0],
        [0, 0, 0, 0],
      ],
      [
        [0, 0, 0, 0],
        [1, 1, 1, 0],
        [0, 1, 0, 0],
        [0, 0, 0, 0],
      ],
      [
        [0, 1, 0, 0],
        [1, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 0, 0, 0],
      ]
    ],
    Z => [
      [
        [1, 1, 0, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
      ],
      [
        [0, 0, 1, 0],
        [0, 1, 1, 0],
        [0, 1, 0, 0],
        [0, 0, 0, 0],
      ],
      [
        [0, 0, 0, 0],
        [1, 1, 0, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
      ],
      [
        [0, 1, 0, 0],
        [1, 1, 0, 0],
        [1, 0, 0, 0],
        [0, 0, 0, 0],
      ]
    ],
  }
}

fn rand_int(max: usize) -> usize {
  (js_sys::Math::random() * (max as f64)) as usize
}

fn tetro_random() -> Tetromino {
  return [I, J, L, O, S, T, Z][rand_int(7)];
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Block {
  Blank,
  Fill,
}
use self::Block::*;

#[derive(Debug, Clone)]
struct Board {
  rows: usize,
  cols: usize,
  block_width: u32,
  matrix: Vec<Vec<Block>>,
  current_tetro: Tetromino,
  current_x: i32,
  current_y: i32,
  current_rotation: u32,
  speed: f64,
  game_over: bool,
}

impl Board {
  fn new(rows: usize, cols: usize, block_width: u32) -> Board {
    let current_tetro = tetro_random();

    Board {
      rows,
      cols,
      block_width,
      matrix: vec![vec![Blank; cols]; rows],
      current_tetro: current_tetro,
      current_x: 4,
      current_y: 0,
      current_rotation: 0,
      speed: 500.0,
      game_over: false,
    }
  }
}

impl Board {
  fn collision(&self, x: i32, y: i32, rotated_piece: [[i32; 4]; 4]) -> bool {
    for row in 0..rotated_piece.len() {
      for col in 0..rotated_piece.len() {
        if rotated_piece[row][col] == 0 { continue; }

        let newX = col + x as usize;
        let newY = row + y as usize;

        if newX < 0 || newX >= self.cols || newY >= self.rows { return true; }

        if newY < 0 { continue; }
        if self.matrix[newY][newX] != Blank { return true; }
      }
    }
    return false;
  }
}

impl Board {
  fn lock(&mut self) {
    let vals = tetro_values(self.current_tetro);
    let rotated_piece = vals[self.current_rotation as usize];
    for row in 0..rotated_piece.len() {
      for col in 0..rotated_piece.len() {
        if rotated_piece[row][col] == 0 { continue; }

        let y = self.current_y + row as i32;
        let x = self.current_x + col as i32;

        if y <= 1 {
          alert("Game over!");
          self.game_over = true;
          return;
        }

        self.matrix[y as usize][x as usize] = Fill;
      }
    }

    for row in 0..self.rows {
      let mut full = true;

      for col in 0..self.cols {
        if self.matrix[row as usize][col as usize] == Blank { full = false; }
      }

      if full {
        for y in (1..row+1).rev() {
          for col in 0..self.cols {
            self.matrix[y as usize][col as usize] = self.matrix[y as usize -1][col as usize]
          }
        }
      }
    }

    self.current_rotation = 0;
    self.current_y = 0;
    self.current_x = 5;
    self.current_tetro = tetro_random()
  }
}

impl Board {
  fn down(&mut self) {
    self.current_y += 1;
  }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Tetris {
  board: Board,
  context: CanvasRenderingContext2d,
  width: u32,
  height: u32,
  delta: u32,
  color_blank: JsValue,
  color_fill: JsValue,
}

impl Tetris {
  fn build(
    canvas: &HtmlCanvasElement,
    rows: usize,
    cols: usize,
    block_width: u32,
  ) -> Tetris {
    let board = Board::new(rows, cols, block_width);
    let delta = block_width + 1;
    let width = cols as u32 * delta;
    let height = 60u32 + rows as u32 * delta;
    let color_blank = JsValue::from_str("white");
    let color_fill = JsValue::from_str("red");

    let context = canvas
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<CanvasRenderingContext2d>()
      .unwrap();

    canvas.set_width(width);
    canvas.set_height(height);

    let rc_tetris = Tetris{
      board,
      context,
      width,
      height,
      delta,
      color_blank,
      color_fill,
    };

    rc_tetris.clone().render();
    rc_tetris
  }
}

impl Tetris {
  fn render(&mut self) {
    let block_width = self.board.block_width as f64;
    let delta = self.delta as f64;

    // draw board
    for row in 0..self.board.rows {
      for col in 0..self.board.cols {
        self.context
          .set_fill_style(if self.board.matrix[row][col] == Fill {
              &self.color_fill
          } else {
              &self.color_blank
          });
        self.context.fill_rect(col as f64 * delta, row as f64 * delta, block_width, block_width);

        self.context.set_stroke_style(&JsValue::from_str("black"));
        self.context.stroke_rect(col as f64 * delta, row as f64 * delta, block_width, block_width);
      }
    }

    // draw currrent piece
    let vals = tetro_values(self.board.current_tetro);
    let rotated_piece = vals[self.board.current_rotation as usize];
    for row in 0..rotated_piece.len() {
      for col in 0..rotated_piece.len() {
        if rotated_piece[row][col] != 0 {
          self.context.set_fill_style(&JsValue::from_str("black"));
          let x = col as f64 + self.board.current_x as f64;
          let y = row as f64 + self.board.current_y as f64;

          self.context.fill_rect(
            x * delta as f64,
            y * delta as f64,
            block_width,
            block_width,
          );
        }
      }
    }
  }
}

fn next_rotation(v: u32) -> u32 {
  if v + 1 > 3 {
    return 0;
  }
  return v +1;
}

#[wasm_bindgen]
impl Tetris {
  pub fn move_down(&mut self) {
    log("move_down...");
    let vals = tetro_values(self.board.current_tetro);
    let rotated_piece = vals[self.board.current_rotation as usize];
    if !self.board.collision(self.board.current_x, self.board.current_y+1, rotated_piece) {
      self.board.down();
    } else {
      self.board.lock()
    }
    self.render();
  }

  pub fn tick(&mut self) {
    log("ticking...");
    if self.board.game_over { return; }
    self.move_down();
  }

  pub fn rotate(&mut self) {
    log("rotate...");

    let next = next_rotation(self.board.current_rotation);

    let vals = tetro_values(self.board.current_tetro);
    let rotated_piece = vals[next as usize];
    if !self.board.collision(self.board.current_x, self.board.current_y, rotated_piece) {
      self.board.current_rotation = next;
      self.render();
    }
  }

  pub fn move_left(&mut self) {
    let vals = tetro_values(self.board.current_tetro);
    let rotated_piece = vals[self.board.current_rotation as usize];
    if !self.board.collision(self.board.current_x-1, self.board.current_y+1, rotated_piece) {
      log("move_left no collision");
      self.board.current_x = self.board.current_x -1;
    }
    self.render();
  }

  pub fn move_right(&mut self) {
    log("move_right...");
    let vals = tetro_values(self.board.current_tetro);
    let rotated_piece = vals[self.board.current_rotation as usize];
    if !self.board.collision(self.board.current_x+1, self.board.current_y+1, rotated_piece) {
      self.board.current_x = self.board.current_x+1;
      self.render();
    }
  }
}

fn window() -> web_sys::Window {
  web_sys::window().expect("global `window` should be OK.")
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) -> i32 {
  window()
    .request_animation_frame(f.as_ref().unchecked_ref())
    .expect("`requestAnimationFrame` should be OK.")
}

fn cancel_animation_frame(id: i32) {
  window()
    .cancel_animation_frame(id)
    .expect("`cancelAnimationFrame` should be OK.");
}

#[wasm_bindgen]
pub fn build_board(rows: usize, cols: usize, block_width: u32) -> Tetris {
  set_panic_hook();

  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let canvas = document.get_element_by_id("board").unwrap()
    .dyn_into::<HtmlCanvasElement>().unwrap();

  return Tetris::build(&canvas, rows, cols, block_width);
}

pub fn set_panic_hook() {
  #[cfg(feature = "console_error_panic_hook")]
  console_error_panic_hook::set_once();
}
