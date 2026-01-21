<template>
  <div class="flex flex-col items-center gap-8">
    <!-- Task input -->
    <div class="pixel-border p-4 w-full max-w-md bg-pixel-bg">
      <input
        v-model="taskInput"
        type="text"
        placeholder="输入当前专注任务..."
        class="w-full bg-transparent text-white font-pixel text-sm outline-none placeholder-gray-500"
        :disabled="isRunning"
      />
    </div>

    <!-- Circular timer -->
    <div class="relative w-72 h-72">
      <!-- Outer pixel border -->
      <div class="absolute inset-0 rounded-full pixel-border border-8 border-pixel-green"></div>

      <!-- Time display -->
      <div class="absolute inset-0 flex flex-col items-center justify-center">
        <p class="text-6xl font-pixel text-pixel-green">{{ formattedTime }}</p>
        <p v-if="isRunning" class="text-sm font-pixel text-pixel-pink mt-4">专注中...</p>
        <p v-else class="text-sm font-pixel text-gray-400 mt-4">准备开始</p>
      </div>

      <!-- Pixel decoration stars -->
      <div class="absolute -top-4 -left-4 text-2xl">⭐</div>
      <div class="absolute -top-4 -right-4 text-2xl">⭐</div>
      <div class="absolute -bottom-4 -left-4 text-2xl">⭐</div>
      <div class="absolute -bottom-4 -right-4 text-2xl">⭐</div>
    </div>

    <!-- Duration selection -->
    <div v-if="!isRunning && remainingSeconds === 0" class="flex gap-4">
      <button
        v-for="duration in [15, 25, 45, 60]"
        :key="duration"
        @click="selectDuration(duration)"
        class="pixel-button px-4 py-2 pixel-border border-pixel-blue text-sm font-pixel"
        :class="{ 'bg-pixel-blue text-black': selectedDuration === duration }"
      >
        {{ duration }}分钟
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'

const props = defineProps<{
  isRunning: boolean
  remainingSeconds: number
}>()

const emit = defineEmits<{
  'update:task': [task: string]
  'select-duration': [duration: number]
}>()

const taskInput = ref('')
const selectedDuration = ref(25)

const formattedTime = computed(() => {
  const minutes = Math.floor(props.remainingSeconds / 60)
  const seconds = props.remainingSeconds % 60
  return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`
})

watch(taskInput, (newTask) => {
  emit('update:task', newTask)
})

function selectDuration(duration: number) {
  selectedDuration.value = duration
  emit('select-duration', duration)
}
</script>
