<template>
  <div class="flex gap-6 items-center justify-center">
    <!-- Start button -->
    <button
      v-if="!isRunning && remainingSeconds === 0"
      @click="handleStart"
      class="pixel-button pixel-border border-pixel-green bg-pixel-green text-black px-8 py-4 font-pixel text-lg hover:shadow-[0_0_20px_#39ff14]"
    >
      ▶ 开始
    </button>

    <!-- Pause button -->
    <button
      v-if="isRunning"
      @click="handlePause"
      class="pixel-button pixel-border border-pixel-yellow bg-pixel-yellow text-black px-8 py-4 font-pixel text-lg hover:shadow-[0_0_20px_#ffff00]"
    >
      ⏸ 暂停
    </button>

    <!-- Resume button -->
    <button
      v-if="!isRunning && remainingSeconds > 0"
      @click="handleResume"
      class="pixel-button pixel-border border-pixel-green bg-pixel-green text-black px-8 py-4 font-pixel text-lg hover:shadow-[0_0_20px_#39ff14]"
    >
      ▶ 继续
    </button>

    <!-- Stop button -->
    <button
      v-if="remainingSeconds > 0"
      @click="handleStop"
      class="pixel-button pixel-border border-pixel-pink bg-pixel-pink text-black px-8 py-4 font-pixel text-lg hover:shadow-[0_0_20px_#ff6ec7]"
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
