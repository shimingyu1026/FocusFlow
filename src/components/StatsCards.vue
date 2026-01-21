<template>
  <div class="grid grid-cols-2 gap-4 mb-6">
    <div class="pixel-border p-4" style="background-color: var(--pixel-bg);">
      <p class="text-xs mb-2" style="color: var(--pixel-text-muted);">今日专注</p>
      <p class="text-2xl" style="color: var(--pixel-primary);">{{ formatMinutes(todayTotal) }}</p>
      <p class="text-xs mt-1" style="color: var(--pixel-text-muted);">{{ todayCount }}次</p>
    </div>

    <div class="pixel-border p-4" style="background-color: var(--pixel-bg);">
      <p class="text-xs mb-2" style="color: var(--pixel-text-muted);">本周专注</p>
      <p class="text-2xl" style="color: #00d9ff;">{{ formatMinutes(weekTotal) }}</p>
      <p class="text-xs mt-1" style="color: var(--pixel-text-muted);">日均 {{ formatMinutes(Math.round(weekTotal / 7)) }}</p>
    </div>

    <div class="pixel-border p-4" style="background-color: var(--pixel-bg);">
      <p class="text-xs mb-2" style="color: var(--pixel-text-muted);">本月专注</p>
      <p class="text-2xl" style="color: var(--pixel-secondary);">{{ formatMinutes(monthTotal) }}</p>
      <p class="text-xs mt-1" style="color: var(--pixel-text-muted);">{{ monthCount }}次</p>
    </div>

    <div class="pixel-border p-4" style="background-color: var(--pixel-bg);">
      <p class="text-xs mb-2" style="color: var(--pixel-text-muted);">历史总计</p>
      <p class="text-2xl" style="color: #ffff00;">{{ formatMinutes(totalAll) }}</p>
      <p class="text-xs mt-1" style="color: var(--pixel-text-muted);">加油！</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { formatMinutes } from '@/utils/stats'

const props = defineProps<{
  sessions: any[]
}>()

const now = new Date()
const todayStart = new Date(now.getFullYear(), now.getMonth(), now.getDate())

const todaySessions = computed(() =>
  props.sessions.filter(s => new Date(s.startTime) >= todayStart)
)

const weekAgo = new Date(now.getTime() - 7 * 24 * 60 * 60 * 1000)
const weekSessions = computed(() =>
  props.sessions.filter(s => new Date(s.startTime) >= weekAgo)
)

const monthAgo = new Date(now.getTime() - 30 * 24 * 60 * 60 * 1000)
const monthSessions = computed(() =>
  props.sessions.filter(s => new Date(s.startTime) >= monthAgo)
)

const todayTotal = computed(() =>
  todaySessions.value.reduce((sum, s) => sum + s.duration, 0)
)
const todayCount = computed(() => todaySessions.value.length)

const weekTotal = computed(() =>
  weekSessions.value.reduce((sum, s) => sum + s.duration, 0)
)

const monthTotal = computed(() =>
  monthSessions.value.reduce((sum, s) => sum + s.duration, 0)
)
const monthCount = computed(() => monthSessions.value.length)

const totalAll = computed(() =>
  props.sessions.reduce((sum, s) => sum + s.duration, 0)
)
</script>
