<template>
  <div class="pixel-border p-4 bg-pixel-bg">
    <h3 class="text-sm font-pixel text-pixel-green mb-4">🏷️ 标签分布</h3>
    <div class="h-64">
      <canvas ref="chartCanvas"></canvas>
    </div>
    <div v-if="tagStats.length === 0" class="text-center text-pixel-text-muted font-pixel text-xs mt-8">
      暂无标签数据
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue'
import Chart from 'chart.js/auto'
import { calculateTagStats } from '@/utils/stats'
import { useSettingsStore } from '@/stores/settings'
import type { FocusSession } from '@/types/database'

const chartCanvas = ref<HTMLCanvasElement>()
let chartInstance: Chart | null = null
const settingsStore = useSettingsStore()

const props = defineProps<{
  sessions: FocusSession[]
}>()

const tagStats = computed(() => calculateTagStats(props.sessions))

function renderChart() {
  if (chartInstance) {
    chartInstance.destroy()
    chartInstance = null
  }

  if (!chartCanvas.value || tagStats.value.length === 0) return

  const ctx = chartCanvas.value.getContext('2d')
  if (!ctx) return

  const styles = getComputedStyle(document.documentElement)
  const primary = styles.getPropertyValue('--pixel-primary').trim() || '#14b8a6'
  const secondary = styles.getPropertyValue('--pixel-secondary').trim() || '#f97316'
  const warning = styles.getPropertyValue('--pixel-warning').trim() || '#facc15'
  const muted = styles.getPropertyValue('--pixel-text-muted').trim() || '#94a3b8'
  const panel = styles.getPropertyValue('--pixel-bg').trim() || '#1e1b4b'

  const colors = [
    primary, secondary, warning, '#38bdf8', '#a78bfa',
    '#fb7185', '#34d399', '#818cf8', '#f472b6', '#fbbf24'
  ]

  chartInstance = new Chart(ctx, {
    type: 'doughnut',
    data: {
      labels: tagStats.value.map(s => s.tag),
      datasets: [{
        data: tagStats.value.map(s => s.total_minutes),
        backgroundColor: colors,
        borderColor: panel,
        borderWidth: 2
      }]
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: {
        legend: {
          position: 'right',
          labels: {
            color: muted,
            font: { family: '"Press Start 2P"', size: 10 }
          }
        }
      }
    }
  })
}

onMounted(() => renderChart())
watch(tagStats, () => renderChart(), { deep: true })
watch(() => [settingsStore.themeMode, settingsStore.themeAccent], () => renderChart())
</script>
