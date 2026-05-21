<template>
  <div ref="viewportRef" class="app-fit-viewport">
    <div v-if="isTimerRoute" class="app-fit-shell" :style="shellStyle">
      <div ref="contentRef" class="app-fit-content" :style="contentStyle">
        <div class="app-shell">
          <AppHeader />
          <main class="app-main">
            <router-view />
          </main>
          <AppNav />
        </div>
      </div>
    </div>

    <div v-else class="app-shell">
      <AppHeader />
      <main class="app-main">
        <router-view />
      </main>
      <AppNav />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import AppHeader from '@/components/AppHeader.vue'
import AppNav from '@/components/AppNav.vue'
import { useSettingsStore } from '@/stores/settings'

useSettingsStore()

const route = useRoute()
const viewportRef = ref<HTMLElement | null>(null)
const contentRef = ref<HTMLElement | null>(null)
const viewportSize = ref({ width: 0, height: 0 })
const contentSize = ref({ width: 0, height: 0 })
let viewportObserver: ResizeObserver | null = null
let contentObserver: ResizeObserver | null = null
let measureFrame = 0

const isTimerRoute = computed(() => route.name === 'timer')

const appScale = computed(() => {
  if (!isTimerRoute.value) {
    return 1
  }

  const { width: viewportWidth, height: viewportHeight } = viewportSize.value
  const { width: contentWidth, height: contentHeight } = contentSize.value

  if (!viewportWidth || !viewportHeight || !contentWidth || !contentHeight) {
    return 1
  }

  return Math.min(viewportWidth / contentWidth, viewportHeight / contentHeight, 1)
})

const shellStyle = computed((): Record<string, string> => {
  const scale = appScale.value
  const { width, height } = contentSize.value

  if (!width || !height) {
    return {}
  }

  return {
    width: `${Math.round(width * scale)}px`,
    height: `${Math.round(height * scale)}px`,
  }
})

const contentStyle = computed(() => ({
  transform: `scale(${appScale.value})`,
}))

function measureViewport() {
  if (!viewportRef.value) return
  viewportSize.value = {
    width: viewportRef.value.clientWidth,
    height: viewportRef.value.clientHeight,
  }
}

function measureContent() {
  if (!contentRef.value) return
  contentSize.value = {
    width: contentRef.value.offsetWidth,
    height: contentRef.value.offsetHeight,
  }
}

function scheduleMeasure() {
  cancelAnimationFrame(measureFrame)
  measureFrame = requestAnimationFrame(() => {
    measureViewport()
    measureContent()
  })
}

function observeContent() {
  contentObserver?.disconnect()
  contentObserver = null

  if (!contentRef.value) return

  measureContent()
  contentObserver = new ResizeObserver(scheduleMeasure)
  contentObserver.observe(contentRef.value)
}

onMounted(async () => {
  if (!viewportRef.value) {
    return
  }

  await nextTick()
  measureViewport()
  observeContent()

  viewportObserver = new ResizeObserver(entries => {
    const entry = entries[0]
    if (!entry) return

    viewportSize.value = {
      width: entry.contentRect.width,
      height: entry.contentRect.height,
    }
  })

  viewportObserver.observe(viewportRef.value)
  scheduleMeasure()
})

watch(() => route.fullPath, async () => {
  await nextTick()
  observeContent()
  scheduleMeasure()
})

onUnmounted(() => {
  cancelAnimationFrame(measureFrame)
  viewportObserver?.disconnect()
  contentObserver?.disconnect()
})
</script>

<style scoped>
.app-fit-viewport {
  width: 100%;
  height: 100%;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
}

.app-fit-shell {
  position: relative;
  flex-shrink: 0;
}

.app-fit-content {
  width: 1000px;
  transform-origin: top left;
}

.app-shell {
  width: 100%;
  height: 100%;
  color: var(--pixel-text);
  display: flex;
  flex-direction: column;
}

.app-main {
  flex: 1;
  overflow: hidden;
}
</style>
