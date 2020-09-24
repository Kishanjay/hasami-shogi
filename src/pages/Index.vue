<template>
  <div class="flex justify-center items-center min-h-screen bg-gray-400">
    <Chessboard />
  </div>
</template>

<script>
import Chessboard from '@/components/Chessboard/Chessboard.vue';

export default {
  components: {
    Chessboard
  },
  metaInfo: {
    title: 'Hello, world!',
  },
};
</script>
