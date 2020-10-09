use hasami_shogi_engine::ShogiGame;
use hasami_shogi_engine::Cell;

// 0 2 0
// 0 1 0
// 0 0 2
#[test]
fn it_makes_the_best_move_as_cpu() {
  let board_state = vec![0, 2, 0, 0, 1, 0, 0, 0, 2];
  let mut sg = ShogiGame::import(3,3, board_state);
  sg.computer_move();

  assert_eq!(sg.cell(7), Cell::Player2);
}
