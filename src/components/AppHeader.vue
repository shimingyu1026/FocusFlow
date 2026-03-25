<template>
  <header class="app-header">
    <div class="app-header__brand">
      <span class="app-header__dot"></span>
      <div>
        <h1 class="app-header__title">FocusFlow</h1>
        <p class="app-header__subtitle">Deep work timer for calm, high-quality focus.</p>
      </div>
    </div>

    <div class="app-header__badge">Desktop Pomodoro</div>
  </header>
</template>

<style scoped>
.app-header {
  display: flex;
  height: 64px;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(15, 23, 42, 0.16);
  padding: 0 20px;
  backdrop-filter: blur(16px);
}

.app-header__brand {
  display: flex;
  align-items: center;
  gap: 14px;
}

.app-header__dot {
  width: 12px;
  height: 12px;
  border-radius: 999px;
  background: linear-gradient(135deg, var(--pixel-primary), var(--pixel-secondary));
  box-shadow: 0 0 18px var(--pixel-glow);
}

.app-header__title {
  margin: 0;
  color: var(--pixel-text);
  font-size: 1.55rem;
  line-height: 1;
}

.app-header__subtitle {
  margin: 4px 0 0;
  color: var(--pixel-text-muted);
  font-size: 0.82rem;
  line-height: 1;
}

.app-header__badge {
  border: 1px solid rgba(20, 184, 166, 0.18);
  border-radius: 999px;
  background: rgba(20, 184, 166, 0.1);
  color: var(--pixel-primary);
  padding: 6px 10px;
  font-size: 0.82rem;
  letter-spacing: 0.04em;
}

@media (max-width: 420px) {
  .app-header {
    height: auto;
    flex-direction: column;
    align-items: flex-start;
    gap: 10px;
    padding: 16px 18px;
  }

  .app-header__badge {
    display: none;
  }
}
</style>
