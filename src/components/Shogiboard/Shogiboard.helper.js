export function getColumn(board, columnIndex) {
  const result = [];
  for (const row of board) {
    result.push(row[columnIndex]);
  }
  return result;
}

export function getRow(board, rowIndex) {
  return board[rowIndex];
}

export function getCell(board, [rowIndex, columnIndex]) {
  return board[rowIndex][columnIndex];
}

export function getWidth(board) {
  return board[0].length;
}

export function getHeight(board) {
  return board.length;
}

export function getRank(rowIndex) {
  const firstRankCharCode = 'A'.charCodeAt(0);
  const charCode = firstRankCharCode + rowIndex;
  return String.fromCharCode(charCode);
}

export function getRanks(numberOfRanks) {
  const ranks = [];
  for (let i = 0; i < numberOfRanks; i += 1) {
    ranks.push(getRank(i));
  }
  return ranks;
}

export function indexToRankFile([rowIndex, columnIndex]) {
  return getRank(rowIndex) + (columnIndex + 1);
}

/**
 * Helper function that compares 2 board indexes with eachother
 *
 * @param {Array} board The shogiBoard(state)
 * @param {Array} index1 Tupple of the [row, column]
 * @param {Array} index2 Tupple of the [row, column]
 */
export function equalIndex(index1, index2) {
  return index1[0] === index2[0] && index1[1] === index2[1];
}

/**
 * Can make a deep copy of the shogiBoard(state)
 *
 * @param {Array} board The shogiboard state
 * @param {String} filling Will replace the original value with this filler
 * value
 */
export function copyBoard(board, filling = undefined) {
  return board.map((row) =>
    row.map((val) => {
      if (filling !== undefined) {
        return filling;
      }
      return val;
    })
  );
}

/**
 * Updates the state of the board by changing the value of a cell. Since we
 * like to do functional programming, we prevent side-effects by by pointer
 * modifications. Therefore a new board will be returned.
 *
 * @param {Array} board The shogiboard state
 * @param {Array} index Index of the board
 * @param {Array} value Value it should become
 */
function setCell(board, index, value) {
  const [rowIndex, columnIndex] = index;
  const boardCopy = copyBoard(board);
  boardCopy[rowIndex][columnIndex] = value;

  return boardCopy;
}

/**
 * @deprecated The board state will be kept fully in the wasm memory
 *
 * @param {Array} board The shogiboard state
 * @param {Array} origin Index of the board
 * @param {Array} destination Index of the board
 */
export function moveBoardPiece(board, origin, destination) {
  let result = copyBoard(board);
  const cellValue = getCell(board, origin);
  result = setCell(result, origin, 0);
  result = setCell(result, destination, cellValue);

  return result;
}

/**
 * Helper function that will check for a row or column if there is a clear path
 * from pos1, to pos2
 *
 * @param {Array} arr array represenation of a row/column on a shogiboard.
 * @param {Number} pos1 digit representing a columnIndex on the row
 * @param {Number} pos2 digit representing a columnIndex on the row
 */
function hasFreePath(arr, pos1, pos2) {
  const start = Math.min(pos1, pos2);
  const end = Math.max(pos1, pos2);

  let count = 0;
  for (let i = start; i <= end; i += 1) {
    if (arr[i]) {
      count += 1;
      // We're expecting the origin column to have a piece, therefore finding
      // a value on the run doesn't mean it encountered a blocker. However,
      // finding 2 pieces does mean it is blocking.
    }
  }
  return count <= 1;
}

export function canMoveTo(board, origin, destination) {
  // Should start from non empty square and end at an empty square
  if (!getCell(board, origin) || getCell(board, destination)) {
    return false;
  }

  const [oRowIndex, oColumnIndex] = origin;
  const [dRowIndex, dColumnIndex] = destination;

  // Although this code can be 'optimised' by reducing the linesOfCode, this
  // part specifically is written verbose to improve readability and efforts
  // to comprehend behaviour
  if (oRowIndex === dRowIndex) {
    const row = getRow(board, oRowIndex);
    return hasFreePath(row, origin[1], destination[1]);
  }
  if (oColumnIndex === dColumnIndex) {
    const column = getColumn(board, oColumnIndex);
    return hasFreePath(column, origin[0], destination[0]);
  }

  return false;
}

/**
 * Extract which piece has moved based on 2 boardStates (right after eachother)
 * determines which piece was moved, from where and to where it was moved, but
 * also extracts whether the move has made a capture
 *
 * @param {*} previousBoardState
 * @param {*} currentBoardState
 * @returns {
 *  position: [<origin>, <destination>],
 *  player: integer,
 *  capture: boolean
 * }
 */
export function extractLastMove(previousBoardState, currentBoardState) {
  const width = getWidth(previousBoardState);
  const height = getHeight(previousBoardState);
  let destination;
  let origin;

  // determine a capture has happend based on these delta's
  let previousDelta = 0;
  let currentDelta = 0;

  for (let row = 0; row < height; row += 1) {
    for (let column = 0; column < width; column += 1) {
      const index = [row, column];

      const previousCellValue = getCell(previousBoardState, index);
      const currentCellValue = getCell(currentBoardState, index);

      if (previousCellValue === 1) {
        previousDelta += 1;
      } else if (previousCellValue === 2) {
        previousDelta -= 1;
      }
      if (currentCellValue === 1) {
        currentDelta += 1;
      } else if (currentCellValue === 2) {
        currentDelta -= 1;
      }

      if (previousCellValue !== 0 && currentCellValue === 0) {
        origin = index;
      } else if (previousCellValue === 0 && currentCellValue !== 0) {
        destination = index;
      }
    }
  }

  return {
    move: [origin, destination],
    player: getCell(currentBoardState, destination),
    capture: previousDelta === currentDelta,
  };
}
