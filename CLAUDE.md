# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

FocusFlow is a cross-platform Pomodoro timer desktop application with a modern pixel art UI. Built with Tauri 2.x + Vue 3, it uses SQLite for local data storage and features a retro-styled interface with modern responsive design.

**Key Technologies:**
- Frontend: Vue 3 (Composition API), TypeScript, Tailwind CSS, Chart.js
- Backend: Tauri 2.x, Rust, SQLite
- State Management: Pinia
- Build: Vite

## Development Commands

```bash
# Web development (hot reload)
pnpm run dev

# Desktop app development (Tauri)
pnpm run tauri dev

# Build for production
pnpm run tauri build

# Type checking
vue-tsc

# Build artifacts location
# Desktop apps: src-tauri/target/release/
```

**Important:** Always use `pnpm` as the package manager (not npm or yarn).

## Architecture

### Frontend-Backend Communication

This is a Tauri application where the frontend (Vue) communicates with the backend (Rust) through Tauri's `invoke()` API:

```typescript
// Frontend calls backend command
import { invoke } from '@tauri-apps/api/core'
await invoke('start_session', { duration: 25, task: 'Focus' })
```

The Rust backend exposes commands via `#[tauri::command]` in `src-tauri/src/commands.rs`. All commands must be registered in `src-tauri/src/main.rs` under `invoke_handler`.

### Rust Backend Structure

Located in `src-tauri/src/`:

- **main.rs** - Tauri app setup, plugin initialization, command registration
- **commands.rs** - Tauri commands exposed to frontend (start_session, get_sessions, etc.)
- **database.rs** - SQLite database operations
- **timer.rs** - Timer state management (TimerState struct)
- **models.rs** - Data models (FocusSession)
- **stats.rs** - Statistics calculations
- **sound.rs** - Sound playback (completion beep, tick sounds)

**Global State:** The `TimerState` is managed as Tauri global state using `.manage(timer_state)` in main.rs.

### Vue Frontend Structure

**State Management:**
- `src/stores/timer.ts` - Pinia store for timer operations, wraps Rust backend commands
- `src/stores/settings.ts` - User preferences (sound enabled, etc.)

**Routing:**
- Hash-based routing (createWebHashHistory)
- Routes: / (timer), /history, /statistics, /settings
- Defined in `src/router/index.ts`

**Components Architecture:**
- **Views** (src/views/): Page-level components (TimerView, HistoryView, StatisticsView, SettingsView)
- **Components** (src/components/): Reusable UI components
  - TimerDisplay.vue - Main timer with SVG progress ring and responsive scaling
  - TimerControls.vue - Start/pause/resume/stop buttons
  - CompletionAnimation.vue - Celebration overlay with confetti particles
  - SessionCard.vue, SessionList.vue - History display
  - StatsCards.vue, TagDistribution.vue, TrendChart.vue - Statistics visualization

### Data Flow

1. **Timer Flow:**
   - User clicks button in Vue component
   - Component calls Pinia store action (e.g., `useTimerStore().startSession()`)
   - Store invokes Rust backend via `invoke('start_session', ...)`
   - Rust backend updates SQLite database and manages TimerState
   - Frontend reactive state updates via store

2. **TypeScript Types:**
   - Shared types in `src/types/database.ts` (FocusSession interface)
   - Must match Rust models in `src-tauri/src/models.rs`

### Design System (v0.2.0)

**Pixel Art UI System:**

The app uses a custom pixel art design system defined in `src/index.css`:

**CSS Variables (Color System):**
```css
--pixel-bg: #1e1b4b (deep indigo background)
--pixel-primary: #14b8a6 (electric teal)
--pixel-primary-dark: #0d9488 (dark teal)
--pixel-secondary: #f97316 (warm coral)
--pixel-text: #f8fafc (soft white)
--pixel-text-muted: #94a3b8 (slate gray)
```

**Typography:**
- Primary font: VT323 (optimized pixel font, narrower than Press Start 2P)
- Fallback: Press Start 2P (available as `font-pixel-old`)
- Use `font-pixel` Tailwind class for VT323

**UI Components:**
- `.pixel-border` - 3px border with 12px border-radius and 3D shadow
- `.pixel-button` - Interactive buttons with hover/active 3D effects
- Buttons use CSS variables for theming (can override via inline styles)

**Responsive Design:**
- TimerDisplay uses CSS `transform: scale()` for responsive scaling
- Breakpoints in TimerDisplay.vue: max-width 450px (scale 0.75), 350px (scale 0.65)
- Always use `vmin` or percentage-based sizing for new responsive features

**Tailwind Configuration:**
- Custom colors registered as `pixel-bg`, `pixel-primary`, etc. in `tailwind.config.js`
- Custom fonts as `font-pixel` and `font-pixel-old`

### Critical Implementation Details

**TimerDisplay Component:**
- SVG progress ring with circumference calculation: `2 * Math.PI * 144`
- Requires `totalSeconds` prop (optional, defaults to 0) for progress calculation
- Text size: 5.5rem with VT323 font to prevent overflow in 320px container
- All colors use CSS variables for theme consistency

**Completion Animation:**
- Teleport to body for proper z-index stacking
- 50 confetti particles with random positions, colors, delays
- Auto-dismisses after 3 seconds

**Data Import/Export:**
- Uses Tauri's file system plugin (`@tauri-apps/plugin-fs`)
- Export format: JSON array of FocusSession objects
- Import validates and inserts into SQLite database

### Tauri-Specific Patterns

**Adding a New Backend Command:**

1. Create command function in `src-tauri/src/commands.rs`:
```rust
#[tauri::command]
async fn my_new_command(param: String) -> Result<(), String> {
    // implementation
}
```

2. Register in `src-tauri/src/main.rs`:
```rust
.invoke_handler(tauri::generate_handler![
    // ... existing commands
    commands::my_new_command,
])
```

3. Call from frontend:
```typescript
await invoke('my_new_command', { param: 'value' })
```

**Database Access:**
- Database connection initialized in `main.rs` setup
- All database operations in `database.rs`
- Uses `rusqlite` crate with connection pooling

## Important Notes

**No Linting/Testing Configured:** This project does not currently have linting (ESLint) or automated tests. Manual testing is performed via `pnpm run tauri dev`.

**Build Requirements:**
- macOS: Xcode Command Line Tools
- Windows: Visual Studio C++ Build Tools
- Linux: Rust toolchain, webkit2gtk

**Asset Management:**
- Icons and app assets in `src-tauri/icons/`
- Generated icons are large binary files (do not edit manually)

**Performance Considerations:**
- SVG animations use CSS transitions (GPU-accelerated)
- Timer updates every 100ms (not every second) for smoother progress
- Use `transform` and `opacity` for animations (not `width`, `height`, `top`, `left`)
