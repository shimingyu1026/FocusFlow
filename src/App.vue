<template>
  <div class="w-full h-full bg-pixel-bg text-white flex flex-col">
    <AppHeader />
    <main class="flex-1 overflow-hidden">
      <router-view />
    </main>
    <AppNav />
  </div>
</template>

<script setup lang="ts">
import AppHeader from '@/components/AppHeader.vue'
import AppNav from '@/components/AppNav.vue'
</script>
