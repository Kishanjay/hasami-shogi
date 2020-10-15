<template>
  <div class="flex" style="height: 640px">
    <Shogiboard
      v-if="shogiGame"
      class="mx-2 p-10"
      :board-state="boardState"
      :moving-player-id="movingPlayerId"
      :highlighted-fields="highlightedFields"
      @move:piece="playerMovePiece"
    />
    <ShogiboardLegend
      class="mx-2"
      :move-history="moveHistory"
      :moving-player-id="movingPlayerId"
      :computer-level.sync="computerLevel"
      :possible-computer-levels="possibleComputerLevel"
      :pvp-mode="pvpMode"
      :started="started"
      @update:pvpMode="(m) => $emit('update:pvpMode', m)"
      @undo="undo"
      @restart="restart"
    />
    <div
      v-if="cpuLoading"
      class="fixed top-0 bottom-0 right-0 left-0 cursor-wait pointer-events-auto"
    />
  </div>
</template>

<script>
import Vue from 'vue';
import Shogiboard from '@/components/Shogiboard/Shogiboard.vue';
import ShogiboardLegend from '@/components/Shogiboard/ShogiboardLegend.vue';

import {
  newGame,
  getBoardState,
  importGame,
} from '@/services/hasami-shogi-engine.service';
import {
  extractLastMove,
  getWidth,
  getHeight,
  copyBoard,
} from '@/components/Shogiboard/Shogiboard.helper';

// to keep some logic at the child components an eventbus is used for
// some communication
const eventBus = new Vue();

export default {
  components: {
    Shogiboard,
    ShogiboardLegend,
  },
  props: {
    pvpMode: {
      required: false,
      type: Boolean,
      default: false,
    },
  },
  data() {
    return {
      started: false,

      shogiGame: null,
      boardState: null,
      moveHistory: [],
      boardStateHistory: [],
      movingPlayerId: 1,
      computerLevel: 3,
      possibleComputerLevel: [1, 2, 3, 4],

      cpuThinkingTimer: null,
      highlightedFields: undefined,
      cpuLoading: false,
    };
  },
  watch: {
    pvpMode(pvpMode) {
      if (!pvpMode && this.movingPlayerId === 2) {
        this.movingPlayerId = 1;
        this.computerMovePiece();
      }
    },
  },
  provide: {
    eventBus,
  },
  created() {
    this.shogiGame = newGame(8, 8);
    this.boardState = getBoardState(this.shogiGame);
    this.setHighlightedFields();
  },
  methods: {
    restart() {
      this.started = false;
      this.shogiGame = newGame(8, 8);
      this.boardState = getBoardState(this.shogiGame);
      this.setHighlightedFields();
      this.moveHistory = [];
      this.boardStateHistory = [];
      this.movingPlayerId = 1;
      clearTimeout(this.cpuThinkingTimer);
      eventBus.$emit('updateSelectedPiece', null);
    },
    playerMovePiece(origin, destination) {
      this.started = true;
      this.shogiGame.player_move(...origin, ...destination);
      this.updateBoardState();
      this.movingPlayerId = this.movingPlayerId === 1 ? 2 : 1;

      if (!this.pvpMode) {
        this.computerMovePiece();
      }
    },
    computerMovePiece() {
      const cpuLevel = this.computerLevel;
      let timeout = 300;

      if (cpuLevel > 3) {
        this.cpuLoading = true; // for heavy functions (no pre-empting in js)
        timeout = 80; // minimal timeout to prevent audio from being interrupted
      }
      this.cpuThinkingTimer = setTimeout(() => {
        this.shogiGame.computer_move(cpuLevel);
        this.updateBoardState();
        this.movingPlayerId = 1;
        this.cpuLoading = false;
      }, timeout);
    },
    updateBoardState() {
      const previousBoardState = this.boardState;
      const newBoardState = getBoardState(this.shogiGame);
      const lastMove = extractLastMove(previousBoardState, newBoardState);

      const previousHighlightedFields = this.highlightedFields;
      this.setHighlightedFields(lastMove.move);

      this.boardStateHistory.push(previousBoardState);
      this.moveHistory.push({
        ...lastMove,
        highlightedFields: previousHighlightedFields,
      });
      this.boardState = newBoardState;

      if (lastMove.capture) {
        this.playsound('tock.wav');
      } else {
        this.playsound('tick.wav');
      }
    },
    undo() {
      clearTimeout(this.cpuThinkingTimer);
      eventBus.$emit('updateSelectedPiece', null);

      this.playsound('back.wav');

      let lastMove = this.moveHistory.pop();
      let boardState = this.boardStateHistory.pop();
      if (lastMove.player === 2) {
        lastMove = this.moveHistory.pop();
        boardState = this.boardStateHistory.pop();
      }

      this.highlightedFields = lastMove.highlightedFields;
      this.shogiGame = importGame(
        getWidth(boardState),
        getHeight(boardState),
        boardState
      );
      this.boardState = boardState;
      this.movingPlayerId = 1;
    },
    setHighlightedFields(fields) {
      const highlightedFields = copyBoard(this.boardState, false);

      if (fields) {
        for (const field of fields) {
          highlightedFields[field[0]][field[1]] = true;
        }
      }
      this.highlightedFields = highlightedFields;
    },
    playsound(name) {
      const audio = new Audio(`/assets/${name}`);
      audio.play();
    },
  },
};
</script>

<style></style>
