<template>
  <div class="h-full overflow-y-auto p-4">
    <StatsCards :sessions="sessions" />
    <TrendChart :sessions="sessions" />
    <TagDistribution :sessions="sessions" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useTimerStore } from '@/stores/timer'
import StatsCards from '@/components/StatsCards.vue'
import TrendChart from '@/components/TrendChart.vue'
import TagDistribution from '@/components/TagDistribution.vue'
import type { FocusSession } from '@/types/database'

const timerStore = useTimerStore()
const sessions = ref<FocusSession[]>([])

async function loadSessions() {
  sessions.value = await timerStore.loadSessions()
}

onMounted(() => loadSessions())
</script>
