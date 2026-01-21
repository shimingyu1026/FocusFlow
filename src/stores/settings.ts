import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useSettingsStore = defineStore('settings', () => {
  const soundEnabled = ref(true)
  const soundVolume = ref(0.7)
  const defaultDuration = ref(25)

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
