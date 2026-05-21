import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import { useSettingsStore } from './stores/settings'
import './index.css'

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.use(router)

const settingsStore = useSettingsStore(pinia)
settingsStore.loadSettings()
  .catch(error => {
    console.warn('Settings failed to load before mount', error)
  })
  .finally(() => {
    app.mount('#app')
  })
