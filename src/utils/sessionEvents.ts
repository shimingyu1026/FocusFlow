const SESSIONS_UPDATED_EVENT = 'focusflow:sessions-updated'

export function emitSessionsUpdated() {
  window.dispatchEvent(new CustomEvent(SESSIONS_UPDATED_EVENT))
}

export function onSessionsUpdated(handler: () => void) {
  window.addEventListener(SESSIONS_UPDATED_EVENT, handler)
  return () => window.removeEventListener(SESSIONS_UPDATED_EVENT, handler)
}
