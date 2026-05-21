<template>
  <div class="pixel-border p-4 bg-pixel-bg mb-6">
    <h3 class="text-sm font-pixel text-pixel-green mb-4">📈 30天趋势</h3>
    <div class="h-48">
      <canvas ref="chartCanvas"></canvas>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import Chart from 'chart.js/auto'
import { useSettingsStore } from '@/stores/settings'
import { calculateDailyStats } from '@/utils/stats'
import type { FocusSession } from '@/types/database'

const chartCanvas = ref<HTMLCanvasElement>()
let chartInstance: Chart | null = null
const settingsStore = useSettingsStore()

const props = defineProps<{
  sessions: FocusSession[]
}>()

async function renderChart() {
  if (!chartCanvas.value) return

  const stats = calculateDailyStats(props.sessions, 30)

  const labels = stats.map(s => {
    const date = new Date(s.date)
    return `${date.getMonth() + 1}/${date.getDate()}`
  })
  const data = stats.map(s => Math.round(s.total_minutes / 60 * 10) / 10)

  if (chartInstance) {
    chartInstance.destroy()
  }

  const ctx = chartCanvas.value.getContext('2d')
  if (!ctx) return

  const styles = getComputedStyle(document.documentElement)
  const primary = styles.getPropertyValue('--pixel-primary').trim() || '#14b8a6'
  const textMuted = styles.getPropertyValue('--pixel-text-muted').trim() || '#94a3b8'

  chartInstance = new Chart(ctx, {
    type: 'line',
    data: {
      labels,
      datasets: [{
        label: '专注时长(小时)',
        data,
        borderColor: primary,
        backgroundColor: `${primary}1f`,
        borderWidth: 2,
        tension: 0,
        pointRadius: 3,
        pointBackgroundColor: primary,
      }]
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: {
        legend: { display: false }
      },
      scales: {
        y: {
          beginAtZero: true,
          grid: { color: `${textMuted}33` },
          ticks: {
            color: textMuted,
            font: { family: 'ui-sans-serif, system-ui, sans-serif' }
          }
        },
        x: {
          grid: { display: false },
          ticks: {
            color: textMuted,
            font: { family: 'ui-sans-serif, system-ui, sans-serif', size: 11 }
          }
        }
      }
    }
  })
}

onMounted(() => renderChart())
watch(() => props.sessions, () => renderChart(), { deep: true })
watch(() => [settingsStore.themeMode, settingsStore.themeAccent], () => renderChart())
</script>
