<template>
  <div class="layout flex min-h-screen">
    <header class="header px-4 py-6 flex-initial flex flex-col">
      <strong class="mb-6 text-lg">
        <a href="/">Hasami Shogi</a>
      </strong>

      <nav class="nav flex flex-col">
        <span class="text-xs font-bold text-gray-600">Play</span>
        <a
          href="/"
          class="p-2 rounded my-1 flex items-center"
          exact-active-class="bg-gray-200"
        >
          <PlayIcon class="w-4 h-4" />
          <span class="ml-2 text-sm font-bold text-gray-900">Offline</span>
        </a>

        <span class="text-xs font-bold text-gray-600 mt-4">Info</span>
        <a
          class="p-2 rounded my-1 flex items-center"
          href="/about"
          exact-active-class="bg-gray-200"
        >
          <InfoIcon class="w-4 h-4" />
          <span class="ml-2 text-sm font-bold text-gray-900">About</span>
        </a>
      </nav>
    </header>
    <main class="p-10 bg-gray-400 flex-auto rounded">
      <slot />
    </main>
  </div>
</template>

<script>
import PlayIcon from '@/components/Icons/PlayIcon.vue';
import InfoIcon from '@/components/Icons/InfoIcon.vue';

export default {
  components: {
    PlayIcon,
    InfoIcon,
  },
};
</script>

<style lang="css" scoped>
nav {
  min-width: 160px;
}
</style>
