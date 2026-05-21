<template>
  <div class="controls-shell">
    <button
      v-if="!isRunning && remainingSeconds === 0"
      @click="emit('start')"
      class="control-button control-button--primary"
    >
      <span class="control-button-copy">
        <strong>开始专注</strong>
        <small>进入这一轮工作</small>
      </span>
    </button>

    <button
      v-if="isRunning"
      @click="emit('pause')"
      class="control-button control-button--warning"
    >
      <span class="control-button-copy">
        <strong>暂停一下</strong>
        <small>先缓一口气</small>
      </span>
    </button>

    <button
      v-if="!isRunning && remainingSeconds > 0"
      @click="emit('resume')"
      class="control-button control-button--primary"
    >
      <span class="control-button-copy">
        <strong>继续专注</strong>
        <small>回到当前节奏</small>
      </span>
    </button>

    <button
      v-if="remainingSeconds > 0"
      @click="emit('stop', false)"
      class="control-button control-button--danger"
    >
      <span class="control-button-copy">
        <strong>立即结束</strong>
        <small>停止这一轮计时</small>
      </span>
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
</script>

<style scoped>
.controls-shell {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 10px;
  width: 100%;
}

.control-button {
  display: inline-flex;
  min-width: 180px;
  align-items: center;
  justify-content: center;
  border: 1px solid var(--surface-line-strong);
  border-radius: 7px;
  background: rgba(0, 0, 0, 0.22);
  color: var(--pixel-text);
  cursor: pointer;
  padding: 12px 16px;
  transition:
    transform 0.16s ease,
    border-color 0.16s ease,
    box-shadow 0.16s ease,
    background-color 0.16s ease;
}

.control-button:hover {
  transform: translateY(-1px);
  background: rgba(0, 0, 0, 0.32);
}

.control-button-copy {
  display: flex;
  flex-direction: column;
  align-items: center;
  line-height: 1;
}

.control-button-copy strong {
  font-size: 1rem;
  font-weight: 760;
}

.control-button-copy small {
  margin-top: 4px;
  color: var(--pixel-text-muted);
  font-size: 0.78rem;
}

.control-button--primary {
  color: var(--pixel-primary);
}

.control-button--warning {
  border-color: rgba(215, 181, 109, 0.42);
  color: var(--pixel-warning);
}

.control-button--danger {
  border-color: rgba(213, 109, 95, 0.46);
  color: var(--pixel-danger);
}

@media (max-width: 560px) {
  .control-button {
    width: 100%;
    max-width: 360px;
  }
}
</style>
