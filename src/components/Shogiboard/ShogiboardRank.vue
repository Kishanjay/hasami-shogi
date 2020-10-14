<template>
  <div class="w-4">
    <div
      v-for="rank of ranks.slice()"
      :key="rank"
      class="h-16 flex items-center justify-center text-xs"
    >
      {{ rank }}
    </div>
  </div>
</template>

<script>
import { getRanks } from '@/components/Shogiboard/Shogiboard.helper';

export default {
  props: {
    position: {
      type: String,
      required: false,
      default: 'left',
      validate: (val) => ['left', 'right'].includes(val),
    },
    numberOfRows: {
      type: Number,
      required: true,
    },
  },
  data() {
    const ranks = getRanks(this.numberOfRows);
    return {
      ranks,
    };
  },
};
</script>
