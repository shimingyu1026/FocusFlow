<template>
  <nav class="app-nav">
    <router-link
      to="/"
      class="app-nav__link"
      :class="{ 'nav-active': $route.name === 'timer' }"
    >
      <span class="app-nav__icon">⏱️</span>
      <span class="app-nav__label">计时器</span>
    </router-link>

    <router-link
      to="/history"
      class="app-nav__link"
      :class="{ 'nav-active': $route.name === 'history' }"
    >
      <span class="app-nav__icon">📝</span>
      <span class="app-nav__label">历史</span>
    </router-link>

    <router-link
      to="/statistics"
      class="app-nav__link"
      :class="{ 'nav-active': $route.name === 'statistics' }"
    >
      <span class="app-nav__icon">📊</span>
      <span class="app-nav__label">统计</span>
    </router-link>

    <router-link
      to="/settings"
      class="app-nav__link"
      :class="{ 'nav-active': $route.name === 'settings' }"
    >
      <span class="app-nav__icon">⚙️</span>
      <span class="app-nav__label">设置</span>
    </router-link>
  </nav>
</template>

<style scoped>
.app-nav {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 10px;
  border-top: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(15, 23, 42, 0.18);
  padding: 10px 14px 12px;
  backdrop-filter: blur(18px);
}

.app-nav__link {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  border: 1px solid rgba(148, 163, 184, 0.14);
  border-radius: 16px;
  background: rgba(255, 255, 255, 0.04);
  color: var(--pixel-text-muted);
  padding: 12px 10px;
  text-decoration: none;
  transition:
    transform 0.16s ease,
    border-color 0.16s ease,
    background-color 0.16s ease,
    color 0.16s ease;
}

.app-nav__link:hover {
  transform: translateY(-1px);
  border-color: rgba(20, 184, 166, 0.22);
  color: var(--pixel-text);
}

.app-nav__icon {
  font-size: 1rem;
}

.app-nav__label {
  font-size: 1rem;
}

.nav-active {
  border-color: rgba(20, 184, 166, 0.3);
  background:
    linear-gradient(135deg, rgba(20, 184, 166, 0.14), rgba(249, 115, 22, 0.08)),
    rgba(255, 255, 255, 0.06);
  color: var(--pixel-text);
  box-shadow: inset 0 0 0 1px rgba(20, 184, 166, 0.08);
}

@media (max-width: 420px) {
  .app-nav {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}
</style>
