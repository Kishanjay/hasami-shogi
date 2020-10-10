<template>
  <section class="bg-white rounded flex flex-col justify-between">
    <div>
      <header class="flex m-4 border-gray-100 border-b pb-4">
        <ComputerIcon /><span class="ml-2">Computer level 1</span>
      </header>

      <button
        v-if="moveHistory.length"
        class="bg-orange-400 text-white py-1 px-2 text-xs"
        @click="$emit('undo')"
      >
        Undo last move
      </button>

      <ol class="m-2">
        <li
          v-for="(move, index) in readibleMoveHistory"
          :key="index"
          class="flex items-center px-2"
          :class="{ 'bg-gray-100': index % 2 === 1 }"
        >
          {{ index + 1 }}.
          <span class="flex-grow font-bold text-sm pl-4">{{ move[0] }}</span>
          <span class="flex-grow font-bold text-sm pl-4">{{ move[1] }}</span>
        </li>
      </ol>
    </div>

    <footer>
      <div
        class="m-2 p-2 rounded border-black border-2 flex flex-col items-center"
      >
        <ShogiboardPiece :team-id="movingPlayerId" />
        <span class="font-bold">To play</span>
      </div>
    </footer>
  </section>
</template>

<script>
import ShogiboardPiece from '@/components/Shogiboard/ShogiboardPiece.vue';
import ComputerIcon from '@/components/Icons/ComputerIcon.vue';
import { indexToRankFile } from './Shogiboard.helper';

export default {
  components: {
    ShogiboardPiece,
    ComputerIcon,
  },
  props: {
    moveHistory: {
      required: false,
      default: () => [],
      type: Array,
    },
    movingPlayerId: {
      required: true,
      type: Number,
    },
  },
  computed: {
    readibleMoveHistory() {
      return this.moveHistory.map(([move]) => [
        indexToRankFile(move[0]),
        indexToRankFile(move[1]),
      ]);
    },
  },
};
</script>

<style></style>
