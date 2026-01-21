<template>
  <div class="pixel-border p-4 bg-pixel-bg mb-4">
    <div class="flex justify-between items-start gap-4">
      <div class="flex-1">
        <h3 class="font-pixel text-pixel-green text-sm mb-2">{{ session.task || '未命名任务' }}</h3>
        <div class="flex gap-4 text-xs font-pixel text-gray-400">
          <span>{{ formatDate(session.startTime) }}</span>
          <span>{{ session.duration }}分钟</span>
          <span v-if="session.completed" class="text-pixel-green">✓ 完成</span>
          <span v-else class="text-pixel-pink">✗ 中断</span>
        </div>
        <div v-if="session.tags.length > 0" class="flex gap-2 mt-2">
          <span
            v-for="tag in session.tags"
            :key="tag"
            class="text-xs px-2 py-1 pixel-border border-pixel-purple text-pixel-purple"
          >
            {{ tag }}
          </span>
        </div>
      </div>
      <button
        @click="handleDelete"
        class="pixel-button text-pixel-pink hover:text-red-500 text-lg"
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
