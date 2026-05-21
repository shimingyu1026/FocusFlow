<template>
  <div class="timer-layout">
    <section class="timer-panel timer-panel--inputs">
      <div class="timer-panel-header">
        <span class="timer-kicker">Focus Session</span>
        <h2 class="timer-title">安排这一轮专注</h2>
      </div>

      <div class="timer-field-group">
        <label class="timer-label" for="task-input">当前任务</label>
        <input
          id="task-input"
          v-model="taskInput"
          type="text"
          placeholder="例如：整理周报、完成登录页、背 20 个单词"
          class="timer-input"
          :disabled="isRunning"
        />
      </div>

      <div class="timer-field-group">
        <label class="timer-label" for="tags-input">标签</label>
        <input
          id="tags-input"
          v-model="tagsInput"
          type="text"
          placeholder="例如：工作, 深度思考, 写作"
          class="timer-input"
          :disabled="isRunning"
        />
      </div>

      <div class="timer-presets">
        <div class="timer-presets-head">
          <span class="timer-label">专注时长</span>
          <span class="timer-preset-note">推荐先从 25 分钟开始</span>
        </div>

        <div class="timer-duration-grid">
          <button
            v-for="duration in [15, 25, 45, 60]"
            :key="duration"
            @click="selectDuration(duration)"
            class="timer-duration-button"
            :class="{ 'timer-duration-button--active': props.selectedDuration === duration }"
          >
            <span class="timer-duration-value">{{ duration }}</span>
            <span class="timer-duration-unit">分钟</span>
          </button>
        </div>
      </div>

      <div class="timer-inline-meta">
        <span>当前状态 {{ statusLabel }}</span>
        <span>目标时长 {{ props.selectedDuration }} 分钟</span>
      </div>
    </section>

    <section class="timer-panel timer-panel--hero">
      <div class="timer-ambient"></div>

      <div class="timer-circle-container">
        <svg class="timer-progress-ring" viewBox="0 0 320 320">
          <circle
            cx="160"
            cy="160"
            r="144"
            fill="none"
            class="progress-ring-track"
            stroke-width="10"
          />
          <circle
            class="progress-circle"
            cx="160"
            cy="160"
            r="144"
            fill="none"
            :stroke="progressColor"
            stroke-width="10"
            stroke-linecap="round"
            transform="rotate(-90 160 160)"
            :stroke-dasharray="progressCircumference"
            :stroke-dashoffset="progressOffset"
          />
        </svg>

        <div class="timer-inner-circle">
          <span class="timer-status-pill" :class="`timer-status-pill--${statusTone}`">
            {{ statusLabel }}
          </span>
          <p class="timer-text">{{ formattedTime }}</p>
          <p class="timer-support">{{ statusDescription }}</p>
        </div>
      </div>

      <div class="timer-actions">
        <slot name="actions" />
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'

const props = withDefaults(defineProps<{
  isRunning: boolean
  remainingSeconds: number
  totalSeconds?: number
  selectedDuration?: number
}>(), {
  totalSeconds: 0,
  selectedDuration: 25,
})

const emit = defineEmits<{
  'update:task': [task: string]
  'update:tags': [tags: string[]]
  'select-duration': [duration: number]
}>()

const taskInput = ref('')
const tagsInput = ref('')

const parsedTags = computed(() =>
  Array.from(
    new Set(
      tagsInput.value
        .split(/[，,]/)
        .map(tag => tag.trim())
        .filter(Boolean),
    ),
  ),
)

const displayRemainingSeconds = computed(() => {
  if (!props.isRunning && props.remainingSeconds === 0 && props.totalSeconds > 0) {
    return props.totalSeconds
  }

  return props.remainingSeconds
})

const isPaused = computed(() => !props.isRunning && props.remainingSeconds > 0)

const statusLabel = computed(() => {
  if (props.isRunning) return '专注中'
  if (isPaused.value) return '已暂停'
  return '待开始'
})

const statusTone = computed(() => {
  if (props.isRunning) return 'active'
  if (isPaused.value) return 'paused'
  return 'idle'
})

const statusDescription = computed(() => {
  if (props.isRunning) return '保持节奏，别切走注意力。'
  if (isPaused.value) return '可以恢复继续，或直接结束这一轮。'
  return '准备好后，一键开始这一轮深度工作。'
})

const progressColor = computed(() => {
  if (props.isRunning) return 'var(--pixel-primary)'
  if (isPaused.value) return 'var(--pixel-warning)'
  return 'var(--pixel-secondary)'
})

const progressCircumference = 2 * Math.PI * 144

const progressOffset = computed(() => {
  if (!props.totalSeconds || props.totalSeconds === 0) {
    return 0
  }
  const progress = displayRemainingSeconds.value / props.totalSeconds
  return progressCircumference * (1 - progress)
})

