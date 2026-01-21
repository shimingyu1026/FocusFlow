<template>
  <nav class="h-20 pixel-border border-t-4 border-pixel-green flex items-center justify-around bg-pixel-bg">
    <router-link
      to="/"
      class="pixel-button flex flex-col items-center gap-2 p-4"
      :class="{ 'text-pixel-green': $route.name === 'timer' }"
    >
      <span class="text-2xl">⏱️</span>
      <span class="text-xs font-pixel">计时器</span>
    </router-link>

    <router-link
      to="/history"
      class="pixel-button flex flex-col items-center gap-2 p-4"
      :class="{ 'text-pixel-green': $route.name === 'history' }"
    >
      <span class="text-2xl">📝</span>
      <span class="text-xs font-pixel">历史</span>
    </router-link>

    <router-link
      to="/statistics"
      class="pixel-button flex flex-col items-center gap-2 p-4"
      :class="{ 'text-pixel-green': $route.name === 'statistics' }"
    >
      <span class="text-2xl">📊</span>
      <span class="text-xs font-pixel">统计</span>
    </router-link>

    <router-link
      to="/settings"
      class="pixel-button flex flex-col items-center gap-2 p-4"
      :class="{ 'text-pixel-green': $route.name === 'settings' }"
    >
      <span class="text-2xl">⚙️</span>
      <span class="text-xs font-pixel">设置</span>
    </router-link>
  </nav>
</template>

<script setup lang="ts">
// Navigation component
</script>
