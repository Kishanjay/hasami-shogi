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
    const ranks = [];
    const firstRankCharCode = 'A'.charCodeAt(0);
    for (
      let charCode = firstRankCharCode;
      charCode < firstRankCharCode + this.numberOfRows;
      charCode += 1
    ) {
      ranks.push(String.fromCharCode(charCode));
    }
    return {
      ranks,
    };
  },
};
</script>
