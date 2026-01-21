<template>
  <div class="h-full flex flex-col items-center justify-center gap-12 p-8">
    <TimerDisplay
      :is-running="isRunning"
      :remaining-seconds="remainingSeconds"
      :total-seconds="selectedDuration * 60"
      @update:task="handleTaskUpdate"
      @select-duration="handleDurationSelect"
    />

    <TimerControls
      :is-running="isRunning"
      :remaining-seconds="remainingSeconds"
      @start="handleStart"
      @pause="handlePause"
      @resume="handleResume"
      @stop="handleStop"
    />

    <!-- Focus tip -->
    <div v-if="isRunning" class="pixel-border p-4 bg-pixel-bg max-w-md text-center">
      <p class="text-sm font-pixel text-pixel-green">💪 保持专注，你可以的！</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useTimerStore } from '@/stores/timer'
import { useSettingsStore } from '@/stores/settings'
import TimerDisplay from '@/components/TimerDisplay.vue'
import TimerControls from '@/components/TimerControls.vue'

const timerStore = useTimerStore()
const settingsStore = useSettingsStore()
const isRunning = ref(false)
const remainingSeconds = ref(0)
const selectedDuration = ref(25)
let timerInterval: number | null = null

function handleTaskUpdate(_task: string) {
  // Save task description
}

function handleDurationSelect(duration: number) {
  selectedDuration.value = duration
}

async function handleStart() {
  await timerStore.startSession(selectedDuration.value, '')
  isRunning.value = true
  remainingSeconds.value = selectedDuration.value * 60
  startTimer()
}

async function handlePause() {
  await timerStore.pauseSession()
  isRunning.value = false
  if (timerInterval) {
    clearInterval(timerInterval)
    timerInterval = null
  }
}

async function handleResume() {
  await timerStore.resumeSession()
  isRunning.value = true
  startTimer()
}

async function handleStop(completed: boolean) {
  await timerStore.stopSession(completed)
  isRunning.value = false
  remainingSeconds.value = 0
  if (timerInterval) {
    clearInterval(timerInterval)
    timerInterval = null
  }

  if (completed && settingsStore.soundEnabled) {
    await invoke('play_completion_sound')
    alert('🎉 恭喜！完成了一次专注！')
  }
}

function startTimer() {
  const endTime = Date.now() + remainingSeconds.value * 1000

  timerInterval = setInterval(() => {
    const now = Date.now()
    const diff = Math.max(0, endTime - now)
    remainingSeconds.value = Math.floor(diff / 1000)

    if (diff <= 0) {
      handleStop(true)
    }
  }, 100) as unknown as number
}

// Keyboard shortcuts
function handleKeyPress(event: KeyboardEvent) {
  // Space: Start/Pause/Resume
  if (event.code === 'Space' && !event.repeat) {
    event.preventDefault()
    if (!isRunning.value && remainingSeconds.value === 0) {
      handleStart()
    } else if (isRunning.value) {
      handlePause()
    } else if (remainingSeconds.value > 0) {
      handleResume()
    }
  }

  // Escape: Stop
  if (event.code === 'Escape' && remainingSeconds.value > 0) {
    event.preventDefault()
    handleStop(false)
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeyPress)
})

onUnmounted(() => {
  if (timerInterval) {
    clearInterval(timerInterval)
  }
  window.removeEventListener('keydown', handleKeyPress)
})
</script>
