<template>
  <div class="layout flex min-h-screen">
    <header class="header px-4 py-6 flex-initial flex flex-col">
      <strong class="mb-4">
        <g-link to="/">{{ $static.metadata.siteName }}</g-link>
      </strong>

      <nav class="nav flex flex-col">
        <g-link
          class="p-2 rounded my-1 flex items-center"
          to="/"
          exact-active-class="bg-gray-200"
        >
          <PlayIcon class="w-4 h-4" />
          <span class="ml-2">Play</span>
        </g-link>
        <g-link
          class="p-2 rounded my-1 flex items-center"
          to="/about"
          exact-active-class="bg-gray-200"
        >
          <InfoIcon class="w-4 h-4" />
          <span class="ml-2">About</span>
        </g-link>
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

<static-query>
query {
  metadata {
    siteName
  }
}
</static-query>

<style lang="css" scoped>
nav {
  min-width: 160px;
}
</style>
