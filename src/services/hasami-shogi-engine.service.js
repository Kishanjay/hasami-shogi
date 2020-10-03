// eslint-disable-next-line import/no-unresolved
// import { ShogiBoard } from 'hasami-shogi-engine';

// eslint-disable-next-line import/named
// import { memory } from 'hasami-shogi-engine/hasami_shogi_engine_bg.wasm';

import { memory } from '../wasm/hasami-shogi-engine/pkg/hasami_shogi_engine_bg.wasm';
import { ShogiGame } from '../wasm/hasami-shogi-engine/pkg/hasami_shogi_engine';

/**
 * Returns an array with arrays of the given size.
 *
 * @param myArray {Array} array to split
 * @param chunk_size {Integer} Size of every group
 */
function chunkArray(arr, chunkSize) {
  const result = [];

  for (let index = 0; index < arr.length; index += chunkSize) {
    const curChunk = Array.from(arr.slice(index, index + chunkSize));
    result.push(curChunk);
  }

  return result;
}

export function newGame(width, height) {
  const shogiGame = ShogiGame.new(width, height);
  return shogiGame;
}

export function getBoardState(shogiBoard) {
  const width = shogiBoard.width();
  const height = shogiBoard.height();

  const cellsPtr = shogiBoard.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
  const result = chunkArray(cells, width);
  return result;
}

export function computerMove(shogiBoard) {
  shogiBoard.computer_move();
}
