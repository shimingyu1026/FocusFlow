<template>
  <div class="h-full overflow-y-auto p-6">
    <h2 class="text-xl font-pixel text-pixel-green mb-8 text-center">⚙️ 设置</h2>

    <!-- Sound settings -->
    <div class="pixel-border p-6 mb-6 bg-pixel-bg">
      <h3 class="text-sm font-pixel text-pixel-green mb-4">🔊 声音</h3>

      <div class="flex items-center justify-between mb-4">
        <span class="font-pixel text-sm">启用提示音</span>
        <button
          @click="settingsStore.toggleSound"
          class="pixel-button px-4 py-2 font-pixel text-xs"
          :class="settingsStore.soundEnabled ? 'bg-pixel-green text-black' : 'bg-gray-700 text-gray-400'"
        >
          {{ settingsStore.soundEnabled ? 'ON' : 'OFF' }}
        </button>
      </div>

      <div v-if="settingsStore.soundEnabled" class="mb-4">
        <label class="font-pixel text-xs block mb-2">音量: {{ Math.round(settingsStore.soundVolume * 100) }}%</label>
        <input
          type="range"
          min="0"
          max="100"
          :value="settingsStore.soundVolume * 100"
          @input="handleVolumeChange"
          class="w-full"
        />
      </div>

      <button
        @click="testSound"
        class="pixel-button pixel-border border-pixel-yellow text-pixel-yellow px-4 py-2 font-pixel text-xs hover:bg-pixel-yellow hover:text-black"
      >
        🔔 测试音效
      </button>
    </div>

    <!-- Default duration -->
    <div class="pixel-border p-6 mb-6 bg-pixel-bg">
      <h3 class="text-sm font-pixel text-pixel-green mb-4">⏱️ 默认时长</h3>

      <div class="flex gap-3">
        <button
          v-for="duration in [15, 25, 45, 60]"
          :key="duration"
          @click="settingsStore.defaultDuration = duration"
          class="pixel-button px-4 py-2 font-pixel text-xs"
          :class="settingsStore.defaultDuration === duration ? 'bg-pixel-green text-black' : 'pixel-border border-pixel-green'"
        >
          {{ duration }}分钟
        </button>
      </div>
    </div>

    <!-- Data management -->
    <div class="pixel-border p-6 mb-6 bg-pixel-bg">
      <h3 class="text-sm font-pixel text-pixel-green mb-4">💾 数据管理</h3>

      <div class="flex gap-4 mb-4">
        <ExportButton />
        <ImportButton @imported="handleImported" />
      </div>

      <button
        @click="handleClearData"
        class="pixel-button pixel-border border-pixel-pink text-pixel-pink px-6 py-3 font-pixel text-sm hover:bg-pixel-pink hover:text-black"
      >
        🗑️ 清除所有数据
      </button>
    </div>

    <!-- About -->
    <div class="pixel-border p-6 bg-pixel-bg text-center">
      <h3 class="text-lg font-pixel text-pixel-green mb-2">FOCUS FLOW</h3>
      <p class="text-xs font-pixel text-gray-400 mb-4">版本 0.1.0</p>
      <p class="text-xs font-pixel text-gray-500">复古像素风番茄钟</p>
      <p class="text-xs font-pixel text-gray-500 mt-2">保持专注，成就梦想 💪</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { useSettingsStore } from '@/stores/settings'
import ExportButton from '@/components/ExportButton.vue'
import ImportButton from '@/components/ImportButton.vue'

const settingsStore = useSettingsStore()

function handleVolumeChange(event: Event) {
  const target = event.target as HTMLInputElement
  settingsStore.setVolume(parseInt(target.value) / 100)
}

async function testSound() {
  await invoke('play_completion_sound')
}

function handleImported() {
  alert('数据已更新，请刷新页面查看')
}

async function handleClearData() {
  const confirmed = confirm('⚠️ 确定要清除所有数据吗？此操作不可恢复！')
  if (confirmed) {
    const doubleConfirm = prompt('请输入 "DELETE" 确认删除')
    if (doubleConfirm === 'DELETE') {
      alert('功能开发中...')
    }
  }
}
</script>
