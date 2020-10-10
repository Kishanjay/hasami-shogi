<template>
  <Layout>
    <h1 class="text-gray-500 text-2xl font-bold py-2 px-3">Play offline</h1>
    <div class="flex items-start">
      <ShogiGame />
    </div>
  </Layout>
</template>

<script>
import ShogiGame from '@/containers/ShogiGame.vue';

export default {
  components: {
    ShogiGame,
  },
  metaInfo: {
    title: 'Lets play some Shogi!',
  },
};
</script>
