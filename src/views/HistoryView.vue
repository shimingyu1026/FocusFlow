<template>
  <div class="h-full p-4">
    <SessionList
      :sessions="sessions"
      @delete="handleDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useTimerStore } from '@/stores/timer'
import SessionList from '@/components/SessionList.vue'
import { emitSessionsUpdated, onSessionsUpdated } from '@/utils/sessionEvents'
import type { FocusSession } from '@/types/database'

const timerStore = useTimerStore()
const sessions = ref<FocusSession[]>([])

async function loadSessions() {
  sessions.value = await timerStore.loadSessions()
}

async function handleDelete(id: string) {
  await timerStore.deleteSession(id)
  emitSessionsUpdated()
  await loadSessions()
}

onMounted(() => {
  loadSessions()
  stopListening = onSessionsUpdated(loadSessions)
})

let stopListening: (() => void) | null = null

onUnmounted(() => {
  stopListening?.()
  stopListening = null
})
</script>
