import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { FocusSession } from '@/types/database'
import { isDesktopRuntime } from '@/utils/runtime'

const LOCAL_SESSIONS_KEY = 'focusflow-sessions'

function createSessionId(): string {
  return crypto.randomUUID?.() ?? `${Date.now()}-${Math.random().toString(16).slice(2)}`
}

function loadLocalSessions(): FocusSession[] {
  const raw = localStorage.getItem(LOCAL_SESSIONS_KEY)
  if (!raw) return []

  try {
    const sessions = JSON.parse(raw) as FocusSession[]
    return Array.isArray(sessions) ? sessions : []
  } catch {
    return []
  }
}

function saveLocalSessions(sessions: FocusSession[]) {
  localStorage.setItem(LOCAL_SESSIONS_KEY, JSON.stringify(sessions))
}

export const useTimerStore = defineStore('timer', () => {
  const isRunning = ref(false)
  const remainingSeconds = ref(0)
  const currentTask = ref('')
  const currentTags = ref<string[]>([])
  const startedAt = ref('')

  async function startSession(duration: number, task: string, tags: string[] = []) {
    if (duration <= 0) {
      throw new Error('专注时长必须大于 0')
    }

    const normalizedTask = task.trim()
    const normalizedTags = Array.from(new Set(tags.map(tag => tag.trim()).filter(Boolean)))

    if (isDesktopRuntime()) {
      await invoke('start_session', { duration, task: normalizedTask, tags: normalizedTags })
    }

    isRunning.value = true
    remainingSeconds.value = duration * 60
    currentTask.value = normalizedTask
    currentTags.value = normalizedTags
    startedAt.value = new Date().toISOString()
  }

  async function pauseSession() {
    if (isDesktopRuntime()) {
      await invoke('pause_session')
    }
    isRunning.value = false
  }

  async function resumeSession() {
    if (isDesktopRuntime()) {
      await invoke('resume_session')
    }
    isRunning.value = true
  }

  async function stopSession(completed: boolean, elapsedSeconds: number) {
    if (isDesktopRuntime()) {
      await invoke('stop_session', { completed, elapsed_seconds: elapsedSeconds })
    } else if (startedAt.value) {
      const safeElapsed = Math.max(0, elapsedSeconds)
      const session: FocusSession = {
        id: createSessionId(),
        task: currentTask.value,
        duration: safeElapsed === 0 ? 0 : Math.ceil(safeElapsed / 60),
        startTime: startedAt.value,
        endTime: new Date().toISOString(),
        completed,
        tags: currentTags.value,
      }
      saveLocalSessions([session, ...loadLocalSessions()])
    }

    isRunning.value = false
    remainingSeconds.value = 0
    currentTask.value = ''
    currentTags.value = []
    startedAt.value = ''
  }

  async function loadSessions(limit?: number): Promise<FocusSession[]> {
    if (isDesktopRuntime()) {
      return await invoke('get_sessions', { limit })
    }

    const sessions = loadLocalSessions()
    return typeof limit === 'number' && limit > 0 ? sessions.slice(0, limit) : sessions
  }

  async function deleteSession(id: string) {
    if (isDesktopRuntime()) {
      await invoke('delete_session', { id })
      return
    }

    saveLocalSessions(loadLocalSessions().filter(session => session.id !== id))
  }

  async function clearAllData(): Promise<number> {
    if (isDesktopRuntime()) {
      return await invoke('clear_all_data')
    }

    const count = loadLocalSessions().length
    localStorage.removeItem(LOCAL_SESSIONS_KEY)
    return count
  }

  async function exportData(): Promise<string> {
    if (isDesktopRuntime()) {
      return await invoke('export_data')
    }

    return JSON.stringify(loadLocalSessions(), null, 2)
  }

  async function importData(jsonData: string): Promise<number> {
    if (isDesktopRuntime()) {
      return await invoke('import_data', { jsonData })
    }

    const sessions = JSON.parse(jsonData) as FocusSession[]
    if (!Array.isArray(sessions)) {
      throw new Error('导入文件格式不正确')
    }

    saveLocalSessions(sessions)
    return sessions.length
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
    clearAllData,
    exportData,
    importData,
  }
})
