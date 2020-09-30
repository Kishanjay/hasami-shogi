<template>
  <div class="flex justify-center items-center min-h-screen bg-gray-400">
    <Shogiboard
      :board-state="boardState"
      :turn-team-id="turnTeamId"
      @move:piece="movePiece"
    />
  </div>
</template>

<script>
import Shogiboard from '@/components/Shogiboard/Shogiboard.vue';

import { moveBoardPiece } from '@/components/Shogiboard/Shogiboard.helper';

export default {
  components: {
    Shogiboard,
  },
  data() {
    return {
      boardState: [
        [1, 1, 1, 1, 1],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [2, 2, 2, 2, 2],
      ],
      turnTeamId: 2,
    };
  },
  metaInfo: {
    title: 'Lets play some Shogi!',
  },
  methods: {
    movePiece(origin, destination) {
      this.boardState = moveBoardPiece(this.boardState, origin, destination);
    },
  },
};
</script>
