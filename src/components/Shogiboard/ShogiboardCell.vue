<template>
  <div class="flex justify-center items-center">
    <slot />
  </div>
</template>

<script>
export default {
  props: {
    piece: {
      type: Object,
      required: false,
      default: null,
    },
  },
};
</script>

<style scoped></style>
