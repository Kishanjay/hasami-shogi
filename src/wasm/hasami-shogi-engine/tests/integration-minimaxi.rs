use hasami_shogi_engine::ShogiGame;
use hasami_shogi_engine::Cell;
use hasami_shogi_engine::mini_maxi;

#[test]
fn it_correctly_thinks_1_moves_ahead() {
  // 2 1 0
  // 0 1 0
  // 2 0 2
  let board_state = vec![2, 1, 0, 0, 1, 0, 2, 0, 2];
  let mut sg = ShogiGame::import(3,3, board_state);
  let game_move = sg.calculate_best_move(Cell::Player2, 1, false);

  assert_eq!(8, game_move.origin_idx);
  assert_eq!(2, game_move.destination_idx);


  // 2 2 0 0
  // 2 0 0 1
  // 2 0 0 1
  // 0 0 1 2
  let board_state = vec![2, 2, 0, 0, 2, 0, 0, 1, 2, 0, 0, 1, 0, 0, 1, 2];
  let mut sg = ShogiGame::import(4,4, board_state);
  let game_move = sg.calculate_best_move(Cell::Player2, 1, false);

  assert_eq!(1, game_move.origin_idx);
  assert_eq!(3, game_move.destination_idx);
}


#[test]
fn it_correctly_thinks_2_moves_ahead() {
  // 1 0 0
  // 1 2 1
  // 0 2 2
  let board_state = vec![1, 0, 0, 1, 2, 1, 0, 2, 2];
  let mut sg = ShogiGame::import(3,3, board_state);
  let game_move = sg.calculate_best_move(Cell::Player2, 2, false);

  assert_eq!(7, game_move.origin_idx);
  assert_eq!(6, game_move.destination_idx);


  // 0 1 1 0
  // 0 2 0 0
  // 0 1 0 1
  // 0 0 0 0
  let board_state = vec![0, 1, 1, 0, 0, 2, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0];
  let mut sg = ShogiGame::import(4,4, board_state);
  let game_move = sg.calculate_best_move(Cell::Player2, 2, false);

  assert_eq!(5, game_move.origin_idx);
  assert_eq!(4, game_move.destination_idx);
}


#[test]
fn it_correctly_thinks_3_moves_ahead() {
  // 0 1 1 1 0
  // 0 1 0 1 0
  // 0 0 0 0 0
  // 0 0 0 0 0
  // 2 2 2 2 2
  let board_state = vec![0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2];
  let mut sg = ShogiGame::import(5,5, board_state);
  let game_move = sg.calculate_best_move(Cell::Player2, 3, false);

  assert_eq!(22, game_move.origin_idx);
  assert_eq!(7, game_move.destination_idx);

  let mut sg = ShogiGame::new(8,8);
  sg.move_piece(58, 42);
  sg.move_piece(0, 8);
  sg.move_piece(60, 44);
  let game_move = sg.calculate_best_move(Cell::Player2, 3, false);

  assert_eq!(3, game_move.origin_idx);
  assert_eq!(43, game_move.destination_idx);

  let mut sg = ShogiGame::new(8,8);
  sg.move_piece(58, 42);
  sg.move_piece(6, 54);
  sg.move_piece(60, 44);
  let game_move = sg.calculate_best_move(Cell::Player2, 3, false);

  assert_eq!(3, game_move.origin_idx);
  assert_eq!(43, game_move.destination_idx);

  let mut sg = ShogiGame::new(8,8);
  sg.move_piece(58, 42);
  sg.move_piece(6, 14);
  sg.move_piece(60, 44);
  sg.computer_move(3);

  assert_eq!(sg.cell(43), Cell::Player2);

  let mut sg = ShogiGame::new(8,8);
  sg.move_piece(58, 42);
  sg.move_piece(6, 14);
  sg.move_piece(60, 44);
  sg.move_piece(3, 43);

  let h = mini_maxi(&mut sg, false, 2, i32::MIN, i32::MAX);
  assert_eq!(h, 1);
}
