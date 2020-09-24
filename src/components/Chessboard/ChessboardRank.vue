<template>
  <div class="w-8">
    <div
      v-for="rank of ranks.slice().reverse()"
      :key="rank"
      class="h-16 flex items-center"
      :class="[position === 'left' ? 'justify-start' : 'justify-end']"
    >
      {{ rank }}
    </div>
  </div>
</template>

<script>
const CHESSBOARD_SIZE = 8;

export default {
  props: {
    position: {
      type: String,
      required: false,
      default: 'left',
      validate: (val) => ['left', 'right'].includes(val),
    },
  },
  data() {
    const ranks = [];
    const firstRankCharCode = 'A'.charCodeAt(0);
    for (
      let charCode = firstRankCharCode;
      charCode < firstRankCharCode + CHESSBOARD_SIZE;
      charCode += 1
    ) {
      ranks.push(String.fromCharCode(charCode));
    }
    return {
      ranks,
      chessboardSize: CHESSBOARD_SIZE,
    };
  },
};
</script>
