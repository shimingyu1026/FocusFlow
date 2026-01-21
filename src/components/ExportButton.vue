<template>
  <button
    @click="handleExport"
    class="pixel-button pixel-border border-pixel-green text-pixel-green px-6 py-3 font-pixel text-sm hover:bg-pixel-green hover:text-black"
  >
    📤 导出数据
  </button>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { save } from '@tauri-apps/plugin-dialog'
import { writeTextFile } from '@tauri-apps/plugin-fs'

async function handleExport() {
  try {
    const filePath = await save({
      defaultPath: 'focusflow-export.json',
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }]
    })

    if (filePath) {
      const jsonData = await invoke<string>('export_data')
      await writeTextFile(filePath, jsonData)
      alert('✅ 数据导出成功！')
    }
  } catch (error) {
    alert('❌ 导出失败: ' + error)
  }
}
</script>
