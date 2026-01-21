<template>
  <div class="pixel-border p-4 mb-4" style="background-color: var(--pixel-bg);">
    <div class="flex justify-between items-start gap-4">
      <div class="flex-1">
        <h3 class="text-sm mb-2" style="color: var(--pixel-primary);">{{ session.task || '未命名任务' }}</h3>
        <div class="flex gap-4 text-xs" style="color: var(--pixel-text-muted);">
          <span>{{ formatDate(session.startTime) }}</span>
          <span>{{ session.duration }}分钟</span>
          <span v-if="session.completed" style="color: var(--pixel-primary);">✓ 完成</span>
          <span v-else style="color: var(--pixel-secondary);">✗ 中断</span>
        </div>
        <div v-if="session.tags.length > 0" class="flex gap-2 mt-2">
          <span
            v-for="tag in session.tags"
            :key="tag"
            class="text-xs px-2 py-1 tag-badge"
          >
            {{ tag }}
          </span>
        </div>
      </div>
      <button
        @click="handleDelete"
        class="pixel-button text-lg"
        style="color: var(--pixel-secondary);"
      >
        🗑️
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { FocusSession } from '@/types/database'

const props = defineProps<{
  session: FocusSession
}>()

const emit = defineEmits<{
  delete: [id: string]
}>()

function formatDate(dateString: string): string {
  const date = new Date(dateString)
  const month = (date.getMonth() + 1).toString().padStart(2, '0')
  const day = date.getDate().toString().padStart(2, '0')
  const hours = date.getHours().toString().padStart(2, '0')
  const minutes = date.getMinutes().toString().padStart(2, '0')
  return `${month}-${day} ${hours}:${minutes}`
}

function handleDelete() {
  if (confirm('确定删除这条记录吗？')) {
    emit('delete', props.session.id)
  }
}
</script>

<style scoped>
.tag-badge {
  border: 2px solid var(--pixel-secondary);
  border-radius: 4px;
  color: var(--pixel-secondary);
}
</style>
