<template>
  <span>
    <button
      @click="handleImport"
      class="pixel-button pixel-border border-pixel-blue text-pixel-blue px-6 py-3 font-pixel text-sm hover:bg-pixel-blue hover:text-black"
    >
      导入数据
    </button>
    <input
      ref="fileInput"
      type="file"
      accept="application/json,.json"
      class="hidden"
      @change="handleWebFileSelected"
    />
  </span>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { readTextFile } from '@tauri-apps/plugin-fs'
import { useTimerStore } from '@/stores/timer'
import { isDesktopRuntime } from '@/utils/runtime'

const emit = defineEmits<{
  imported: []
}>()

const timerStore = useTimerStore()
const fileInput = ref<HTMLInputElement | null>(null)

async function importJsonData(jsonData: string) {
  const confirmed = confirm('⚠️ 导入将覆盖现有数据，确定继续吗？')
  if (!confirmed) return

  const count = await timerStore.importData(jsonData)
  alert(`✅ 成功导入 ${count} 条记录！`)
  emit('imported')
}

async function handleImport() {
  try {
    if (!isDesktopRuntime()) {
      fileInput.value?.click()
      return
    }

    const selected = await open({
      multiple: false,
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }]
    })

    if (selected && typeof selected === 'string') {
      const jsonData = await readTextFile(selected)
      await importJsonData(jsonData)
    }
  } catch (error) {
    alert('❌ 导入失败: ' + error)
  }
}

async function handleWebFileSelected(event: Event) {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (!file) return

  try {
    await importJsonData(await file.text())
  } catch (error) {
    alert('❌ 导入失败: ' + error)
  } finally {
    target.value = ''
  }
}
</script>
