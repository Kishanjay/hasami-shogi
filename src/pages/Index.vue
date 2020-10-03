<template>
  <div class="flex justify-center items-center min-h-screen bg-gray-400">
    <Shogiboard
      :board-state="boardState"
      :moving-player-id="movingPlayerId"
      :allow-move-piece="allowMovePiece"
      @move:piece="movePiece"
    />
  </div>
</template>

<script>
import Shogiboard from '@/components/Shogiboard/Shogiboard.vue';

import { moveBoardPiece } from '@/components/Shogiboard/Shogiboard.helper';

import { getComputerMove } from '@/services/hasami-shogi-engine.service';

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
      movingPlayerId: 2,
    };
  },
  metaInfo: {
    title: 'Lets play some Shogi!',
  },
  computed: {
    allowMovePiece() {
      return this.movingPlayerId === 2;
    },
  },
  watch: {
    allowMovePiece(allowMovePiece) {
      if (!allowMovePiece) {
        this.computerMovePiece();
      }
    },
  },
  created() {
    if (!this.allowMovePiece) {
      this.computerMovePiece();
    }
  },
  methods: {
    movePiece(origin, destination) {
      this.boardState = moveBoardPiece(this.boardState, origin, destination);
      this.movingPlayerId = 1;
    },
    computerMovePiece() {
      const [origin, destination] = getComputerMove(this.boardState);
      this.boardState = moveBoardPiece(this.boardState, origin, destination);
      this.movingPlayerId = 2;
    },
  },
};
</script>
