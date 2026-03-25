# Repository Guidelines

## Project Structure & Module Organization
FocusFlow is a Tauri desktop app with a Vue 3 + TypeScript frontend.

- `src/`: frontend app code.
- `src/views/`: route-level pages (`TimerView.vue`, `HistoryView.vue`, etc.).
- `src/components/`: reusable UI components (timer, charts, session cards, import/export).
- `src/stores/`: Pinia stores (`timer.ts`, `settings.ts`) that call Tauri commands.
- `src/router/`: Vue Router config.
- `src-tauri/src/`: Rust backend (`commands.rs`, `database.rs`, `timer.rs`, `stats.rs`).
- `docs/`: plans, changelog, and manual test reports.

## Build, Test, and Development Commands
Use `pnpm` only.

- `pnpm install`: install JS dependencies.
- `pnpm run dev`: run Vite web dev server.
- `pnpm run tauri dev`: run full desktop app in development mode.
- `pnpm run build`: type-check frontend (`vue-tsc`) and build web assets.
- `pnpm run tauri build`: build desktop binaries (output under `src-tauri/target/release/`).

## Coding Style & Naming Conventions
- TypeScript/Vue: 2-space indentation, Composition API, and typed store/actions.
- Vue components: `PascalCase.vue` filenames (example: `TimerDisplay.vue`).
- Stores/utilities: lower-case file names; exported symbols use `camelCase` (example: `useTimerStore`).
- Rust: follow idiomatic Rust naming (`snake_case` functions/modules, `CamelCase` types).
- Keep shared data models aligned across `src/types/database.ts` and `src-tauri/src/models.rs`.
- Reuse theme tokens in `src/index.css` and Tailwind config instead of hard-coded colors.

## Testing Guidelines
There is no automated unit/integration test suite configured yet. Validate changes with:

- `pnpm run build` for frontend type/build checks.
- `pnpm run tauri dev` for functional manual verification (timer flow, history, stats, import/export, sound).
- When UI changes are included, add/update notes in `docs/test-results/` with viewport/platform details.

## Commit & Pull Request Guidelines
History follows Conventional Commit prefixes, including: `feat:`, `fix:`, `docs:`, `style:`, `test:`, `config:`.

- Use concise imperative summaries (can be Chinese or English).
- Keep commits focused by concern (UI, backend command, docs).
- PRs should include: change summary, affected modules, manual test steps/results, linked issue (if any), and screenshots/GIFs for UI changes.
