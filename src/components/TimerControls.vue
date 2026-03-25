<template>
  <div class="controls-shell">
    <button
      v-if="!isRunning && remainingSeconds === 0"
      @click="emit('start')"
      class="control-button control-button--primary"
    >
      <span class="control-button-icon">▶</span>
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
      <span class="control-button-icon">⏸</span>
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
      <span class="control-button-icon">▶</span>
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
      <span class="control-button-icon">■</span>
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
  gap: 14px;
  border: 1px solid rgba(148, 163, 184, 0.18);
  border-radius: 22px;
  background: rgba(15, 23, 42, 0.16);
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
  transform: translateY(-2px);
}

.control-button-icon {
  display: inline-flex;
  width: 36px;
  height: 36px;
  align-items: center;
  justify-content: center;
  border-radius: 14px;
  background: rgba(255, 255, 255, 0.08);
  font-size: 1.3rem;
  flex-shrink: 0;
}

.control-button-copy {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  line-height: 1;
}

.control-button-copy strong {
  font-size: 1.25rem;
  font-weight: 400;
}

.control-button-copy small {
  margin-top: 4px;
  color: var(--pixel-text-muted);
  font-size: 0.88rem;
}

.control-button--primary {
  border-color: rgba(20, 184, 166, 0.35);
  box-shadow: 0 12px 30px rgba(20, 184, 166, 0.12);
}

.control-button--primary .control-button-icon,
.control-button--primary .control-button-copy strong {
  color: var(--pixel-primary);
}

.control-button--warning {
  border-color: rgba(250, 204, 21, 0.35);
  box-shadow: 0 12px 30px rgba(250, 204, 21, 0.1);
}

.control-button--warning .control-button-icon,
.control-button--warning .control-button-copy strong {
  color: var(--pixel-warning);
}

.control-button--danger {
  border-color: rgba(255, 68, 68, 0.4);
  box-shadow: 0 12px 30px rgba(255, 68, 68, 0.12);
}

.control-button--danger .control-button-icon,
.control-button--danger .control-button-copy strong {
  color: var(--pixel-danger);
}

@media (max-width: 560px) {
  .control-button {
    width: 100%;
    max-width: 360px;
  }
}
</style>
