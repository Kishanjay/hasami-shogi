/* eslint-disable no-param-reassign */
/* eslint-disable no-await-in-loop */
/* eslint-disable camelcase */

/**
 * Demo file
 *
 * This file has been setup rapidly to provide a demo to the shogi game.
 * the code in this file should remain isolated from the general codebase
 * to prevent further polution.
 */
import {
  moveBoardPiece,
  setCell,
  copyBoard,
} from '@/components/Shogiboard/Shogiboard.helper';

export const initialMovingBoard = [
  [1, 2, 2, 0, 0, 2],
  [0, 0, 0, 1, 0, 0],
  [0, 2, 0, 2, 2, 0],
  [2, 0, 1, 0, 1, 1],
];

export const initialCapturingBoard = [
  [1, 0, 0, 0, 0],
  [0, 2, 2, 1, 0],
  [0, 0, 1, 0, 0],
];

export const initialCapturingBoard2 = [
  [2, 2, 0, 1, 0],
  [2, 0, 0, 0, 1],
  [1, 0, 0, 0, 2],
];

export const initialMovingBoard2 = [
  [0, 0, 0, 0, 0, 2],
  [0, 0, 0, 1, 0, 0],
  [2, 1, 1, 0, 1, 2],
];

async function timeout(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

function getHighlightedFieldsArray(board, fields) {
  const highlightedFields = copyBoard(board, false);

  if (fields) {
    for (const field of fields) {
      highlightedFields[field[0]][field[1]] = true;
    }
  }
  return highlightedFields;
}

export async function capturingBoardDemo(vm) {
  // Capture horizontally
  let capturingBoard_demo1 = moveBoardPiece(
    initialCapturingBoard,
    [0, 0],
    [0, 2]
  );
  capturingBoard_demo1 = setCell(capturingBoard_demo1, [1, 2], 0);

  // Capture vertically
  let capturingBoard_demo2 = moveBoardPiece(
    initialCapturingBoard,
    [0, 0],
    [1, 0]
  );
  capturingBoard_demo2 = setCell(capturingBoard_demo2, [1, 2], 0);
  capturingBoard_demo2 = setCell(capturingBoard_demo2, [1, 1], 0);

  // Capture corner
  let capturingBoard2_demo1 = moveBoardPiece(
    initialCapturingBoard2,
    [0, 3],
    [2, 3]
  );
  capturingBoard2_demo1 = setCell(capturingBoard2_demo1, [2, 4], 0);

  // Multi capture corner
  let capturingBoard2_demo2 = moveBoardPiece(
    initialCapturingBoard2,
    [0, 3],
    [0, 2]
  );
  capturingBoard2_demo2 = setCell(capturingBoard2_demo2, [0, 0], 0);
  capturingBoard2_demo2 = setCell(capturingBoard2_demo2, [0, 1], 0);
  capturingBoard2_demo2 = setCell(capturingBoard2_demo2, [1, 0], 0);

  // Moving into hasami
  const movingBoard2_demo1 = moveBoardPiece(
    initialMovingBoard2,
    [1, 3],
    [2, 3]
  );
  const movingBoard2_demo2 = moveBoardPiece(
    initialMovingBoard2,
    [1, 3],
    [1, 5]
  );

  // eslint-disable-next-line no-constant-condition
  while (true) {
    vm.capturingBoard = initialCapturingBoard;
    vm.capturingBoard_highlightedFields = null;
    vm.capturingBoard2 = capturingBoard2_demo1;
    vm.capturingBoard2_highlightedFields = getHighlightedFieldsArray(
      initialCapturingBoard2,
      [
        [0, 3],
        [2, 3],
      ]
    );
    vm.movingBoard2 = movingBoard2_demo1;
    vm.movingBoard2_highlightedFields = getHighlightedFieldsArray(
      initialMovingBoard2,
      [
        [1, 3],
        [2, 3],
      ]
    );
    await timeout(1500);

    vm.capturingBoard = capturingBoard_demo1;
    vm.capturingBoard_highlightedFields = getHighlightedFieldsArray(
      initialCapturingBoard,
      [
        [0, 0],
        [0, 2],
      ]
    );
    vm.capturingBoard2 = initialCapturingBoard2;
    vm.capturingBoard2_highlightedFields = null;
    vm.movingBoard2 = initialMovingBoard2;
    vm.movingBoard2_highlightedFields = null;
    await timeout(1500);

    vm.capturingBoard = initialCapturingBoard;
    vm.capturingBoard_highlightedFields = null;
    vm.capturingBoard2 = capturingBoard2_demo2;
    vm.capturingBoard2_highlightedFields = getHighlightedFieldsArray(
      initialCapturingBoard2,
      [
        [0, 3],
        [0, 2],
      ]
    );
    vm.movingBoard2 = movingBoard2_demo2;
    vm.movingBoard2_highlightedFields = getHighlightedFieldsArray(
      initialMovingBoard2,
      [
        [1, 3],
        [1, 5],
      ]
    );
    await timeout(1500);

    vm.capturingBoard = capturingBoard_demo2;
    vm.capturingBoard_highlightedFields = getHighlightedFieldsArray(
      initialCapturingBoard,
      [
        [0, 0],
        [1, 0],
      ]
    );
    vm.capturingBoard2 = initialCapturingBoard2;
    vm.capturingBoard2_highlightedFields = null;
    vm.movingBoard2 = initialMovingBoard2;
    vm.movingBoard2_highlightedFields = null;
    await timeout(1500);
  }
}
