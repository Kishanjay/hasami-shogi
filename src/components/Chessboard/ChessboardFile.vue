<template>
  <div class="flex">
    <div class="w-8" />
    <div
      v-for="file of chessboardSize"
      :key="file"
      class="w-16 h-8 flex justify-center"
      :class="[position === 'top' ? 'items-start' : 'items-end']"
    >
      {{ file }}
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
      default: 'top',
      validate: (val) => ['top', 'bottom'].includes(val),
    },
  },
  data() {
    return {
      chessboardSize: CHESSBOARD_SIZE,
    };
  },
};
</script>
