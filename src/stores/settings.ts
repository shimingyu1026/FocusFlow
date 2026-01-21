import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export const useSettingsStore = defineStore('settings', () => {
  const soundEnabled = ref(true)
  const soundVolume = ref(0.7)
  const defaultDuration = ref(25)

  // Load from localStorage
  const saved = localStorage.getItem('focusflow-settings')
  if (saved) {
    try {
      const parsed = JSON.parse(saved)
      soundEnabled.value = parsed.soundEnabled ?? true
      soundVolume.value = parsed.soundVolume ?? 0.7
      defaultDuration.value = parsed.defaultDuration ?? 25
    } catch (e) {
      console.error('Failed to load settings', e)
    }
  }

  // Auto-save to localStorage
  watch([soundEnabled, soundVolume, defaultDuration], () => {
    localStorage.setItem('focusflow-settings', JSON.stringify({
      soundEnabled: soundEnabled.value,
      soundVolume: soundVolume.value,
      defaultDuration: defaultDuration.value,
    }))
  }, { deep: true })

  function toggleSound() {
    soundEnabled.value = !soundEnabled.value
  }

  function setVolume(volume: number) {
    soundVolume.value = Math.max(0, Math.min(1, volume))
  }

  return {
    soundEnabled,
    soundVolume,
    defaultDuration,
    toggleSound,
    setVolume,
  }
})
