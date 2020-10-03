<template>
  <div class="flex justify-center items-center min-h-screen bg-gray-400">
    <Shogiboard
      v-if="shogiGame"
      :board-state="boardState"
      @move:piece="movePiece"
    />
  </div>
</template>

<script>
import Shogiboard from '@/components/Shogiboard/Shogiboard.vue';

import { newGame, getBoardState } from '@/services/hasami-shogi-engine.service';

export default {
  components: {
    Shogiboard,
  },
  data() {
    return {
      shogiGame: null,
      boardState: null,
    };
  },
  metaInfo: {
    title: 'Lets play some Shogi!',
  },
  created() {
    this.shogiGame = newGame(8, 8);
    this.boardState = getBoardState(this.shogiGame);
  },
  methods: {
    movePiece(origin, destination) {
      const sourceIdx = this.shogiGame.get_index(origin[0], origin[1]);
      const destIdx = this.shogiGame.get_index(destination[0], destination[1]);
      this.shogiGame.move_piece(sourceIdx, destIdx);

      this.shogiGame.computer_move();
      this.boardState = getBoardState(this.shogiGame);
    },
  },
};
</script>
