<template>
  <section class="bg-white rounded flex flex-col">
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
              moveOption: selectedPieceMoveOptions[rowNumber][columnNumber],
              highlighted:
                highlightedFields &&
                highlightedFields.length &&
                highlightedFields.length > rowNumber &&
                highlightedFields[rowNumber].length &&
                highlightedFields[rowNumber].length > columnNumber &&
                highlightedFields[rowNumber][columnNumber],
            }"
            @click.native="cellClickHandler([rowNumber, columnNumber])"
          >
            <ShogiboardPiece v-if="cellValue" :team-id="cellValue" />
          </ShogiboardCell>
        </div>
      </div>
      <ShogiboardRank position="right" :number-of-rows="numberOfRows" />
    </div>
    <ShogiboardFile position="bottom" :number-of-columns="numberOfColumns" />
  </section>
</template>

<script>
import ShogiboardFile from './ShogiboardFile.vue';
import ShogiboardRank from './ShogiboardRank.vue';
import ShogiboardCell from './ShogiboardCell.vue';

import ShogiboardPiece from './ShogiboardPiece.vue';

import { getCell, canMoveTo, copyBoard, equalIndex } from './Shogiboard.helper';

export default {
  components: {
    ShogiboardCell,
    ShogiboardFile,
    ShogiboardRank,
    ShogiboardPiece,
  },
  inject: ['eventBus'],
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
  },
  data() {
    return {
      selectedPiece: null,
      selectedPieceMoveOptions: [],
    };
  },
  computed: {
    numberOfRows() {
      return this.boardState.length;
    },
    numberOfColumns() {
      return this.boardState[0].length;
    },
  },
  created() {
    this.calculateSelectedPieceMoveOptions();
    if (this.eventBus) {
      this.eventBus.$on('updateSelectedPiece', this.updateSelectedPiece);
    }
  },
  methods: {
    cellClickHandler(index) {
      // Logic related to moving a piece
      if (this.selectedPiece) {
        // Put down the picked up piece
        if (equalIndex(this.selectedPiece, index)) {
          this.updateSelectedPiece(null);
          return;
        }
        // Valid move
        if (getCell(this.selectedPieceMoveOptions, index)) {
          this.moveSelectedPiece(index);
          this.updateSelectedPiece(null);
          return;
        }
        // Invalid move
        this.updateSelectedPiece(null);
      }

      // Logic related to selecting a piece
      const cellValue = getCell(this.boardState, index);
      if (cellValue === this.movingPlayerId) {
        this.updateSelectedPiece(index);
      }
    },
    moveSelectedPiece(index) {
      this.$emit('move:piece', this.selectedPiece, index);
    },
    updateSelectedPiece(index) {
      if (!index || !getCell(this.boardState, index)) {
        this.selectedPiece = null;
      } else {
        this.selectedPiece = index;
      }
      this.calculateSelectedPieceMoveOptions();
    },

    calculateSelectedPieceMoveOptions() {
      if (!this.selectedPiece) {
        this.selectedPieceMoveOptions = copyBoard(this.boardState, false);
        return;
      }

      const result = [];
      for (let rowIndex = 0; rowIndex < this.numberOfRows; rowIndex += 1) {
        const row = [];
        for (
          let columnIndex = 0;
          columnIndex < this.numberOfColumns;
          columnIndex += 1
        ) {
          const destination = [rowIndex, columnIndex];
          row.push(canMoveTo(this.boardState, this.selectedPiece, destination));
        }
        result.push(row);
      }
      this.selectedPieceMoveOptions = result;
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
