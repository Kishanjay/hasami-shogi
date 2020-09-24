<template>
  <div class="flex justify-center items-center min-h-screen bg-gray-400">
    <Chessboard :board-state="boardState" />
  </div>
</template>

<script>
import Chessboard from '@/components/Chessboard/Chessboard.vue';

export default {
  components: {
    Chessboard,
  },
  data() {
    return {
      boardState: [
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
      ],
    };
  },
  metaInfo: {
    title: 'Hello, world!',
  },
};
</script>
