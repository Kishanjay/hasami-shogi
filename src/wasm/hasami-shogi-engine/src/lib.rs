// Author: Kishan Nirghin
// Date: Oktober 2020
extern crate js_sys;

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

  pub fn cell(&self, idx: usize) -> Cell {
    self.cells[idx]
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
    let move_options = self.move_options();
    let move_option = self.best_move_option(move_options);

    self.move_piece(move_option.0, move_option.1);
  }

  pub fn player_move(&mut self, source_row: usize, source_column: usize, dest_row: usize, dest_column: usize) {
    let source_idx = self.get_index(source_row, source_column);
    let dest_idx = self.get_index(dest_row, dest_column);
    self.move_piece(source_idx, dest_idx);
  }
}

/** ****************************************************************************
 * Private helper functions
 * ****************************************************************************/
impl ShogiGame {
  // returns a clone of the current shogiGame object including its state
  fn clone(&self) -> ShogiGame {
    ShogiGame {
      width: self.width,
      height: self.height,
      cells: self.cells.to_vec()
    }
  }

  // The board state is stored as a single array, this helper function will
  // convert a <row,column> combination to the internal index for the element
  fn get_index (&self, row: usize, column: usize) -> usize {
    (row * self.width + column) as usize
  }

  fn column_idx(&self, idx: usize) -> usize {
    idx % self.width
  }

  fn row_idx(&self, idx: usize) -> usize {
    let column_idx = self.column_idx(idx);
    (idx - column_idx) / self.height
  }

  pub fn move_piece(&mut self, source_idx: usize, dest_idx: usize) {
    let cell_value = self.cells[source_idx];
    self.cells[source_idx] = Cell::Empty;
    self.cells[dest_idx] = cell_value;

    let mut captured_pieces = self.captured_pieces(dest_idx);
    for idx in captured_pieces.iter_mut() {
      self.cells[*idx] = Cell::Empty;
    }
  }

  // returns the pieces that will be captured when a piece is placed at idx
  fn captured_pieces(&self, idx: usize) -> Vec<usize> {
    let mut captures = vec![];

    let cell_value = self.cells[idx];

    captures.append(&mut self.capture_group(cell_value, idx, true, true).0);
    captures.append(&mut self.capture_group(cell_value, idx, true, false).0);
    captures.append(&mut self.capture_group(cell_value, idx, false, true).0);
    captures.append(&mut self.capture_group(cell_value, idx, false, false).0);

    captures
  }

  // returns a value indicating how good the passed move option is
  fn evaluate_move_option(&self, move_option: (usize, usize)) -> i32 {
    let mut shogi_game = self.clone();
    shogi_game.move_piece(move_option.0, move_option.1);
    shogi_game.heuristic_value()
  }

  // returns a value indicating how many pieces the cpu has over the player
  fn heuristic_value(&self) -> i32 {
    let mut total = 0;
    for cell in self.cells.iter() {
      match cell {
        Cell::Player1 => total -= 1,
        Cell::Player2 => total += 1,
        _ => ()
      }
    }
    total
  }

  // returns the move with the highest heuristic value given a set of moves
  // if there are multiple moves with the same heuristic value, one will be
  // chosen at random
  fn best_move_option(&self, mut move_options: Vec<(usize, usize)>) -> (usize, usize) {
    let mut best_heuristic = i32::MIN;
    let mut best_move = move_options[0];
    let mut best_moves = vec![best_move];

    for move_option in move_options.iter_mut() {
      let move_heuristic = self.evaluate_move_option(*move_option);

      if move_heuristic > best_heuristic {
        best_heuristic = move_heuristic;
        best_move = *move_option;
        best_moves = vec![best_move];
      } else if move_heuristic == best_heuristic {
        best_moves.push(*move_option);
      }
    }

    if best_moves.len() > 1 {
      let random_idx = (js_sys::Math::random() * best_moves.len() as f64).floor() as usize;
      log!("{:?}", random_idx);
      best_move = best_moves[random_idx];
    }

    best_move
  }

  // get the current move options of the whole board of Player2
  fn move_options(&self) -> Vec<(usize, usize)> {
    let mut options: Vec<(usize, usize)> = vec![];

    for (idx, cell) in self.cells.iter().enumerate() {
      match cell {
        Cell::Player2 => options.append(&mut self.piece_move_options(idx)),
        _ => ()
      }
    }

    options
  }


