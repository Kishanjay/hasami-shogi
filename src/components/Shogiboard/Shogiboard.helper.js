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

/**
 * Helper function that compares 2 board indexes with eachother
 *
 * @param {Array} board The shogiBoard(state)
 * @param {Array} index1 Tupple of the [row, column]
 * @param {Array} index2 Tupple of the [row, column]
 */
export function equalIndex(board, index1, index2) {
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
