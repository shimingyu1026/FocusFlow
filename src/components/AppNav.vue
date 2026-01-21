<template>
  <nav class="h-20 border-t-4 flex items-center justify-around" style="background-color: var(--pixel-bg); border-color: var(--pixel-primary);">
    <router-link
      to="/"
      class="pixel-button flex flex-col items-center gap-2 p-4"
      :class="{ 'nav-active': $route.name === 'timer' }"
    >
      <span class="text-2xl">⏱️</span>
      <span class="text-xs">计时器</span>
    </router-link>

    <router-link
      to="/history"
      class="pixel-button flex flex-col items-center gap-2 p-4"
      :class="{ 'nav-active': $route.name === 'history' }"
    >
      <span class="text-2xl">📝</span>
      <span class="text-xs">历史</span>
    </router-link>

    <router-link
      to="/statistics"
      class="pixel-button flex flex-col items-center gap-2 p-4"
      :class="{ 'nav-active': $route.name === 'statistics' }"
    >
      <span class="text-2xl">📊</span>
      <span class="text-xs">统计</span>
    </router-link>

    <router-link
      to="/settings"
      class="pixel-button flex flex-col items-center gap-2 p-4"
      :class="{ 'nav-active': $route.name === 'settings' }"
    >
      <span class="text-2xl">⚙️</span>
      <span class="text-xs">设置</span>
    </router-link>
  </nav>
</template>

<script setup lang="ts">
// Navigation component
</script>

<style scoped>
.nav-active {
  color: var(--pixel-primary);
}
</style>
