// Author: Kishan Nirghin
// Date: 03 Oktober 2020
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty = 0,
    Player1 = 1,
    Player2 = 2, // CPU
}

#[wasm_bindgen]
pub struct ShogiGame {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}


#[wasm_bindgen]
impl ShogiGame {
  pub fn new(width: usize, height: usize) -> ShogiGame {
    utils::set_panic_hook();
    let mut cells: Vec<Cell> = vec![];

    let board_size = width*height;
    for i in 0..board_size {
      if i < width {
        cells.push(Cell::Player2);
      } else if i >= board_size-width {
        cells.push(Cell::Player1);
      } else {
        cells.push(Cell::Empty);
      }
    }
    ShogiGame {
      width: width,
      height: height,
      cells: cells,
    }
  }

  pub fn import(width: usize, height: usize, board_state: Vec<u8>) -> ShogiGame {
    utils::set_panic_hook();
    let mut cells: Vec<Cell> = vec![];

    for cell in board_state {
      match cell {
        0 => cells.push(Cell::Empty),
        1 => cells.push(Cell::Player1),
        2 => cells.push(Cell::Player2),
        _ => ()
      }
    }
    ShogiGame {
      width: width,
      height: height,
      cells: cells,
    }
  }

  pub fn cells(&self) -> *const Cell {
    self.cells.as_ptr()
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn height(&self) -> usize {
    self.height
  }

  pub fn computer_move(&mut self) {
    let mut move_options = self.board_move_options();
    let mut best_heuristic = i32::MIN;
    let mut best_move = move_options[0];

    for move_option in move_options.iter_mut() {
      let move_heuristic = self.evaluate_move_option(*move_option);
      if move_heuristic > best_heuristic {
        best_heuristic = move_heuristic;
        best_move = *move_option;
      }
    }

    self.move_piece(best_move.0, best_move.1);
  }

  // Moves a piece on the shogiBoard also removes pieces that are captured with
  // the current move
  pub fn move_piece(&mut self, source_idx: usize, dest_idx: usize) {
    let cell_value = self.cells[source_idx];
    self.cells[source_idx] = Cell::Empty;
    self.cells[dest_idx] = cell_value;

    // Do capture detection.
    let mut captured_pieces = self.piece_captures(dest_idx);
    for idx in captured_pieces.iter_mut() {
      self.cells[*idx] = Cell::Empty;
    }
  }

  // The board state is stored as a single array, this helper function will
  // convert a <row,column> combination to the internal index for the element
  pub fn get_index (&self, row: usize, column: usize) -> usize {
    (row * self.width + column) as usize
  }
}

impl ShogiGame {
  fn evaluate_move_option(&self, move_option: (usize, usize)) -> i32 {
    let mut shogi_game = self.clone();
    shogi_game.move_piece(move_option.0, move_option.1);
    shogi_game.heuristic_value()
  }

  // returns a value indicating how many pieces player1 has over player2
  // for example:
  //   1 => player1 has 1 piece over player2
  //   -2 => player2 has 2 pieces over player1
  fn heuristic_value(&self) -> i32 {
    let mut total = 0;
    for cell in self.cells.iter() {
      match cell {
        Cell::Player1 => total += 1,
        Cell::Player2 => total -= 1,
        _ => ()
      }
    }
    total
  }

