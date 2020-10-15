<template>
  <section class="bg-white rounded flex flex-col" style="min-width: 250px">
    <header class="p-4 pb-2 border-gray-200 border-b">
      <div v-if="!pvpMode" class="flex pb-3">
        <ComputerIcon />
        <span class="ml-2"> Computer level {{ computerLevel }} </span>
      </div>
      <div v-else class="flex pb-3">
        <PeopleIcon />
        <span class="ml-2"> Offline PvP </span>
      </div>

      <div class="flex justify-between">
        <button
          class="transition duration-200 transform"
          :class="[
            moveHistory.length
              ? 'hover:text-red-600 hover:-rotate-45'
              : 'opacity-25 cursor-not-allowed',
          ]"
          :disabled="!moveHistory.length"
          @click="$emit('undo')"
        >
          <UndoIcon class="w-4 h-4" />
        </button>
        <button
          class="hover:text-red-600 transition duration-200 transform hover:rotate-45"
          :class="{ 'text-red-700': settingsOpened || !started }"
          @click="toggleSettingsOpened"
        >
          <SettingsIcon class="w-4 h-4" />
        </button>
      </div>
    </header>
    <section
      v-if="settingsOpened || !started"
      class="border-2 border-red-100 bg-gray-100 p-4"
    >
      <h2 class="text-sm font-bold mb-3">Settings</h2>
      <div class="mb-2">
        <h3 class="text-xs text-gray-600 mb-1">Offline mode</h3>
        <div class="flex">
          <button
            class="w-16 h-10 flex items-center justify-center mr-2"
            :class="[
              !pvpMode
                ? 'border-2 border-black text-black bg-white'
                : 'border border-gray-400 text-gray-400',
            ]"
            @click="$emit('update:pvpMode', false)"
          >
            <ComputerIcon class="w-4 h-4" />
          </button>
          <button
            class="w-16 h-10 flex items-center justify-center mr-2"
            :class="[
              pvpMode
                ? 'border-2 border-black text-black bg-white'
                : 'border border-gray-400 text-gray-400',
            ]"
            @click="$emit('update:pvpMode', true)"
          >
            <PeopleIcon class="w-4 h-4" />
          </button>
        </div>
      </div>
      <div v-if="!pvpMode">
        <h3 class="text-xs text-gray-600 mb-1">Computer level</h3>
        <div class="flex">
          <button
            v-for="possibleComputerLevel in possibleComputerLevels"
            :key="possibleComputerLevel"
            class="mr-2 w-10 h-10 flex items-center justify-center"
            :class="[
              possibleComputerLevel === computerLevel
                ? 'border-2 border-black text-black bg-white'
                : 'border border-gray-400 text-gray-400',
            ]"
            @click="
              $emit(
                'update:computerLevel',
                possibleComputerLevel === computerLevel
                  ? 0
                  : possibleComputerLevel
              )
            "
          >
            {{ possibleComputerLevel }}
          </button>
        </div>
      </div>

      <button
        v-if="started"
        class="bg-orange-400 text-white py-1 px-2 text-xs mt-6"
        @click="restart"
      >
        Restart game
      </button>
      <div v-else class="text-sm mt-6 italic text-gray-700">
        Move a piece to start...
      </div>
    </section>

    <section ref="moveHistory" class="flex-grow overflow-y-scroll">
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
    </section>
    <footer class="flex flex-col">
      <div
        class="m-2 p-2 rounded border-black border-2 flex flex-col items-center flex-grow"
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
import PeopleIcon from '@/components/Icons/PeopleIcon.vue';
import SettingsIcon from '@/components/Icons/SettingsIcon.vue';
import UndoIcon from '@/components/Icons/UndoIcon.vue';

import { indexToRankFile } from './Shogiboard.helper';

export default {
  components: {
    ShogiboardPiece,
    ComputerIcon,
    PeopleIcon,
    SettingsIcon,
    UndoIcon,
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
    computerLevel: {
      required: true,
      type: Number,
    },
    possibleComputerLevels: {
      required: true,
      type: Array,
    },
    pvpMode: {
      required: true,
      type: Boolean,
    },
    started: {
      required: false,
      type: Boolean,
      default: true,
    },
  },
  data() {
    return {
      settingsOpened: false,
    };
  },
  computed: {
    readibleMoveHistory() {
      return this.moveHistory.map(({ move }) => [
        indexToRankFile(move[0]),
        indexToRankFile(move[1]),
      ]);
    },
  },
  watch: {
    // auto scroll down when movehistory list gets too big
    readibleMoveHistory() {
      this.$nextTick(() => {
        const container = this.$refs.moveHistory;
        container.scrollTop = container.scrollHeight;
      });
    },
  },
  methods: {
    toggleSettingsOpened() {
      if (this.started) {
        this.settingsOpened = !this.settingsOpened;
      } else {
        this.settingsOpened = false;
      }
    },
    restart() {
      this.settingsOpened = false;
      this.$emit('restart');
    },
  },
};
</script>

<style>
button {
  outline: 0;
}
button:focus {
  outline: 0;
}
</style>
