<template>
  <div class="h-full p-4">
    <SessionList
      :sessions="sessions"
      @delete="handleDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useTimerStore } from '@/stores/timer'
import SessionList from '@/components/SessionList.vue'
import type { FocusSession } from '@/types/database'

const timerStore = useTimerStore()
const sessions = ref<FocusSession[]>([])

async function loadSessions() {
  sessions.value = await timerStore.loadSessions()
}

async function handleDelete(id: string) {
  await timerStore.deleteSession(id)
  await loadSessions()
}

onMounted(() => {
  loadSessions()
})
</script>