const formattedTime = computed(() => {
  const minutes = Math.floor(displayRemainingSeconds.value / 60)
  const seconds = displayRemainingSeconds.value % 60
  return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`
})

watch(taskInput, newTask => {
  emit('update:task', newTask)
})

watch(parsedTags, newTags => {
  emit('update:tags', newTags)
})

function selectDuration(duration: number) {
  emit('select-duration', duration)
}
</script>

<style scoped>
.timer-layout {
  display: grid;
  grid-template-columns: minmax(280px, 360px) minmax(320px, 1fr);
  gap: 16px;
  width: 100%;
  align-items: stretch;
}

.timer-panel {
  position: relative;
  overflow: hidden;
  border: 1px solid var(--surface-line);
  border-radius: 8px;
  background: var(--pixel-panel-solid);
  box-shadow: none;
  padding: 24px;
}

.timer-panel--inputs {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  gap: 16px;
}

.timer-panel--hero {
  display: flex;
  min-height: 0;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 14px;
}

.timer-panel-header {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.timer-kicker {
  display: inline-flex;
  width: fit-content;
  border-radius: 6px;
  background: rgba(0, 0, 0, 0.18);
  padding: 7px 10px;
  color: var(--pixel-text-muted);
  font-size: 0.74rem;
  font-weight: 760;
  letter-spacing: 0.06em;
  text-transform: uppercase;
}

.timer-title {
  margin: 0;
  color: var(--pixel-text);
  font-family: var(--app-font-serif);
  font-size: 1.7rem;
  font-weight: 700;
  line-height: 1.08;
}

.timer-field-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.timer-label {
  color: var(--pixel-text-muted);
  font-size: 0.86rem;
  font-weight: 650;
}

.timer-input {
  width: 100%;
  border: 1px solid var(--surface-line);
  border-radius: 7px;
  background: rgba(0, 0, 0, 0.16);
  padding: 13px 14px;
  color: var(--pixel-text);
  font-family: var(--app-font-serif);
  font-size: 1rem;
  outline: none;
}

.timer-input:focus {
  border-color: var(--pixel-primary-dark);
  box-shadow: 0 0 0 3px var(--pixel-glow);
}

.timer-presets {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.timer-presets-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.timer-preset-note {
  color: var(--pixel-text-muted);
  font-size: 0.82rem;
}

.timer-duration-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12px;
}

.timer-duration-button {
  display: flex;
  min-height: 74px;
  flex-direction: column;
  align-items: flex-start;
  justify-content: center;
  gap: 4px;
  border: 1px solid var(--surface-line);
  border-radius: 7px;
  background: rgba(0, 0, 0, 0.12);
  color: var(--pixel-text);
  cursor: pointer;
  padding: 14px 16px;
  transition: transform 0.16s ease, border-color 0.16s ease, background-color 0.16s ease;
}

.timer-duration-button:hover {
  transform: translateY(-2px);
  border-color: var(--surface-line-strong);
}

.timer-duration-button--active {
  border-color: var(--pixel-primary-dark);
  background: rgba(0, 0, 0, 0.24);
  box-shadow: inset 0 0 0 1px var(--surface-line);
}

.timer-duration-value {
  color: var(--pixel-text);
  font-family: var(--app-font-serif);
  font-size: 1.9rem;
  font-weight: 700;
  line-height: 1;
}

.timer-duration-unit {
  color: var(--pixel-text-muted);
  font-size: 0.88rem;
}

.timer-inline-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  color: var(--pixel-text-muted);
  font-size: 0.95rem;
}

.timer-ambient {
  display: none;
}

.timer-circle-container {
  position: relative;
  width: 286px;
  height: 286px;
  flex-shrink: 0;
}

.timer-progress-ring {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  filter: none;
}

.progress-ring-track {
  stroke: var(--surface-line);
}

.progress-circle {
  transition: stroke-dashoffset 1s linear, stroke 0.2s ease;
}

.timer-inner-circle {
  position: absolute;
  top: 50%;
  left: 50%;
  display: flex;
  width: 230px;
  height: 230px;
  transform: translate(-50%, -50%);
  border: 1px solid var(--surface-line);
  border-radius: 50%;
  background: rgba(0, 0, 0, 0.18);
  flex-direction: column;
  align-items: center;
  justify-content: center;
  box-shadow: none;
}

.timer-status-pill {
  margin-bottom: 14px;
  border-radius: 6px;
  padding: 6px 12px;
  font-size: 0.78rem;
  font-weight: 700;
}

.timer-status-pill--active {
  background: rgba(0, 0, 0, 0.22);
  color: var(--pixel-primary);
}

.timer-status-pill--paused {
  background: rgba(215, 181, 109, 0.14);
  color: var(--pixel-warning);
}

.timer-status-pill--idle {
  background: rgba(199, 143, 90, 0.16);
  color: var(--pixel-secondary);
}

.timer-text {
  margin: 0;
  color: var(--pixel-text);
  font-family: var(--app-font-serif);
  font-size: 4.1rem;
  font-weight: 700;
  line-height: 1;
  letter-spacing: 0;
  text-shadow: none;
}

.timer-support {
  margin: 12px 24px 0;
  color: var(--pixel-text-muted);
  font-size: 0.94rem;
  line-height: 1.35;
  text-align: center;
}

.timer-actions {
  width: 100%;
}

@media (max-width: 460px) {
  .timer-layout {
    grid-template-columns: 1fr;
  }

  .timer-panel--hero {
    order: -1;
  }
}

@media (max-height: 760px) {
  .timer-panel {
    padding: 22px;
  }

  .timer-layout {
    gap: 18px;
  }

  .timer-circle-container {
    width: 280px;
    height: 280px;
  }

  .timer-inner-circle {
    width: 226px;
    height: 226px;
  }

  .timer-text {
    font-size: 4rem;
  }
}

@media (max-width: 420px) {
  .timer-panel {
    padding: 18px;
    border-radius: 8px;
  }

  .timer-title {
    font-size: 1.7rem;
  }

  .timer-duration-grid {
    grid-template-columns: 1fr;
  }

  .timer-circle-container {
    width: 260px;
    height: 260px;
  }

  .timer-inner-circle {
    width: 208px;
    height: 208px;
  }

  .timer-text {
    font-size: 4rem;
  }
}
</style>
