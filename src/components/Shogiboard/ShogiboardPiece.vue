<template>
  <div
    class="w-8 h-8 rounded-full shadow-inner-lg"
    :class="{ 'bg-blue-300': teamId === 1, 'bg-red-300': teamId === 2 }"
  />
</template>

<script>
export default {
  props: {
    teamId: {
      type: Number,
      required: true,
    },
  },
};
</script>
