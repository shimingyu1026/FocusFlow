<template>
  <div class="flex h-full items-center justify-center overflow-hidden px-5 py-4 sm:px-6 sm:py-5">
    <div class="mx-auto flex w-full max-w-4xl flex-col items-center justify-center gap-6 sm:gap-8">
      <TimerDisplay
        :is-running="isRunning"
        :remaining-seconds="remainingSeconds"
        :total-seconds="selectedDuration * 60"
        :selected-duration="selectedDuration"
        @update:task="handleTaskUpdate"
        @update:tags="handleTagsUpdate"
        @select-duration="handleDurationSelect"
      >
        <template #actions>
          <TimerControls
            :is-running="isRunning"
            :remaining-seconds="remainingSeconds"
            @start="handleStart"
            @pause="handlePause"
            @resume="handleResume"
            @stop="handleStop"
          />
        </template>
      </TimerDisplay>

      <!-- Focus tip -->
      <div v-if="isRunning" class="pixel-border p-4 bg-pixel-bg max-w-md text-center">
        <p class="text-sm font-pixel text-pixel-green">💪 保持专注，你可以的！</p>
      </div>
    </div>

    <CompletionAnimation
      v-if="showCompletion"
      :duration="selectedDuration"
      :today-count="todayCompletedCount"
      :variant="settingsStore.celebrationStyle"
      @close="showCompletion = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useTimerStore } from '@/stores/timer'
import { useSettingsStore } from '@/stores/settings'
import TimerDisplay from '@/components/TimerDisplay.vue'
import TimerControls from '@/components/TimerControls.vue'
import CompletionAnimation from '@/components/CompletionAnimation.vue'
import { emitSessionsUpdated } from '@/utils/sessionEvents'

const timerStore = useTimerStore()
const settingsStore = useSettingsStore()
const isRunning = ref(false)
const remainingSeconds = ref(0)
const selectedDuration = ref(settingsStore.defaultDuration)
const showCompletion = ref(false)
const todayCompletedCount = ref(0)
const currentTask = ref('')
const currentTags = ref<string[]>([])
let timerInterval: number | null = null

function handleTaskUpdate(task: string) {
  currentTask.value = task.trim()
}

function handleTagsUpdate(tags: string[]) {
  currentTags.value = tags
}

function handleDurationSelect(duration: number) {
  selectedDuration.value = duration
}

async function handleStart() {
  await timerStore.startSession(selectedDuration.value, currentTask.value, currentTags.value)
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
  const elapsedSeconds = Math.max(0, selectedDuration.value * 60 - remainingSeconds.value)
  await timerStore.stopSession(completed, elapsedSeconds)
  emitSessionsUpdated()
  isRunning.value = false
  remainingSeconds.value = 0
  if (timerInterval) {
    clearInterval(timerInterval)
    timerInterval = null
  }

  if (completed && settingsStore.soundEnabled) {
    await invoke('play_completion_sound')
    // Show completion animation instead of alert
    todayCompletedCount.value++
    showCompletion.value = true
  }
}

watch(() => settingsStore.defaultDuration, (newDuration) => {
  if (!isRunning.value && remainingSeconds.value === 0) {
    selectedDuration.value = newDuration
  }
})

function startTimer() {
  const endTime = Date.now() + remainingSeconds.value * 1000

  timerInterval = setInterval(() => {
    const now = Date.now()
    const diff = Math.max(0, endTime - now)
    remainingSeconds.value = Math.floor(diff / 1000)

    if (diff <= 0) {
      if (timerInterval) {
        clearInterval(timerInterval)
        timerInterval = null
      }
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
