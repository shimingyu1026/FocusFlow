<template>
  <div class="h-full flex flex-col">
    <!-- Filter bar -->
    <div class="pixel-border p-4 mb-4 bg-pixel-bg">
      <div class="flex gap-4 items-center">
        <span class="font-pixel text-sm text-pixel-green">筛选:</span>
        <button
          @click="filterTag = ''"
          class="pixel-button px-3 py-1 text-xs font-pixel"
          :class="filterTag === '' ? 'bg-pixel-green text-black' : 'pixel-border border-pixel-green'"
        >
          全部
        </button>
        <button
          v-for="tag in allTags"
          :key="tag"
          @click="filterTag = tag"
          class="pixel-button px-3 py-1 text-xs font-pixel"
          :class="filterTag === tag ? 'bg-pixel-green text-black' : 'pixel-border border-pixel-green'"
        >
          {{ tag }}
        </button>
      </div>
    </div>

    <!-- Session list -->
    <div class="flex-1 overflow-y-auto px-4 pb-4">
      <div v-if="filteredSessions.length === 0" class="text-center text-gray-500 font-pixel text-sm mt-12">
        暂无记录
      </div>
      <SessionCard
        v-for="session in filteredSessions"
        :key="session.id"
        :session="session"
        @delete="handleDelete"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import SessionCard from './SessionCard.vue'
import type { FocusSession } from '@/types/database'

const props = defineProps<{
  sessions: FocusSession[]
}>()

const emit = defineEmits<{
  delete: [id: string]
}>()

const filterTag = ref('')

const allTags = computed(() => {
  const tags = new Set<string>()
  props.sessions.forEach(session => {
    session.tags.forEach(tag => tags.add(tag))
  })
  return Array.from(tags)
})

const filteredSessions = computed(() => {
  if (!filterTag.value) {
    return props.sessions
  }
  return props.sessions.filter(session =>
    session.tags.includes(filterTag.value)
  )
})

function handleDelete(id: string) {
  emit('delete', id)
}
</script>
