<template>
  <div class="bg-white rounded p-10 flex flex-col">
    <div class="flex">
      <div class="w-8" />
      <div
        v-for="file of chessboardSize"
        :key="file"
        class="w-16 h-8 flex justify-center items-start"
      >
        {{ file }}
      </div>
    </div>
    <div class="flex">
      <div class="w-8">
        <div
          v-for="rank of ranks.slice().reverse()"
          :key="rank"
          class="h-16 flex justify-start items-center"
        >
          {{ rank }}
        </div>
      </div>
      <div class="border-2 border-black">
        <div
          v-for="row of chessboardSize"
          :key="row"
          class="flex"
        >
          <ChessboardCell
            v-for="column of chessboardSize"
            :key="column"
            class="w-16 h-16"
            :class="[(column+row) % 2 === 1 ? 'bg-black' : 'bg-white']"
          />
        </div>
      </div>
      <div class="w-8">
        <div
          v-for="rank of ranks.slice().reverse()"
          :key="rank"
          class="h-16 flex justify-end items-center"
        >
          {{ rank }}
        </div>
      </div>
    </div>
    <div class="flex">
      <div class="w-8" />
      <div
        v-for="file of chessboardSize"
        :key="file"
        class="w-16 h-8 flex justify-center items-end"
      >
        {{ file }}
      </div>
    </div>
  </div>
</template>

<script>
import ChessboardCell from './ChessboardCell.vue';

const CHESSBOARD_SIZE = 8;

export default {
  components: {
    ChessboardCell,
  },
  data() {
    const ranks = [];
    const firstRankCharCode = 'A'.charCodeAt(0);
    for (let charCode = firstRankCharCode; charCode < firstRankCharCode+CHESSBOARD_SIZE; charCode += 1){
      ranks.push(String.fromCharCode(charCode));
    }
    return {
      ranks,
      chessboardSize: CHESSBOARD_SIZE,
    }
  }
}
</script>
