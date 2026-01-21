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
import { invoke } from '@tauri-apps/api/core'
import Chart from 'chart.js/auto'

const chartCanvas = ref<HTMLCanvasElement>()
let chartInstance: Chart | null = null

const props = defineProps<{
  sessions: any[]
}>()

async function renderChart() {
  if (!chartCanvas.value) return

  const stats = await invoke<any[]>('get_stats')

  const labels = stats.map((s: any) => {
    const date = new Date(s.date)
    return `${date.getMonth() + 1}/${date.getDate()}`
  })
  const data = stats.map((s: any) => Math.round(s.total_minutes / 60 * 10) / 10)

  if (chartInstance) {
    chartInstance.destroy()
  }

  const ctx = chartCanvas.value.getContext('2d')
  if (!ctx) return

  chartInstance = new Chart(ctx, {
    type: 'line',
    data: {
      labels,
      datasets: [{
        label: '专注时长(小时)',
        data,
        borderColor: '#39ff14',
        backgroundColor: 'rgba(57, 255, 20, 0.1)',
        borderWidth: 2,
        tension: 0,
        pointRadius: 3,
        pointBackgroundColor: '#39ff14',
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
          grid: { color: 'rgba(255, 255, 255, 0.1)' },
          ticks: {
            color: '#9ca3af',
            font: { family: '"Press Start 2P"' }
          }
        },
        x: {
          grid: { display: false },
          ticks: {
            color: '#9ca3af',
            font: { family: '"Press Start 2P"', size: 8 }
          }
        }
      }
    }
  })
}

onMounted(() => renderChart())
watch(() => props.sessions, () => renderChart(), { deep: true })
</script>
