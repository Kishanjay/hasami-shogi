// eslint-disable-next-line import/no-unresolved
import { ShogiBoard } from 'hasami-shogi-engine';
// eslint-disable-next-line import/named
import { memory } from 'hasami-shogi-engine/hasami_shogi_engine_bg.wasm';

import { getHeight, getWidth } from '@/components/Shogiboard/Shogiboard.helper';

function rustSerializeBoardState(boardState) {
  return boardState.reduce((prev, cur) => prev.concat(cur), []);
}

function newGame(boardState) {
  const width = getWidth(boardState);
  const height = getHeight(boardState);

  const rustBoardState = rustSerializeBoardState(boardState);
  const shogiBoard = ShogiBoard.new(width, height, rustBoardState);
  const cellsPtr = shogiBoard.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  console.log({ cells });
  return shogiBoard;
}

export function getComputerMove(boardState) {
  const shogiBoard = newGame(boardState);
  shogiBoard.computer_move();
  return [
    [0, 1],
    [2, 2],
  ];
}

export function getGameBoard(boardState) {
  return boardState;
}
