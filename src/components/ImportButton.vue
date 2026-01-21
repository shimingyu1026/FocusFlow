<template>
  <button
    @click="handleImport"
    class="pixel-button pixel-border border-pixel-blue text-pixel-blue px-6 py-3 font-pixel text-sm hover:bg-pixel-blue hover:text-black"
  >
    📥 导入数据
  </button>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { readTextFile } from '@tauri-apps/plugin-fs'

const emit = defineEmits<{
  imported: []
}>()

async function handleImport() {
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }]
    })

    if (selected && typeof selected === 'string') {
      const confirmed = confirm('⚠️ 导入将覆盖现有数据，确定继续吗？')
      if (!confirmed) return

      const jsonData = await readTextFile(selected)
      const count = await invoke<number>('import_data', { jsonData })
      alert(`✅ 成功导入 ${count} 条记录！`)
      emit('imported')
    }
  } catch (error) {
    alert('❌ 导入失败: ' + error)
  }
}
</script>
