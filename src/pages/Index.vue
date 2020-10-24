<template>
  <Layout>
    <h1 class="text-gray-500 text-2xl font-bold py-2 px-3">
      Player
      <button @click="pvpMode = !pvpMode">
        vs
        {{ pvpMode ? 'Player' : 'Computer' }}
      </button>
    </h1>
    <ShogiGame v-model:pvp-mode="pvpMode" />
  </Layout>
</template>

<script>
import Layout from '@/layouts/Default.vue';
import ShogiGame from '@/containers/ShogiGame.vue';

export default {
  components: {
    Layout,
    ShogiGame,
  },
  metaInfo: {
    title: 'Lets play some Shogi!',
  },
  data() {
    return {
      pvpMode: false,
    };
  },
};
</script>
