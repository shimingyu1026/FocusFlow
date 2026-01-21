<template>
  <div class="pixel-border p-4 bg-pixel-bg">
    <h3 class="text-sm font-pixel text-pixel-green mb-4">🏷️ 标签分布</h3>
    <div class="h-64">
      <canvas ref="chartCanvas"></canvas>
    </div>
    <div v-if="tagStats.length === 0" class="text-center text-gray-500 font-pixel text-xs mt-8">
      暂无标签数据
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue'
import Chart from 'chart.js/auto'
import { calculateTagStats } from '@/utils/stats'

const chartCanvas = ref<HTMLCanvasElement>()
let chartInstance: Chart | null = null

const props = defineProps<{
  sessions: any[]
}>()

const tagStats = computed(() => calculateTagStats(props.sessions))

function renderChart() {
  if (!chartCanvas.value || tagStats.value.length === 0) return

  if (chartInstance) {
    chartInstance.destroy()
  }

  const ctx = chartCanvas.value.getContext('2d')
  if (!ctx) return

  const colors = [
    '#39ff14', '#ff6ec7', '#ffff00', '#00d9ff', '#b14eff',
    '#ff6b35', '#f7931a', '#7b68ee', '#00ced1', '#ffd700'
  ]

  chartInstance = new Chart(ctx, {
    type: 'doughnut',
    data: {
      labels: tagStats.value.map(s => s.tag),
      datasets: [{
        data: tagStats.value.map(s => s.total_minutes),
        backgroundColor: colors,
        borderColor: '#2d1b4e',
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
            color: '#ffffff',
            font: { family: '"Press Start 2P"', size: 10 }
          }
        }
      }
    }
  })
}

onMounted(() => renderChart())
watch(tagStats, () => renderChart(), { deep: true })
</script>
