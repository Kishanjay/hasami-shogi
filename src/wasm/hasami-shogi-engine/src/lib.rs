mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty = 0,
    Player1 = 1,
    Player2 = 2,
}

#[wasm_bindgen]
pub struct ShogiBoard {
    width: u8,
    height: u8,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl ShogiBoard {
  pub fn new(width: u8, height: u8, board_state: Vec<u8>) -> ShogiBoard {
    let mut cells: Vec<Cell> = vec![];

    for cell in board_state {
      match cell {
        0 => cells.push(Cell::Empty),
        1 => cells.push(Cell::Player1),
        2 => cells.push(Cell::Player2),
        _ => ()
      }
    }
    ShogiBoard {
      width: width,
      height: height,
      cells: cells,
    }
  }
  pub fn computer_move(&self) {

  }

  pub fn cells(&self) -> *const Cell {
    self.cells.as_ptr()
}
}

impl ShogiBoard {
  fn move_options(&self) -> Vec<(usize, usize)> {
    let x: Vec<(usize, usize)> = vec![(0, 4)];
    x
  }

  fn heuristic_value(&self) -> (i32) {
    1 // Placeholder
  }

  fn get_index (&self, row: u8, column: u8) -> usize {
    (row * self.width + column) as usize
  }
}
