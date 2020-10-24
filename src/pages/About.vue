<template>
  <Layout>
    <article>
      <h1 class="text-gray-500 text-2xl font-bold py-2 px-1">About</h1>

      <section class="bg-white p-4 rounded max-w-4xl mb-4">
        <h1 class="text-xl mb-2">Backstory</h1>
        <p class="mb-2">
          Hasami shogi (はさみ将棋 hasami shōgi, "intercepting chess") is a
          variant of shogi (Japanese chess). The game has two main variants, and
          all Hasami variants, unlike other shogi variants, use only one type of
          piece, and the winning objective is not checkmate.
        </p>
        <p class="mb-2">
          One main variant involves capturing all but one of the opponent's men;
          the other involves building an unbroken vertical or horizontal chain
          of five-in-a-row. Hasami shogi possesses simple rules while offering
          complex strategy.
        </p>
        <i>
          Source:
          <a href="https://en.wikipedia.org/wiki/Hasami_shogi">WikiPedia</a>
        </i>
      </section>

      <section class="bg-white p-4 rounded max-w-4xl">
        <h1 class="text-xl mb-2">How to play</h1>
        <div class="flex items-stretch border p-8">
          <div class="flex flex-col justify-between py-5 mr-6">
            <div>
              <h2 class="text-md mb-1 font-bold">1. Moving</h2>
              <p class="mb-4">
                All pieces move the same as a rook in chess. (That is, any
                number of empty cells vertically or horizontally.) A move
                consists of moving a piece to an empty cell of the board. As in
                shogi there is no is no jumping, so a piece can move no further
                than adjacent to a friendly or enemy piece in its path.
              </p>
            </div>
            <p class="mb-2 italic font-bold text-sm text-gray-800">
              Click on the blue pieces to see how they move
            </p>
          </div>

          <ShogiBoard
            :board-state="movingBoard"
            :selected-piece="movingBoardSelectedPiece"
            :selected-piece-move-options="movingBoardSelectedPieceMoveOptions"
            @move:selected-piece="moveSelectedMovingBoardPiece"
            @update:selected-piece="updateMovingBoardSelectedPiece"
          />
        </div>

        <div class="border px-4">
          <div class="flex items-stretch px-8 pt-8">
            <ShogiBoard
              class="pb-2"
              :board-state="capturingBoard"
              :moving-player-id="0"
              :highlighted-fields="capturingBoard_highlightedFields"
            />
            <div class="py-4 ml-8">
              <h2 class="text-md mb-1 font-bold">2. Capturing</h2>
              <p class="mb-2">
                Capturing An opponent's piece is captured using the custodian
                method: the player occupies the two cells adjacent to the piece
                either horizontally (on a rank) or vertically (on a file).
              </p>
              <p class="mb-8">
                Multiple pieces can be captured in a single move if all between
                the capturing player's two pieces are filled by enemy men.
              </p>
            </div>
          </div>
          <div class="flex items-stretch px-8 pb-8">
            <ShogiBoard
              class="pt-2"
              :board-state="capturingBoard2"
              :moving-player-id="0"
              :highlighted-fields="capturingBoard2_highlightedFields"
            />
            <p class="py-4 pt-6 ml-8 mb-2">
              An enemy piece in a corner cell can be captured by occupying the
              two cells that orthogonally surround it. Captured pieces are
              removed from the game.
            </p>
          </div>
        </div>
        <div class="flex items-stretch border p-8">
          <div class="flex flex-col justify-between py-5 mr-6">
            <div>
              <h2 class="text-md mb-1 font-bold">3. Exceptions</h2>
              <p class="mb-4">
                A player can safely move a piece to a cell between two enemy
                pieces without being captured. Likewise, it is safe to move a
                piece to complete a chain of friendly pieces flanked by two
                pieces—none of the "sandwiched" pieces are captured.
              </p>
            </div>
          </div>

          <ShogiBoard
            :board-state="movingBoard2"
            :highlighted-fields="movingBoard2_highlightedFields"
          />
        </div>

        <div class="border p-16 text-center">
          <h2 class="text-md mb-1 font-bold">4. Play and WIN</h2>
          <p class="mb-8">
            A player wins by capturing all but one of their opponent's pieces.
          </p>
          <a
            href="/"
            class="bg-green-500 hover:bg-green-400 text-white font-bold py-4 px-8 border-b-4 border-green-700 hover:border-green-500 rounded outline-none focus:outline-none"
          >
            Lets get started
          </a>
        </div>

        <p class="mb-2" />
      </section>
    </article>
  </Layout>
</template>

<script>
import Layout from '@/layouts/Default.vue';
import ShogiBoard from '@/components/Shogiboard/Shogiboard.vue';
import {
  getMoveOptions,
  moveBoardPiece,
} from '@/components/Shogiboard/Shogiboard.helper';

import {
  initialMovingBoard,
  initialCapturingBoard,
  initialMovingBoard2,
  initialCapturingBoard2,
  capturingBoardDemo,
} from '@/services/demo.service';

export default {
  metaInfo: {
    title: 'About Hasami Shogi',
  },
  components: {
    Layout,
    ShogiBoard,
  },
  data() {
    return {
      movingBoard: initialMovingBoard,
      movingBoardSelectedPiece: null,
      movingBoardSelectedPieceMoveOptions: null,

      capturingBoard: initialCapturingBoard,
      capturingBoard_highlightedFields: null,
      capturingBoard2: initialCapturingBoard2,
      capturingBoard2_highlightedFields: null,
      movingBoard2: initialMovingBoard2,
    };
  },
  created() {
    capturingBoardDemo(this);
  },
  methods: {
    updateMovingBoardSelectedPiece(index) {
      this.movingBoardSelectedPiece = index;
      this.movingBoardSelectedPieceMoveOptions = getMoveOptions(
        this.movingBoard,
        this.movingBoardSelectedPiece
      );
    },
    moveSelectedMovingBoardPiece(destination) {
      this.movingBoard = moveBoardPiece(
        this.movingBoard,
        this.movingBoardSelectedPiece,
        destination
      );
      this.movingBoardSelectedPiece = null;
      this.movingBoardSelectedPieceMoveOptions = null;
    },
  },
};
</script>
