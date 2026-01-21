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

    <!-- Circular timer with progress ring -->
    <div class="timer-scale-wrapper">
      <div class="timer-circle-container">
        <!-- SVG Progress Ring -->
        <svg class="timer-progress-ring" viewBox="0 0 320 320">
          <!-- Background circle -->
          <circle
            cx="160"
            cy="160"
            r="144"
            fill="none"
            stroke="rgba(30, 27, 75, 0.3)"
            stroke-width="8"
          />
          <!-- Progress circle -->
          <circle
            class="progress-circle"
            cx="160"
            cy="160"
            r="144"
            fill="none"
            :stroke="isRunning ? 'var(--color-primary)' : 'var(--color-secondary)'"
            stroke-width="8"
            stroke-linecap="round"
            transform="rotate(-90 160 160)"
            :stroke-dasharray="progressCircumference"
            :stroke-dashoffset="progressOffset"
          />
        </svg>

        <!-- Inner circle container -->
        <div class="timer-inner-circle">
          <!-- Time display -->
          <p class="timer-text">{{ formattedTime }}</p>

          <!-- Status label -->
          <p
            v-if="isRunning"
            class="timer-status-active"
          >
            专注中...
          </p>
          <p v-else class="timer-status-idle">准备开始</p>
        </div>

        <!-- Pixel star decorations -->
        <div class="pixel-star pixel-star-top-left"></div>
        <div class="pixel-star pixel-star-top-right"></div>
        <div class="pixel-star pixel-star-bottom-left"></div>
        <div class="pixel-star pixel-star-bottom-right"></div>
      </div>
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
  totalSeconds: number
}>()

const emit = defineEmits<{
  'update:task': [task: string]
  'select-duration': [duration: number]
}>()

const taskInput = ref('')
const selectedDuration = ref(25)

// Progress ring calculations
const progressCircumference = 2 * Math.PI * 144 // ≈ 904.78

const progressOffset = computed(() => {
  if (props.totalSeconds === 0) return 0
  const progress = props.remainingSeconds / props.totalSeconds
  return progressCircumference * (1 - progress)
})

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

<style scoped>
/* Responsive scaling wrapper */
.timer-scale-wrapper {
  max-width: 100%;
  max-height: 100%;
  transform-origin: center center;
}

/* Main circle container - 320px × 320px */
.timer-circle-container {
  position: relative;
  width: 320px;
  height: 320px;
  margin: 0 auto;
}

/* SVG Progress Ring */
.timer-progress-ring {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  filter: drop-shadow(0 0 8px var(--color-primary));
}

/* Progress circle animation */
.progress-circle {
  transition: stroke-dashoffset 1s linear;
}

/* Inner circle with semi-transparent background */
.timer-inner-circle {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 280px;
  height: 280px;
  border-radius: 50%;
  background: rgba(30, 27, 75, 0.8);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  box-shadow: inset 0 0 20px rgba(0, 0, 0, 0.5);
}

/* Timer text - VT323 font, 5.5rem (88px) */
.timer-text {
  font-family: 'VT323', monospace;
  font-size: 5.5rem;
  font-weight: 400;
  line-height: 1;
  color: var(--color-primary);
  letter-spacing: 0.1em;
  text-shadow: 0 0 10px var(--color-primary);
  margin: 0;
}

/* Active status label with pulse animation */
.timer-status-active {
  font-family: 'VT323', monospace;
  font-size: 1.25rem;
  color: var(--color-secondary);
  margin-top: 1rem;
  animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
    text-shadow: 0 0 10px var(--color-secondary);
  }
  50% {
    opacity: 0.7;
    text-shadow: 0 0 20px var(--color-secondary);
  }
}

/* Idle status label */
.timer-status-idle {
  font-family: 'VT323', monospace;
  font-size: 1.25rem;
  color: rgba(255, 255, 255, 0.4);
  margin-top: 1rem;
}

/* Pixel star decorations */
.pixel-star {
  position: absolute;
  width: 20px;
  height: 20px;
  background: var(--color-primary);
  clip-path: polygon(
    50% 0%,
    61% 35%,
    98% 35%,
    68% 57%,
    79% 91%,
    50% 70%,
    21% 91%,
    32% 57%,
    2% 35%,
    39% 35%
  );
  box-shadow: 0 0 8px var(--color-primary);
}

.pixel-star-top-left {
  top: -8px;
  left: -8px;
}

.pixel-star-top-right {
  top: -8px;
  right: -8px;
}

.pixel-star-bottom-left {
  bottom: -8px;
  left: -8px;
}

.pixel-star-bottom-right {
  bottom: -8px;
  right: -8px;
}

/* Responsive breakpoints */
@media (max-height: 700px) {
  .timer-scale-wrapper {
    transform: scale(0.85);
  }
}

@media (max-height: 450px) {
  .timer-scale-wrapper {
    transform: scale(0.75);
  }
}

@media (max-width: 350px) {
  .timer-scale-wrapper {
    transform: scale(0.65);
  }
}
</style>
