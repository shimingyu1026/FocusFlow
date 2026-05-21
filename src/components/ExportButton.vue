<template>
  <button
    @click="handleExport"
    class="pixel-button pixel-border border-pixel-green text-pixel-green px-6 py-3 font-pixel text-sm hover:bg-pixel-green hover:text-black"
  >
    📤 导出数据
  </button>
</template>

<script setup lang="ts">
import { save } from '@tauri-apps/plugin-dialog'
import { writeTextFile } from '@tauri-apps/plugin-fs'
import { useTimerStore } from '@/stores/timer'
import { isDesktopRuntime } from '@/utils/runtime'

const timerStore = useTimerStore()

async function handleExport() {
  try {
    const jsonData = await timerStore.exportData()

    if (!isDesktopRuntime()) {
      const blob = new Blob([jsonData], { type: 'application/json' })
      const url = URL.createObjectURL(blob)
      const link = document.createElement('a')
      link.href = url
      link.download = 'focusflow-export.json'
      link.click()
      URL.revokeObjectURL(url)
      return
    }

    const filePath = await save({
      defaultPath: 'focusflow-export.json',
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }]
    })

    if (filePath) {
      await writeTextFile(filePath, jsonData)
      alert('✅ 数据导出成功！')
    }
  } catch (error) {
    alert('❌ 导出失败: ' + error)
  }
}
</script>