  fn piece_captures(&self, idx: usize) -> Vec<usize> {
    let mut captures = vec![];

    let cell_value = self.cells[idx];

    // left size
    let mut i = idx - 1;
    let mut captureGroup = vec![];
    while (idx != 0 && i+1 % self.width == 0){
      if (self.cells[i] == Cell::Empty) {
        break;
      }
      if (self.cells[i] == cell_value) {
        captures.append(&mut captureGroup);
        captureGroup = vec![];
        break;
      }
      captureGroup.push(i);
      i += 1;
    }

    if captureGroup.len() > 0 {
      let topX = i - self.width;
      let validTopCondition = topX < 0 || self.cells[topX] == cell_value;

      let bottomX = i + self.width;
      let validBottomCondition = bottomX > self.cells.len() || self.cells[bottomX] == cell_value;

      if (validBottomCondition && validTopCondition) {
        captures.append(&mut captureGroup);
      }
    }

    // right size
    let mut i = idx - 1;
    let mut captureGroup = vec![];
    while (i % self.width == 0){
      if (self.cells[i] == Cell::Empty) {
        break;
      }
      if (self.cells[i] == cell_value) {
        captures.append(&mut captureGroup);
        captureGroup = vec![];
        break;
      }
      captureGroup.push(i);
      i -= 1;
    }

    if captureGroup.len() > 0 {
      let topX = i - self.width;
      let validTopCondition = topX < 0 || self.cells[topX] == cell_value;

      let bottomX = i + self.width;
      let validBottomCondition = bottomX > self.cells.len() || self.cells[bottomX] == cell_value;

      if (validBottomCondition && validTopCondition) {
        captures.append(&mut captureGroup);
      }
    }


    // up side
    let mut i = idx - self.width;
    let mut captureGroup = vec![];
    while (idx >= self.width && i > 0){
      if (self.cells[i] == Cell::Empty) {
        break;
      }
      if (self.cells[i] == cell_value) {
        captures.append(&mut captureGroup);
        captureGroup = vec![];
        break;
      }
      captureGroup.push(i);
      if (i < self.width){
        break;
      }
      i -= self.width;
    }

    if captureGroup.len() > 0 {
      let leftIdx = i - 1;
      let validLeftCondition = leftIdx+1 % self.width == 0 || self.cells[leftIdx] == cell_value;

      let rightIdx = i + 1;
      let validRightCondition = rightIdx % self.width == 0 || self.cells[rightIdx] == cell_value;

      if (validLeftCondition && validRightCondition) {
        captures.append(&mut captureGroup);
      }
    }


    // down side
    let mut captureGroup = vec![];

    let mut i = idx;
    loop {
      i += self.width;
      if (i >= self.width * self.height) {
        break;
      }
      // No need to look in this direction anymore
      if (self.cells[i] == Cell::Empty) {
        break;
      }
      // Grab the capture group if present
      if (self.cells[i] == cell_value) {
        captures.append(&mut captureGroup);
        captureGroup = vec![];
        break;
      }
      // current index is part of a capture group
      captureGroup.push(i);
      if (i+self.width >= self.width * self.height) {
        break;
      }
    }

    if captureGroup.len() > 0 {
      let leftIdx = i - 1;
      let validLeftCondition = leftIdx >= self.width * self.height || leftIdx+1 % self.width == 0 || self.cells[leftIdx] == cell_value;

      let rightIdx = i + 1;
      let validRightCondition = rightIdx >= self.width * self.height || rightIdx % self.width == 0 || self.cells[rightIdx] == cell_value;

      if (validLeftCondition && validRightCondition) {
        captures.append(&mut captureGroup);
      }
    }



    captures
  }

  // get the current move options of a single piece on the board
  fn piece_move_options(&self, idx: usize) -> Vec<(usize, usize)> {
    let mut options: Vec<(usize, usize)> = vec![];

    // check available moves going up
    let mut result_idx = idx - self.width; // TODO make this a signed int
    while result_idx >= 0 && result_idx < self.width * self.height {
      if self.cells[result_idx] != Cell::Empty {
        break;
      }
      options.push((idx, result_idx));
      result_idx -= self.width;
    }

    // check available moves going down
    result_idx = idx + self.width;
    while result_idx < self.width * self.height {
      if self.cells[result_idx] != Cell::Empty {
        break;
      }
      options.push((idx, result_idx));
      result_idx += self.width;
    }

    // check available moves going left
    result_idx = idx - 1;
    while result_idx >= 0 && result_idx < self.width * self.height {
      if self.cells[result_idx] != Cell::Empty {
        break;
      }
      options.push((idx, result_idx));
      result_idx -= 1;
    }

    // check available moves going right
    result_idx = idx + 1;
    while result_idx % self.width != 0 {
      if self.cells[result_idx] != Cell::Empty {
        break;
      }
      options.push((idx, result_idx));
      result_idx -= 1;
    }

    options
  }

  // get the current move options of the whole board of Player2
  fn board_move_options(&self) -> Vec<(usize, usize)> {
    let mut options: Vec<(usize, usize)> = vec![];

    for (idx, cell) in self.cells.iter().enumerate() {
      match cell {
        Cell::Player2 => options.append(&mut self.piece_move_options(idx)),
        _ => ()
      }
    }

    options
  }

  fn clone(&self) -> ShogiGame {
    ShogiGame {
      width: self.width,
      height: self.height,
      cells: self.cells.to_vec()
    }
  }

}
