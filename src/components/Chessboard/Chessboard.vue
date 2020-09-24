<template>
  <div class="bg-white rounded p-10 flex flex-col">
    <ChessboardFile position="top" />
    <div class="flex">
      <ChessboardRank position="left" />
      <div class="border-2 border-black">
        <div v-for="row of chessboardSize" :key="row" class="flex">
          <ChessboardCell
            v-for="column of chessboardSize"
            :key="column"
            class="w-16 h-16"
            :class="[(column + row) % 2 === 1 ? 'bg-black' : 'bg-white']"
          />
        </div>
      </div>
      <ChessboardRank position="right" />
    </div>
    <ChessboardFile position="bottom" />
  </div>
</template>

<script>
import ChessboardCell from './ChessboardCell.vue';
import ChessboardFile from './ChessboardFile.vue';
import ChessboardRank from './ChessboardRank.vue';

const CHESSBOARD_SIZE = 8;

export default {
  components: {
    ChessboardCell,
    ChessboardFile,
    ChessboardRank,
  },
  data() {
    return {
      chessboardSize: CHESSBOARD_SIZE,
    };
  },
};
</script>
