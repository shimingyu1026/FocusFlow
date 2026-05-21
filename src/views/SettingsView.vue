<template>
  <div class="h-full overflow-y-auto p-6">
    <h2 class="text-xl font-pixel text-pixel-green mb-8 text-center">⚙️ 设置</h2>

    <!-- Appearance settings -->
    <div class="pixel-border p-6 mb-6 bg-pixel-bg">
      <h3 class="text-sm font-pixel text-pixel-green mb-4">🎨 外观</h3>

      <div class="mb-5">
        <p class="font-pixel text-xs text-pixel-text-muted mb-3">主题模式</p>
        <div class="flex flex-wrap gap-3">
          <button
            v-for="option in themeModeOptions"
            :key="option.value"
            @click="settingsStore.setThemeMode(option.value)"
            class="pixel-button px-4 py-2 font-pixel text-xs"
            :class="settingsStore.themeMode === option.value ? 'bg-pixel-green text-black' : 'pixel-border border-pixel-green'"
          >
            {{ option.label }}
          </button>
        </div>
      </div>

      <div>
        <p class="font-pixel text-xs text-pixel-text-muted mb-3">颜色主题</p>
        <div class="flex flex-wrap gap-3">
          <button
            v-for="option in themeAccentOptions"
            :key="option.value"
            @click="settingsStore.setThemeAccent(option.value)"
            class="pixel-button px-4 py-2 font-pixel text-xs min-w-[110px]"
            :class="settingsStore.themeAccent === option.value ? 'text-black' : ''"
            :style="{
              '--pixel-primary': option.primary,
              '--pixel-primary-dark': option.primaryDark,
              backgroundColor: settingsStore.themeAccent === option.value ? option.primary : 'var(--pixel-bg)',
              color: settingsStore.themeAccent === option.value ? '#020617' : option.primary,
            }"
          >
            {{ option.label }}
          </button>
        </div>
      </div>
    </div>

    <!-- Sound settings -->
    <div class="pixel-border p-6 mb-6 bg-pixel-bg">
      <h3 class="text-sm font-pixel text-pixel-green mb-4">🔊 声音</h3>

      <div class="flex items-center justify-between mb-4">
        <span class="font-pixel text-sm">启用提示音</span>
        <button
          @click="settingsStore.toggleSound"
          class="pixel-button px-4 py-2 font-pixel text-xs"
          :class="settingsStore.soundEnabled ? 'bg-pixel-green text-black' : ''"
          :style="settingsStore.soundEnabled ? undefined : { color: 'var(--pixel-text-muted)' }"
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

      <div class="flex flex-wrap gap-3">
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

      <div v-if="storageLocations" class="mt-5 space-y-3 text-xs">
        <div>
          <p class="font-pixel text-pixel-text-muted mb-1">专注记录数据库</p>
          <p class="break-all font-mono text-pixel-text">{{ storageLocations.databasePath }}</p>
        </div>
        <div>
          <p class="font-pixel text-pixel-text-muted mb-1">设置缓存文件</p>
          <p class="break-all font-mono text-pixel-text">{{ storageLocations.settingsPath }}</p>
        </div>
      </div>
    </div>

    <!-- Celebration settings -->
    <div class="pixel-border p-6 mb-6 bg-pixel-bg">
      <h3 class="text-sm font-pixel text-pixel-green mb-4">🎉 完成动画</h3>

      <div class="flex flex-wrap gap-3">
        <button
          v-for="option in celebrationOptions"
          :key="option.value"
          @click="settingsStore.setCelebrationStyle(option.value)"
          class="pixel-button px-4 py-2 font-pixel text-xs"
          :class="settingsStore.celebrationStyle === option.value ? 'bg-pixel-yellow text-black' : 'pixel-border border-pixel-yellow text-pixel-yellow'"
        >
          {{ option.label }}
        </button>
      </div>

      <p class="font-pixel text-xs text-pixel-text-muted mt-4">
        当前效果: {{ currentCelebrationLabel }}
      </p>
    </div>

    <!-- About -->
    <div class="pixel-border p-6 bg-pixel-bg text-center">
      <h3 class="text-lg font-pixel text-pixel-green mb-2">FOCUS FLOW</h3>
      <p class="text-xs font-pixel text-pixel-text-muted mb-4">版本 0.2.0</p>
      <p class="text-xs font-pixel text-pixel-text-muted">复古像素风番茄钟</p>
      <p class="text-xs font-pixel text-pixel-text-muted mt-2">保持专注，成就梦想 💪</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { computed, onMounted, ref } from 'vue'
import { useSettingsStore, type CelebrationStyle, type ThemeAccent, type ThemeMode } from '@/stores/settings'
import ExportButton from '@/components/ExportButton.vue'
import ImportButton from '@/components/ImportButton.vue'
import { emitSessionsUpdated } from '@/utils/sessionEvents'

const settingsStore = useSettingsStore()

interface StorageLocations {
  databasePath: string
  settingsPath: string
  appDataDir: string
  appConfigDir: string
}

const storageLocations = ref<StorageLocations | null>(null)

const themeModeOptions: Array<{ label: string; value: ThemeMode }> = [
  { label: '暗色', value: 'dark' },
  { label: '亮色', value: 'light' },
]

const themeAccentOptions: Array<{ label: string; value: ThemeAccent; primary: string; primaryDark: string }> = [
  { label: '海盐青', value: 'ocean', primary: '#14b8a6', primaryDark: '#0f766e' },
  { label: '落日粉', value: 'sunset', primary: '#fb7185', primaryDark: '#be123c' },
  { label: '街机绿', value: 'arcade', primary: '#84cc16', primaryDark: '#4d7c0f' },
]

const celebrationOptions: Array<{ label: string; value: CelebrationStyle }> = [
  { label: '彩纸雨', value: 'confetti' },
  { label: '星星雨', value: 'stars' },
  { label: '烟花', value: 'fireworks' },
]

const currentCelebrationLabel = computed(() => {
  const current = celebrationOptions.find(option => option.value === settingsStore.celebrationStyle)
  return current?.label ?? '彩纸雨'
})

function handleVolumeChange(event: Event) {
  const target = event.target as HTMLInputElement
  settingsStore.setVolume(parseInt(target.value) / 100)
}

async function testSound() {
  await invoke('play_completion_sound')
}

async function loadStorageLocations() {
  try {
    storageLocations.value = await invoke<StorageLocations>('get_storage_locations')
  } catch {
    storageLocations.value = null
  }
}

function handleImported() {
  emitSessionsUpdated()
}

async function handleClearData() {
  const confirmed = confirm('⚠️ 确定要清除所有数据吗？此操作不可恢复！')
  if (!confirmed) return

  const doubleConfirm = prompt('请输入 "DELETE" 确认删除')
  if (doubleConfirm !== 'DELETE') return

  try {
    const deleted = await invoke<number>('clear_all_data')
    emitSessionsUpdated()
    alert(`✅ 已清除 ${deleted} 条记录`)
  } catch (error) {
    alert('❌ 清除失败: ' + error)
  }
}

onMounted(() => {
  loadStorageLocations()
})
</script>
