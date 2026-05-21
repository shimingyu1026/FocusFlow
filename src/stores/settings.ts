import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export type ThemeMode = 'dark' | 'light'
export type ThemeAccent = 'ocean' | 'sunset' | 'arcade'
export type CelebrationStyle = 'confetti' | 'stars' | 'fireworks'

export interface AppSettingsSnapshot {
  soundEnabled: boolean
  soundVolume: number
  defaultDuration: number
  themeMode: ThemeMode
  themeAccent: ThemeAccent
  celebrationStyle: CelebrationStyle
}

const SETTINGS_STORAGE_KEY = 'focusflow-settings'
const isDesktopRuntime = () => typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window

export const useSettingsStore = defineStore('settings', () => {
  const soundEnabled = ref(true)
  const soundVolume = ref(0.7)
  const defaultDuration = ref(25)
  const themeMode = ref<ThemeMode>('dark')
  const themeAccent = ref<ThemeAccent>('ocean')
  const celebrationStyle = ref<CelebrationStyle>('confetti')
  const isLoaded = ref(false)

  function applyAppearance() {
    document.documentElement.dataset.themeMode = themeMode.value
    document.documentElement.dataset.themeAccent = themeAccent.value
  }

  function snapshot(): AppSettingsSnapshot {
    return {
      soundEnabled: soundEnabled.value,
      soundVolume: soundVolume.value,
      defaultDuration: defaultDuration.value,
      themeMode: themeMode.value,
      themeAccent: themeAccent.value,
      celebrationStyle: celebrationStyle.value,
    }
  }

  function applySettings(settings: Partial<AppSettingsSnapshot>) {
    soundEnabled.value = settings.soundEnabled ?? true
    soundVolume.value = Math.max(0, Math.min(1, settings.soundVolume ?? 0.7))
    defaultDuration.value = settings.defaultDuration && settings.defaultDuration > 0
      ? Math.min(settings.defaultDuration, 480)
      : 25
    themeMode.value = settings.themeMode === 'light' ? 'light' : 'dark'
    themeAccent.value = ['ocean', 'sunset', 'arcade'].includes(settings.themeAccent ?? '')
      ? settings.themeAccent as ThemeAccent
      : 'ocean'
    celebrationStyle.value = ['confetti', 'stars', 'fireworks'].includes(settings.celebrationStyle ?? '')
      ? settings.celebrationStyle as CelebrationStyle
      : 'confetti'
    applyAppearance()
  }

  function loadLocalSettings(): AppSettingsSnapshot | null {
    const saved = localStorage.getItem(SETTINGS_STORAGE_KEY)
    if (!saved) return null

    try {
      return JSON.parse(saved) as AppSettingsSnapshot
    } catch (e) {
      console.error('Failed to load settings', e)
      return null
    }
  }

  function saveLocalSettings() {
    localStorage.setItem(SETTINGS_STORAGE_KEY, JSON.stringify(snapshot()))
  }

  async function loadSettings() {
    const localSettings = loadLocalSettings()
    if (localSettings) {
      applySettings(localSettings)
    }

    if (isDesktopRuntime()) {
      try {
        const saved = await invoke<AppSettingsSnapshot>('get_settings')
        applySettings(saved)
      } catch (error) {
        console.warn('Falling back to local settings storage', error)
      }
    }

    isLoaded.value = true
    applyAppearance()
  }

  applyAppearance()

  watch([soundEnabled, soundVolume, defaultDuration, themeMode, themeAccent, celebrationStyle], async () => {
    if (!isLoaded.value) return

    applyAppearance()
    if (isDesktopRuntime()) {
      try {
        await invoke<AppSettingsSnapshot>('save_settings', { settings: snapshot() })
        return
      } catch (error) {
        console.warn('Failed to save desktop settings, using local fallback', error)
      }
    }

    saveLocalSettings()
  }, { deep: true })

  function toggleSound() {
    soundEnabled.value = !soundEnabled.value
  }

  function setVolume(volume: number) {
    soundVolume.value = Math.max(0, Math.min(1, volume))
  }

  function setThemeMode(mode: ThemeMode) {
    themeMode.value = mode
  }

  function setThemeAccent(accent: ThemeAccent) {
    themeAccent.value = accent
  }

  function setCelebrationStyle(style: CelebrationStyle) {
    celebrationStyle.value = style
  }

  return {
    soundEnabled,
    soundVolume,
    defaultDuration,
    themeMode,
    themeAccent,
    celebrationStyle,
    isLoaded,
    loadSettings,
    toggleSound,
    setVolume,
    setThemeMode,
    setThemeAccent,
    setCelebrationStyle,
  }
})