  // get the available move options of a single piece on the board
  fn piece_move_options(&self, idx: usize) -> Vec<(usize, usize)> {
    let mut options: Vec<(usize, usize)> = vec![];

    options.append(&mut self.piece_move_options_helper(idx, true, true));
    options.append(&mut self.piece_move_options_helper(idx, true, false));
    options.append(&mut self.piece_move_options_helper(idx, false, true));
    options.append(&mut self.piece_move_options_helper(idx, false, false));

    options
  }

  // the function responsible for defining the move options of a piece
  // these are all the moves the cpu player will take into account when making
  // its move.
  //
  // <start_idx>: the starting index from where the piece will move
  // <check_row>: search for moves in the horizontal direction
  // <inverse>: search for moves in the inverse direction
  //
  // returns: Vec: all possible moves in the format (start_idx, end_idx)
  fn piece_move_options_helper(&self, start_idx: usize, check_row: bool, inverse: bool) -> Vec<(usize, usize)> {
    let mut move_options: Vec<(usize, usize)> = vec![];
    // current row_index to be used with check_row
    let row_idx = self.row_idx(start_idx);
    // pre_computed board_size for efficiency
    let board_size = (self.width*self.height) as i32;

    let mut delta_idx = if check_row { 1 } else { self.width as i32};
    if inverse {
      delta_idx *= -1;
    }
    let mut idx = start_idx as i32;

    loop {
      idx += delta_idx;

      // prevent overflow errors
      if idx < 0 || idx >= board_size {
        break;
      }
      // when checking the row, respect the end condition
      if check_row && self.row_idx(idx as usize) != row_idx {
        break;
      }
      // cannot move to nor through a non-empty cell
      if self.cells[idx as usize] != Cell::Empty {
        break;
      }

      move_options.push((start_idx, idx as usize));
    }

    move_options
  }

  // the function responsible for defining when a piece should be captured
  // this is the main logic of the game.
  //
  // <cell_value>: the value of the starting color
  // <start_idx>: the starting index from where a capture group should be searched
  // <check_row>: search for the capture group in the horizontal direction
  // <inverse>: changes the delta_x to it's inverse (for going from right to
  //  left or down to up
  //
  // returns: (Vec, bool): all pieces that can be captured, and whether it ends
  //  on an empty tile. The later is necessary for the recursion step below
  fn capture_group(&self, cell_value: Cell, start_idx: usize, check_row: bool, inverse: bool) -> (Vec<usize>, bool) {
    // will store all pieces that can be captured with the move to idx
    let mut captures = vec![];
    // current row_index to be used with check_row
    let row_idx = self.row_idx(start_idx);
    // pre_computed board_size for efficiency
    let board_size = self.width*self.height;
    // helper variable that gets returned along with the capture group
    let mut ended_on_empty_value = false;

    let mut delta_idx = if check_row { 1 } else { self.width as i32};
    if inverse {
      delta_idx *= -1;
    }
    let mut idx = start_idx as i32;
    let mut capture_group = vec![];
    loop {
      idx += delta_idx;
      // prevent overflow errors
      if idx < 0 || idx >= board_size as i32 {
        break;
      }
      // when checking the row, respect the end condition
      if check_row && self.row_idx(idx as usize) != row_idx {
        break;
      }
      // if there is an empty value this capture group cannot be captured
      if self.cells[idx as usize] == Cell::Empty {
        capture_group = vec![];
        ended_on_empty_value = true;
        break;
      }
      // when encountering the search color, it means there is a successful
      // wrap of the capture group and thus it can be captured
      if self.cells[idx as usize] == cell_value {
        captures.append(&mut capture_group);
        capture_group = vec![];
        break;
      }
      capture_group.push(idx as usize);
    }
    // if the capture_group hasn't been captured, and the indexes are out of
    // bounds for the current row/column, we should check along the edges for
    // a bigger possible capture group
    if capture_group.len() > 0 {
      ended_on_empty_value = true;
      // we're starting at the last position that was within bounds
      let new_start_idx = (idx - delta_idx) as usize;
      // this returns the captures and whether it ended with invalid as a tupple
      let mut edge_captures_start = self.capture_group(cell_value, new_start_idx, !check_row, true);
      // if it didn't end with invalid we can continue checking the other side
      // of the current capture_group along the edge
      if !edge_captures_start.1 {
        let mut edge_captures_end = self.capture_group(cell_value, new_start_idx, !check_row, false);
        // if here also it didn't end with an empty tile, it means we can
        // capture the whole group
        if !edge_captures_end.1 {
          captures.append(&mut edge_captures_start.0);
          captures.append(&mut edge_captures_end.0);
          captures.append(&mut capture_group);
          ended_on_empty_value = false;
        }
      }
    }
    (captures, ended_on_empty_value)
  }
}
