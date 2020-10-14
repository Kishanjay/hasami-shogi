<template>
  <div class="flex">
    <!-- rank offset -->
    <div class="w-4" />

    <div
      v-for="file of numberOfColumns"
      :key="file"
      class="w-16 h-5 flex justify-center items-center text-xs"
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
