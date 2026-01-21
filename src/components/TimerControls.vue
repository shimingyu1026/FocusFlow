<template>
  <div class="flex gap-6 items-center justify-center">
    <!-- Start button -->
    <button
      v-if="!isRunning && remainingSeconds === 0"
      @click="handleStart"
      class="pixel-button px-8 py-4 text-lg"
      style="--pixel-primary: #39ff14; --pixel-primary-dark: #2ecc0f;"
    >
      ▶ 开始
    </button>

    <!-- Pause button -->
    <button
      v-if="isRunning"
      @click="handlePause"
      class="pixel-button px-8 py-4 text-lg"
      style="--pixel-primary: #ffff00; --pixel-primary-dark: #cccc00;"
    >
      ⏸ 暂停
    </button>

    <!-- Resume button -->
    <button
      v-if="!isRunning && remainingSeconds > 0"
      @click="handleResume"
      class="pixel-button px-8 py-4 text-lg"
      style="--pixel-primary: #39ff14; --pixel-primary-dark: #2ecc0f;"
    >
      ▶ 继续
    </button>

    <!-- Stop button -->
    <button
      v-if="remainingSeconds > 0"
      @click="handleStop"
      class="pixel-button px-8 py-4 text-lg"
      style="--pixel-primary: #ff4444; --pixel-primary-dark: #cc0000;"
    >
      ⏹ 停止
    </button>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  isRunning: boolean
  remainingSeconds: number
}>()

const emit = defineEmits<{
  start: []
  pause: []
  resume: []
  stop: [completed: boolean]
}>()

function handleStart() {
  emit('start')
}

function handlePause() {
  emit('pause')
}

function handleResume() {
  emit('resume')
}

function handleStop() {
  const confirmed = confirm('确定要停止当前专注吗？')
  if (confirmed) {
    emit('stop', false)
  }
}
</script>
