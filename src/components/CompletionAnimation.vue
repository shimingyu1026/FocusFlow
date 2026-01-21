<template>
  <Teleport to="body">
    <Transition name="fade">
      <div
        v-if="visible"
        class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
        @click="handleClick"
      >
        <div class="text-center p-8">
          <!-- Confetti Animation Container -->
          <div class="relative">
            <!-- Animated Emoji -->
            <div class="text-8xl mb-6 animate-bounce">🎉</div>

            <!-- Success Message -->
            <h2 class="text-4xl font-bold mb-4 animate-pulse text-white">
              {{ title }}
            </h2>

            <!-- Stats -->
            <div v-if="stats" class="text-xl text-white mb-6">
              <p class="mb-2">⏱️ 专注时长: {{ stats.duration }} 分钟</p>
              <p>🔥 今日完成: {{ stats.todayCount }} 次</p>
            </div>

            <!-- Motivational Message -->
            <p class="text-lg text-pixel-green mb-8">
              {{ motivationalMessage }}
            </p>

            <!-- Close Button -->
            <button
              class="px-8 py-3 bg-pixel-green hover:bg-pixel-green/80 text-black font-pixel text-lg transition-all transform hover:scale-105"
              @click.stop="handleClose"
            >
              继续加油！
            </button>
          </div>

          <!-- Confetti Particles -->
          <div class="absolute inset-0 pointer-events-none overflow-hidden">
            <div
              v-for="i in 50"
              :key="i"
              class="confetti"
              :style="getConfettiStyle(i)"
            />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

interface Props {
  duration?: number
  todayCount?: number
}

const props = withDefaults(defineProps<Props>(), {
  duration: 25,
  todayCount: 1,
})

const emit = defineEmits<{
  close: []
}>()

const visible = ref(true)

const title = computed(() => {
  const messages = [
    '太棒了！',
    '完成！',
    '专注成功！',
    '优秀！',
    '继续保持！',
  ]
  return messages[Math.floor(Math.random() * messages.length)]
})

const motivationalMessage = computed(() => {
  const messages = [
    '每一次专注都是向目标迈进的一步！',
    '你的专注力正在不断提升！',
    '休息一下，然后继续前进！',
    '你已经养成了良好的习惯！',
    '坚持下去，你会变得更强大！',
  ]
  return messages[Math.floor(Math.random() * messages.length)]
})

const stats = computed(() => ({
  duration: props.duration,
  todayCount: props.todayCount,
}))

function handleClose() {
  visible.value = false
  setTimeout(() => {
    emit('close')
  }, 300) // Wait for fade-out animation
}

function handleClick() {
  // Close when clicking outside the content
  handleClose()
}

function getConfettiStyle(index: number) {
  const colors = ['#ff6b6b', '#4ecdc4', '#45b7d1', '#f9ca24', '#6c5ce7', '#a29bfe']
  const color = colors[index % colors.length]
  const left = Math.random() * 100
  const animationDelay = Math.random() * 2
  const animationDuration = 2 + Math.random() * 2

  return {
    position: 'absolute',
    left: `${left}%`,
    top: '-20px',
    width: '10px',
    height: '10px',
    backgroundColor: color,
    animation: `fall ${animationDuration}s linear ${animationDelay}s infinite`,
  } as const
}
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

@keyframes fall {
  0% {
    transform: translateY(0) rotate(0deg);
    opacity: 1;
  }
  100% {
    transform: translateY(100vh) rotate(720deg);
    opacity: 0;
  }
}

.confetti {
  border-radius: 2px;
}
</style>
