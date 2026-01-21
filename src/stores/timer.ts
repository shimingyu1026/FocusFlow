import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { FocusSession } from '@/types/database'

export const useTimerStore = defineStore('timer', () => {
  const isRunning = ref(false)
  const remainingSeconds = ref(0)
  const currentTask = ref('')

  async function startSession(duration: number, task: string) {
    await invoke('start_session', { duration, task })
    isRunning.value = true
    remainingSeconds.value = duration * 60
    currentTask.value = task
  }

  async function pauseSession() {
    await invoke('pause_session')
    isRunning.value = false
  }

  async function resumeSession() {
    await invoke('resume_session')
    isRunning.value = true
  }

  async function stopSession(completed: boolean) {
    await invoke('stop_session', { completed })
    isRunning.value = false
    remainingSeconds.value = 0
    currentTask.value = ''
  }

  async function loadSessions(limit?: number): Promise<FocusSession[]> {
    return await invoke('get_sessions', { limit })
  }

  async function deleteSession(id: string) {
    await invoke('delete_session', { id })
  }

  return {
    isRunning,
    remainingSeconds,
    currentTask,
    startSession,
    pauseSession,
    resumeSession,
    stopSession,
    loadSessions,
    deleteSession,
  }
})
