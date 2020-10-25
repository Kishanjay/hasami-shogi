<template>
  <section class="relative bg-white rounded flex flex-col">
    <div v-if="victorPlayerId" class="px-2 pb-3">
      <strong>Gameover</strong>
      The {{ victorPlayerId === 1 ? 'blue' : 'red' }} player won the game!
    </div>
    <ShogiboardFile position="top" :number-of-columns="numberOfColumns" />
    <div class="flex">
      <ShogiboardRank position="left" :number-of-rows="numberOfRows" />
      <div class="border-2 border-black">
        <div
          v-for="(row, rowNumber) of boardState"
          :key="rowNumber"
          class="flex"
        >
          <ShogiboardCell
            v-for="(cellValue, columnNumber) of row"
            :key="columnNumber"
            class="relative w-16 h-16 border cursor-pointer"
            :class="{
              moveOption:
                selectedPiece &&
                selectedPieceMoveOptions[rowNumber][columnNumber],
              highlighted:
                highlightedFields && highlightedFields[rowNumber][columnNumber],
            }"
            @click="cellClickHandler([rowNumber, columnNumber])"
          >
            <ShogiboardPiece v-if="cellValue" :team-id="cellValue" />
          </ShogiboardCell>
        </div>
      </div>
      <ShogiboardRank position="right" :number-of-rows="numberOfRows" />
    </div>
    <ShogiboardFile position="bottom" :number-of-columns="numberOfColumns" />

    <div
      v-if="victorPlayerId"
      class="absolute bg-gray-200 bg-opacity-25 top-0 bottom-0 left-0 right-0"
    />
  </section>
</template>

<script>
import ShogiboardFile from './ShogiboardFile.vue';
import ShogiboardRank from './ShogiboardRank.vue';
import ShogiboardCell from './ShogiboardCell.vue';

import ShogiboardPiece from './ShogiboardPiece.vue';

import { getCell, equalIndex } from './Shogiboard.helper';

export default {
  components: {
    ShogiboardCell,
    ShogiboardFile,
    ShogiboardRank,
    ShogiboardPiece,
  },
  props: {
    boardState: {
      type: Array,
      required: true,
    },
    movingPlayerId: {
      type: Number,
      required: false,
      default: 1,
    },
    highlightedFields: {
      type: Array,
      required: false,
      default: null,
    },
    selectedPiece: {
      type: Array,
      required: false,
      default: null,
    },
    selectedPieceMoveOptions: {
      type: Array,
      required: false,
      default: null,
    },
    victorPlayerId: {
      type: Number,
      required: false,
      default: null,
    },
  },
  emits: ['move:selected-piece', 'update:selected-piece'],
  computed: {
    numberOfRows() {
      return this.boardState.length;
    },
    numberOfColumns() {
      return this.boardState[0].length;
    },
  },
  methods: {
    /**
     * Smart cell click handler function that correctly determines what should
     * happen based on the boardstate received from the props
     */
    cellClickHandler(index) {
      // when clicking on the movingplayers piece, select that piece
      if (getCell(this.boardState, index) === this.movingPlayerId) {
        this.$emit('update:selected-piece', index);
        return;
      }
      if (this.selectedPiece) {
        // clicking on the already selected piece
        if (equalIndex(this.selectedPiece, index)) {
          this.$emit('update:selected-piece', null);
          return;
        }

        // clicking on an invalid move option
        if (this.selectedPieceMoveOptions[index[0]][index[1]] !== true) {
          this.$emit('update:selected-piece', null);
          return;
        }

        // clicking on a valid move option
        // eslint-disable-next-line vue/custom-event-name-casing
        this.$emit('move:selected-piece', index);
        return;
      }
      // default
      this.$emit('update:selected-piece', null);
    },
  },
};
</script>

<style scoped>
.moveOption {
  @apply bg-gray-100;
  transition: 0.1s;
}
.moveOption::before {
  content: ' ';
  position: absolute;
  width: 8px;
  height: 8px;
  border-radius: 100%;
  @apply bg-gray-400;
  transition: 0.1s;
}
.moveOption:hover {
  @apply bg-gray-300;
}
.moveOption:hover::before {
  @apply bg-gray-500;
}
.highlighted {
  @apply bg-green-100;
}
</style>
