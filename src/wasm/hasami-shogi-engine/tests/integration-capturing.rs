// extern crate hasami_shogi_engine;

use hasami_shogi_engine::ShogiGame;
use hasami_shogi_engine::Cell;

// // 0 1 2
// // 3 4 5
// // 6 7 8

#[test]
fn it_doesnt_invalid_capture_whole_row() {
  let mut sg = ShogiGame::new(3,3);
  sg.move_piece(6, 3);

  assert_eq!(sg.cell(0), Cell::Player2);
  assert_eq!(sg.cell(1), Cell::Player2);
  assert_eq!(sg.cell(2), Cell::Player2);
  assert_eq!(sg.cell(3), Cell::Player1);

  assert_eq!(sg.cell(7), Cell::Player1);
  assert_eq!(sg.cell(8), Cell::Player1);
}

#[test]
fn it_removes_a_captured_piece_on_the_left() {
  let mut sg = ShogiGame::new(3,3);
  sg.move_piece(6, 3);
  sg.move_piece(1, 4);
  sg.move_piece(8, 5);

  assert_eq!(sg.cell(4), Cell::Empty);
  assert_eq!(sg.cell(3), Cell::Player1);
  assert_eq!(sg.cell(5), Cell::Player1);
}

#[test]
fn it_removes_a_captured_piece_on_the_right() {
  let mut sg = ShogiGame::new(3,3);
  sg.move_piece(8, 5);
  sg.move_piece(1, 4);
  sg.move_piece(6, 3);

  assert_eq!(sg.cell(4), Cell::Empty);
  assert_eq!(sg.cell(3), Cell::Player1);
  assert_eq!(sg.cell(5), Cell::Player1);
}

// 0 0 1
// 1 0 2
// 0 0 1
#[test]
fn it_considers_middle_edges_on_the_right() {
  let board_state = vec![0, 0, 1, 1, 0, 2, 0, 0, 1];
  let mut sg = ShogiGame::import(3,3, board_state);
  sg.move_piece(3, 4);

  assert_eq!(sg.cell(5), Cell::Empty);
  assert_eq!(sg.cell(2), Cell::Player1);
  assert_eq!(sg.cell(4), Cell::Player1);
  assert_eq!(sg.cell(8), Cell::Player1);
}

// 0 1 2
// 0 0 0
// 0 0 1
#[test]
fn it_considers_top_edges_on_the_right() {
  let board_state = vec![0, 1, 2, 0, 0, 0, 0, 0, 1];
  let mut sg = ShogiGame::import(3,3, board_state);
  sg.move_piece(8, 5);

  assert_eq!(sg.cell(2), Cell::Empty);
  assert_eq!(sg.cell(1), Cell::Player1);
  assert_eq!(sg.cell(5), Cell::Player1);
}

// 2 1 0
// 0 0 0
// 1 0 0
#[test]
fn it_considers_top_edges_on_the_left() {
  let board_state = vec![2, 1, 0, 0, 0, 0, 1, 0, 0];
  let mut sg = ShogiGame::import(3,3, board_state);
  sg.move_piece(6, 3);

  assert_eq!(sg.cell(0), Cell::Empty);
  assert_eq!(sg.cell(1), Cell::Player1);
  assert_eq!(sg.cell(3), Cell::Player1);
}

// 0 0 1
// 0 0 0
// 0 1 2
#[test]
fn it_considers_bottom_edges_on_the_right() {
  let board_state = vec![0, 0, 1, 0, 0, 0, 0, 1, 2];
  let mut sg = ShogiGame::import(3,3, board_state);
  sg.move_piece(2, 5);


  assert_eq!(sg.cell(8), Cell::Empty);
  assert_eq!(sg.cell(7), Cell::Player1);
  assert_eq!(sg.cell(5), Cell::Player1);
}

// 1 0 0
// 0 0 0
// 2 1 0
#[test]
fn it_considers_bottom_edges_on_the_left() {
  let board_state = vec![1, 0, 0, 0, 0, 0, 2, 1, 0];
  let mut sg = ShogiGame::import(3,3, board_state);
  sg.move_piece(0, 3);


  assert_eq!(sg.cell(6), Cell::Empty);
  assert_eq!(sg.cell(3), Cell::Player1);
  assert_eq!(sg.cell(7), Cell::Player1);
}

// 0 1 0
// 0 2 0
// 1 0 0
#[test]
fn it_removes_a_captured_piece_on_the_topside() {
  let board_state = vec![0, 1, 0, 0, 2, 0, 1, 0, 0];
  let mut sg = ShogiGame::import(3,3, board_state);
  sg.move_piece(6, 7);

  assert_eq!(sg.cell(4), Cell::Empty);
  assert_eq!(sg.cell(7), Cell::Player1);
  assert_eq!(sg.cell(1), Cell::Player1);
}

// 0 0 1
// 0 2 0
// 0 1 0
#[test]
fn it_removes_a_captured_piece_on_the_bottomside() {
  let board_state = vec![0, 0, 1, 0, 2, 0, 0, 1, 0];
  let mut sg = ShogiGame::import(3,3, board_state);
  sg.move_piece(2, 1);

  assert_eq!(sg.cell(4), Cell::Empty);
  assert_eq!(sg.cell(7), Cell::Player1);
  assert_eq!(sg.cell(1), Cell::Player1);
}



#[test]
fn it_removes_the_whole_capture_group_on_the_right() {
  let mut sg = ShogiGame::new(4,4);
  sg.move_piece(1, 5);
  sg.move_piece(12, 4);
  sg.move_piece(2, 6);
  sg.move_piece(15, 7);

  assert_eq!(sg.cell(5), Cell::Empty);
  assert_eq!(sg.cell(6), Cell::Empty);
  assert_eq!(sg.cell(4), Cell::Player1);
  assert_eq!(sg.cell(7), Cell::Player1);
}

#[test]
fn it_removes_the_whole_capture_group_on_the_right_edge() {
  let mut sg = ShogiGame::new(5,5);
  sg.move_piece(0, 10);
  sg.move_piece(24, 9);
  sg.move_piece(10, 11);
  sg.move_piece(20, 0);

  assert_eq!(sg.cell(1), Cell::Empty);
  assert_eq!(sg.cell(2), Cell::Empty);
  assert_eq!(sg.cell(3), Cell::Empty);
  assert_eq!(sg.cell(4), Cell::Empty);

  assert_eq!(sg.cell(0), Cell::Player1);
  assert_eq!(sg.cell(9), Cell::Player1);

  assert_eq!(sg.cell(11), Cell::Player2);
}
