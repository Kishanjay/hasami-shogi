<template>
  <div class="bg-white rounded p-10 flex flex-col">
    <ChessboardFile position="top" :numberOfColumns="numberOfColumns" />
    <div class="flex">
      <ChessboardRank position="left" :numberOfRows="numberOfRows" />
      <div class="border-2 border-black">
        <div v-for="row of numberOfRows" :key="row" class="flex">
          <ChessboardCell
            v-for="column of numberOfColumns"
            :key="column"
            class="w-16 h-16"
            :class="[(column + row) % 2 === 1 ? 'bg-black' : 'bg-white']"
          />
        </div>
      </div>
      <ChessboardRank position="right" :numberOfRows="numberOfRows" />
    </div>
    <ChessboardFile position="bottom" :numberOfColumns="numberOfColumns" />
  </div>
</template>

<script>
import ChessboardCell from './ChessboardCell.vue';
import ChessboardFile from './ChessboardFile.vue';
import ChessboardRank from './ChessboardRank.vue';

export default {
  props: {
    boardState: {
      type: Array,
      required: true,
    },
  },
  computed: {
    numberOfRows() {
      return this.boardState.length;
    },
    numberOfColumns() {
      return this.boardState[0].length;
    },
  },
  components: {
    ChessboardCell,
    ChessboardFile,
    ChessboardRank,
  },
};
</script>
