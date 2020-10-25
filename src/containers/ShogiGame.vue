<template>
  <div class="flex" style="height: 640px">
    <Shogiboard
      v-if="shogiGame"
      class="mx-2 p-10"
      :board-state="boardState"
      :moving-player-id="movingPlayerId"
      :highlighted-fields="highlightedFields"
      :selected-piece="selectedPiece"
      :selected-piece-move-options="selectedPieceMoveOptions"
      :victor-player-id="victorPlayerId"
      @update:selected-piece="updateSelectedPiece"
      @move:selected-piece="playerMovePiece"
    />
    <ShogiboardLegend
      v-model:computer-level="computerLevel"
      class="mx-2"
      :move-history="moveHistory"
      :moving-player-id="movingPlayerId"
      :possible-computer-levels="possibleComputerLevel"
      :pvp-mode="pvpMode"
      :started="started"
      :victor-player-id="victorPlayerId"
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
import Shogiboard from '@/components/Shogiboard/Shogiboard.vue';
import ShogiboardLegend from '@/components/Shogiboard/ShogiboardLegend.vue';

import {
  extractLastMove,
  getWidth,
  getHeight,
  copyBoard,
  getMoveOptions,
  countCells,
} from '@/components/Shogiboard/Shogiboard.helper';

// to keep some logic at the child components an eventbus is used for
// some communication
const eventBus = null;

const hse = import('@/services/hasami-shogi-engine.service.js');

export default {
  components: {
    Shogiboard,
    ShogiboardLegend,
  },
  provide: {
    eventBus,
  },
  props: {
    pvpMode: {
      required: false,
      type: Boolean,
      default: false,
    },
  },
  emits: ['update:pvpMode'],
  data() {
    return {
      started: false,
      victorPlayerId: null,

      shogiGame: null,
      boardState: null,

      moveHistory: [],
      boardStateHistory: [],

      selectedPiece: null,
      selectedPieceMoveOptions: null,

      highlightedFields: undefined,

      movingPlayerId: 1,
      computerLevel: 3,
      possibleComputerLevel: [1, 2, 3, 4],

      cpuThinkingTimer: null,
      cpuLoading: false,

      hse: null, // linked hasami-shogi-engine
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
  created() {
    // wait for the wasm engine to be compiled and ready to run
    hse.then((r) => {
      this.hse = r;
      this.shogiGame = this.hse.newGame(8, 8);
      this.boardState = this.hse.getBoardState(this.shogiGame);
      this.setHighlightedFields(null);
    });
  },
  methods: {
    restart() {
      this.started = false;
      this.shogiGame = this.hse.newGame(8, 8);
      this.boardState = this.hse.getBoardState(this.shogiGame);
      this.setHighlightedFields(null);
      this.moveHistory = [];
      this.boardStateHistory = [];
      this.movingPlayerId = 1;
      clearTimeout(this.cpuThinkingTimer);
      this.selectedPiece = null;
      this.selectedPieceMoveOptions = null;
      this.victorPlayerId = null;
    },
    updateSelectedPiece(index) {
      if (this.victorPlayerId) {
        return;
      }
      this.selectedPiece = index;
      this.selectedPieceMoveOptions = getMoveOptions(
        this.boardState,
        this.selectedPiece
      );
    },
    playerMovePiece(destination) {
      if (this.victorPlayerId) {
        return;
      }
      this.started = true;
      this.shogiGame.player_move(...this.selectedPiece, ...destination);
      this.updateBoardState();
      this.movingPlayerId = this.movingPlayerId === 1 ? 2 : 1;

      this.selectedPiece = null;
      this.selectedPieceMoveOptions = null;

      if (this.checkAndGetFinished()) {
        return;
      }

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

        this.checkAndGetFinished();
      }, timeout);
    },
    /**
     * function that will update the boardstate based on the shogiGame instance
     * updating the boardstate here allows us to also compute the move that was
     * played by the cpu
     */
    updateBoardState() {
      const previousBoardState = this.boardState;
      const newBoardState = this.hse.getBoardState(this.shogiGame);
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
      this.playsound('back.wav');

      let lastMove = this.moveHistory.pop();
      let boardState = this.boardStateHistory.pop();
      if (lastMove.player === 2) {
        lastMove = this.moveHistory.pop();
        boardState = this.boardStateHistory.pop();
      }

      this.highlightedFields = lastMove.highlightedFields;
      this.shogiGame = this.hse.importGame(
        getWidth(boardState),
        getHeight(boardState),
        boardState
      );
      this.boardState = boardState;
      this.movingPlayerId = 1;
      this.selectedPiece = null;
      this.selectedPieceMoveOptions = null;
      this.victorPlayerId = null;
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
    /**
     * will check if the game has ended. Will update the victorPlayer variable
     * accordingly. Returns a boolean based on whether the game has ended.
     */
    checkAndGetFinished() {
      if (countCells(this.boardState, 1) <= 1) {
        this.victorPlayerId = 2;
        return true;
      }
      if (countCells(this.boardState, 2) <= 1) {
        this.victorPlayerId = 1;
        return true;
      }
      return false;
    },
  },
};
</script>

<style></style>
