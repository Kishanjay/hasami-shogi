<template>
  <div class="flex">
    <div class="w-8" />
    <div
      v-for="file of numberOfColumns"
      :key="file"
      class="w-16 h-8 flex justify-center"
      :class="[position === 'top' ? 'items-start' : 'items-end']"
    >
      {{ file }}
    </div>
  </div>
</template>
<script>
export default {
  props: {
    position: {
      type: String,
      required: false,
      default: 'top',
      validate: (val) => ['top', 'bottom'].includes(val),
    },
    numberOfColumns: {
      type: Number,
      required: true,
    },
  },
};
</script>
