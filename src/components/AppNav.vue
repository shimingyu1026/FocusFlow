<template>
  <nav class="app-nav">
    <router-link
      to="/"
      class="app-nav__link"
      :class="{ 'nav-active': $route.name === 'timer' }"
    >
      <span class="app-nav__label">计时器</span>
    </router-link>

    <router-link
      to="/history"
      class="app-nav__link"
      :class="{ 'nav-active': $route.name === 'history' }"
    >
      <span class="app-nav__label">历史</span>
    </router-link>

    <router-link
      to="/statistics"
      class="app-nav__link"
      :class="{ 'nav-active': $route.name === 'statistics' }"
    >
      <span class="app-nav__label">统计</span>
    </router-link>

    <router-link
      to="/settings"
      class="app-nav__link"
      :class="{ 'nav-active': $route.name === 'settings' }"
    >
      <span class="app-nav__label">设置</span>
    </router-link>
  </nav>
</template>

<style scoped>
.app-nav {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 8px;
  border-top: 1px solid var(--surface-line);
  background: rgba(25, 24, 15, 0.88);
  padding: 12px 18px 14px;
  backdrop-filter: blur(18px);
}

.app-nav__link {
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid transparent;
  border-radius: 7px;
  background: transparent;
  color: var(--pixel-text-muted);
  padding: 11px 10px;
  text-decoration: none;
  font-weight: 650;
  transition:
    transform 0.16s ease,
    border-color 0.16s ease,
    background-color 0.16s ease,
    color 0.16s ease;
}

.app-nav__link:hover {
  transform: translateY(-1px);
  border-color: var(--surface-line);
  color: var(--pixel-text);
}

.app-nav__label {
  font-size: 0.95rem;
}

.nav-active {
  border-color: var(--surface-line-strong);
  background: rgba(0, 0, 0, 0.22);
  color: var(--pixel-text);
  box-shadow: none;
}

@media (max-width: 420px) {
  .app-nav {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}
</style>
