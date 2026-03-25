<template>
  <Teleport to="body">
    <Transition name="fade">
      <div
        v-if="visible"
        class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
        @click="handleClick"
      >
        <div class="text-center p-8">
          <!-- Celebration container -->
          <div class="relative">
            <!-- Animated Emoji -->
            <div class="text-8xl mb-6 animate-bounce">{{ headlineEmoji }}</div>

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

          <!-- Celebration particles -->
          <div class="absolute inset-0 pointer-events-none overflow-hidden">
            <span
              v-for="particle in particles"
              :key="particle.id"
              class="particle"
              :class="`particle--${props.variant}`"
              :style="particle.style"
            >
              {{ particle.symbol }}
            </span>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, ref, type CSSProperties } from 'vue'
import type { CelebrationStyle } from '@/stores/settings'

interface Props {
  duration?: number
  todayCount?: number
  variant?: CelebrationStyle
}

const props = withDefaults(defineProps<Props>(), {
  duration: 25,
  todayCount: 1,
  variant: 'confetti',
})

const emit = defineEmits<{
  close: []
}>()

const visible = ref(true)

const headlineEmoji = computed(() => {
  const emojis: Record<CelebrationStyle, string> = {
    confetti: '🎉',
    stars: '✨',
    fireworks: '🎆',
  }
  return emojis[props.variant]
})

const title = computed(() => {
  const messageMap: Record<CelebrationStyle, string[]> = {
    confetti: ['太棒了！', '完成！', '专注成功！', '优秀！', '继续保持！'],
    stars: ['闪闪发光！', '今天状态在线！', '专注值拉满！', '节奏很好！', '稳定输出！'],
    fireworks: ['漂亮收尾！', '火力全开！', '高光时刻！', '势头正猛！', '再来一轮！'],
  }
  const messages = messageMap[props.variant]
  return messages[Math.floor(Math.random() * messages.length)]
})

const motivationalMessage = computed(() => {
  const messageMap: Record<CelebrationStyle, string[]> = {
    confetti: [
      '每一次专注都是向目标迈进的一步！',
      '你的专注力正在不断提升！',
      '休息一下，然后继续前进！',
      '你已经养成了良好的习惯！',
      '坚持下去，你会变得更强大！',
    ],
    stars: [
      '状态正亮，继续把今天点亮。',
      '稳定的专注，正在变成你的默认节奏。',
      '你已经抓住感觉了，下一轮会更顺。',
      '每一段安静投入，都会积累成真正的优势。',
      '节奏稳住，你会比昨天更从容。',
    ],
    fireworks: [
      '这一轮收得漂亮，下一轮继续发力。',
      '能量已经起来了，别让 momentum 掉下去。',
      '高质量专注会自己发光。',
      '把这一股劲延续下去，今天会很不一样。',
      '你正在进入最好的工作状态。',
    ],
  }
  const messages = messageMap[props.variant]
  return messages[Math.floor(Math.random() * messages.length)]
})

const stats = computed(() => ({
  duration: props.duration,
  todayCount: props.todayCount,
}))

type ParticleModel = {
  id: string
  symbol: string
  style: CSSProperties
}

const particles = computed<ParticleModel[]>(() => {
  const countMap: Record<CelebrationStyle, number> = {
    confetti: 42,
    stars: 28,
    fireworks: 32,
  }

  return Array.from({ length: countMap[props.variant] }, (_, index) => ({
    id: `${props.variant}-${index}`,
    symbol: getParticleSymbol(index),
    style: getParticleStyle(index),
  }))
})

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

function getParticleSymbol(index: number) {
  const symbolMap: Record<CelebrationStyle, string[]> = {
    confetti: ['■', '◆', '●', '▲'],
    stars: ['✦', '✧', '★', '⋆'],
    fireworks: ['•', '✺', '✹', '✷'],
  }

  const symbols = symbolMap[props.variant]
  return symbols[index % symbols.length]
}

function getParticleStyle(index: number) {
  const colors = ['#fb7185', '#f59e0b', '#38bdf8', '#4ade80', '#a78bfa', '#f97316']
  const color = colors[index % colors.length]
  const delay = (index % 8) * 0.18

  if (props.variant === 'fireworks') {
    const angle = (index / 32) * Math.PI * 2
    const distance = 90 + (index % 6) * 18
    const dx = Math.cos(angle) * distance
    const dy = Math.sin(angle) * distance

    return {
      color,
      left: '50%',
      top: '50%',
      '--dx': `${dx}px`,
      '--dy': `${dy}px`,
      animationDelay: `${delay}s`,
      animationDuration: '1.6s',
      fontSize: `${18 + (index % 3) * 4}px`,
    }
  }

  return {
    color,
    left: `${(index * 13) % 100}%`,
    top: props.variant === 'stars' ? `${(index * 7) % 35}%` : '-12%',
    animationDelay: `${delay}s`,
    animationDuration: props.variant === 'stars' ? `${3.4 + (index % 4) * 0.35}s` : `${2.6 + (index % 5) * 0.25}s`,
    fontSize: `${props.variant === 'stars' ? 20 + (index % 3) * 4 : 16 + (index % 4) * 3}px`,
  }
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

.particle {
  position: absolute;
  display: block;
  line-height: 1;
  text-shadow: 0 0 10px currentColor;
}

.particle--confetti {
  animation-name: confetti-fall;
  animation-timing-function: linear;
  animation-iteration-count: infinite;
}

.particle--stars {
  animation-name: star-float;
  animation-timing-function: ease-in-out;
  animation-iteration-count: infinite;
}

.particle--fireworks {
  animation-name: firework-burst;
  animation-timing-function: ease-out;
  animation-iteration-count: infinite;
  opacity: 0;
}

@keyframes confetti-fall {
  0% {
    transform: translate3d(0, 0, 0) rotate(0deg);
    opacity: 1;
  }
  100% {
    transform: translate3d(0, 105vh, 0) rotate(720deg);
    opacity: 0;
  }
}

@keyframes star-float {
  0% {
    transform: translate3d(-20px, -10px, 0) scale(0.8);
    opacity: 0;
  }
  20% {
    opacity: 1;
  }
  50% {
    transform: translate3d(12px, 28px, 0) scale(1.15);
    opacity: 1;
  }
  100% {
    transform: translate3d(-10px, 80px, 0) scale(0.85);
    opacity: 0;
  }
}

@keyframes firework-burst {
  0% {
    transform: translate3d(0, 0, 0) scale(0.2);
    opacity: 0;
  }
  15% {
    opacity: 1;
  }
  100% {
    transform: translate3d(var(--dx), var(--dy), 0) scale(1.1);
    opacity: 0;
  }
}
</style>
