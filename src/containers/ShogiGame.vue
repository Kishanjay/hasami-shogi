<template>
  <div class="flex">
    <Shogiboard
      v-if="shogiGame"
      class="mx-2"
      :board-state="boardState"
      :moving-player-id="movingPlayerId"
      @move:piece="playerMovePiece"
    />
    <ShogiboardLegend
      class="mx-2"
      :move-history="moveHistory"
      :moving-player-id="movingPlayerId"
      @undo="undo"
    />
  </div>
</template>

<script>
import Shogiboard from '@/components/Shogiboard/Shogiboard.vue';
import ShogiboardLegend from '@/components/Shogiboard/ShogiboardLegend.vue';

import {
  newGame,
  getBoardState,
  importGame,
} from '@/services/hasami-shogi-engine.service';
import {
  getMovedPiece,
  getWidth,
  getHeight,
} from '@/components/Shogiboard/Shogiboard.helper';

export default {
  components: {
    Shogiboard,
    ShogiboardLegend,
  },
  data() {
    return {
      shogiGame: null,
      boardState: null,
      moveHistory: [],
      boardStateHistory: [],
      movingPlayerId: 1,

      cpuThinkingTimer: null,
    };
  },
  created() {
    this.shogiGame = newGame(8, 8);
    this.boardState = getBoardState(this.shogiGame);
  },
  methods: {
    playerMovePiece(origin, destination) {
      this.shogiGame.player_move(...origin, ...destination);
      this.update_board_state();
      this.movingPlayerId = 2;

      this.cpuThinkingTimer = setTimeout(() => {
        this.computerMovePiece();
      }, 500);
    },
    computerMovePiece() {
      this.shogiGame.computer_move();
      this.update_board_state();
      this.movingPlayerId = 1;
    },
    update_board_state() {
      const previousBoardState = this.boardState;
      const newBoardState = getBoardState(this.shogiGame);

      this.boardStateHistory.push(previousBoardState);
      this.moveHistory.push(getMovedPiece(previousBoardState, newBoardState));
      this.boardState = newBoardState;
    },
    undo() {
      clearTimeout(this.cpuThinkingTimer);

      const lastMovePlayerId = this.moveHistory.pop()[1];
      let boardState = this.boardStateHistory.pop();
      if (lastMovePlayerId === 2) {
        this.moveHistory.pop();
        boardState = this.boardStateHistory.pop();
      }

      this.shogiGame = importGame(
        getWidth(boardState),
        getHeight(boardState),
        boardState
      );
      this.boardState = boardState;
      this.movingPlayerId = 1;
    },
  },
};
</script>

<style></style>
