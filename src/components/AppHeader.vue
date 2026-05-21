<template>
  <header class="app-header">
    <div class="app-header__brand">
      <span class="app-header__mark">FF</span>
      <div>
        <h1 class="app-header__title">FocusFlow</h1>
        <p class="app-header__subtitle">Calm focus sessions, saved locally.</p>
      </div>
    </div>

    <div class="app-header__badge">Local Pomodoro</div>
  </header>
</template>

<style scoped>
.app-header {
  display: flex;
  height: 72px;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid var(--surface-line);
  background: rgba(25, 24, 15, 0.86);
  padding: 0 28px;
  backdrop-filter: blur(18px);
}

.app-header__brand {
  display: flex;
  align-items: center;
  gap: 14px;
}

.app-header__mark {
  display: inline-flex;
  width: 34px;
  height: 34px;
  align-items: center;
  justify-content: center;
  border: 1px solid var(--surface-line-strong);
  border-radius: 7px;
  color: var(--pixel-text);
  font-size: 0.82rem;
  font-weight: 800;
  letter-spacing: 0;
}

.app-header__title {
  margin: 0;
  color: var(--pixel-text);
  font-size: 1.08rem;
  font-weight: 720;
  line-height: 1;
}

.app-header__subtitle {
  margin: 4px 0 0;
  color: var(--pixel-text-muted);
  font-size: 0.78rem;
  line-height: 1;
}

.app-header__badge {
  border: 1px solid var(--surface-line-strong);
  border-radius: 7px;
  background: rgba(0, 0, 0, 0.16);
  color: var(--pixel-primary);
  padding: 8px 12px;
  font-size: 0.82rem;
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
