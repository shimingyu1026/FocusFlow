import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export type ThemeMode = 'dark' | 'light'
export type ThemeAccent = 'ocean' | 'sunset' | 'arcade'
export type CelebrationStyle = 'confetti' | 'stars' | 'fireworks'

export const useSettingsStore = defineStore('settings', () => {
  const soundEnabled = ref(true)
  const soundVolume = ref(0.7)
  const defaultDuration = ref(25)
  const themeMode = ref<ThemeMode>('dark')
  const themeAccent = ref<ThemeAccent>('ocean')
  const celebrationStyle = ref<CelebrationStyle>('confetti')

  function applyAppearance() {
    document.documentElement.dataset.themeMode = themeMode.value
    document.documentElement.dataset.themeAccent = themeAccent.value
  }

  // Load from localStorage
  const saved = localStorage.getItem('focusflow-settings')
  if (saved) {
    try {
      const parsed = JSON.parse(saved)
      soundEnabled.value = parsed.soundEnabled ?? true
      soundVolume.value = parsed.soundVolume ?? 0.7
      defaultDuration.value = parsed.defaultDuration ?? 25
      themeMode.value = parsed.themeMode ?? 'dark'
      themeAccent.value = parsed.themeAccent ?? 'ocean'
      celebrationStyle.value = parsed.celebrationStyle ?? 'confetti'
    } catch (e) {
      console.error('Failed to load settings', e)
    }
  }
  applyAppearance()

  // Auto-save to localStorage
  watch([soundEnabled, soundVolume, defaultDuration, themeMode, themeAccent, celebrationStyle], () => {
    localStorage.setItem('focusflow-settings', JSON.stringify({
      soundEnabled: soundEnabled.value,
      soundVolume: soundVolume.value,
      defaultDuration: defaultDuration.value,
      themeMode: themeMode.value,
      themeAccent: themeAccent.value,
      celebrationStyle: celebrationStyle.value,
    }))
    applyAppearance()
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
    toggleSound,
    setVolume,
    setThemeMode,
    setThemeAccent,
    setCelebrationStyle,
  }
})
