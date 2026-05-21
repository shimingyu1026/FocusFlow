# FocusFlow - AI Agent 开发指南

> 本文件面向 AI 编程助手。阅读本文档前，默认你对本项目一无所知。

## 项目概览

**FocusFlow** 是一款跨平台番茄钟桌面应用，当前版本为 **v0.2.0**。

- **应用类型**: Tauri 2.x 桌面应用（macOS / Windows / Linux）
- **界面语言**: 中文
- **当前设计方向**: 暖黑背景、棕色卡片、克制边线、全局衬线字体
- **字体策略**: 英文和数字优先使用 `Georgia` / `Times New Roman`；中文 fallback 使用 `Songti SC` / `STSong` / `Noto Serif CJK SC` / `SimSun`
- **核心功能**: 自定义专注时长、任务标签、历史记录、统计图表、音效提醒、数据导出/导入、主题切换

## 技术栈

| 层级 | 技术 |
| --- | --- |
| 前端框架 | Vue 3（Composition API + `<script setup lang="ts">`） |
| 类型系统 | TypeScript 5.6+（严格模式） |
| 样式方案 | Tailwind CSS 3.4 + 自定义 CSS 变量 |
| 状态管理 | Pinia（Composition API 风格 Store） |
| 路由 | Vue Router（Hash 模式） |
| 图表 | Chart.js + vue-chartjs |
| 桌面框架 | Tauri 2.x |
| 后端语言 | Rust |
| 数据库 | SQLite（rusqlite，bundled 特性） |
| 构建工具 | Vite 5 |
| 包管理器 | **pnpm**（强制使用，不可用 npm/yarn） |

## 项目结构

```text
src/
  components/              可复用 UI 组件
    TimerDisplay.vue       计时器主显示区（SVG 进度环）
    TimerControls.vue      开始 / 暂停 / 继续 / 停止按钮
    CompletionAnimation.vue
    AppHeader.vue
    AppNav.vue
    SessionCard.vue / SessionList.vue
    StatsCards.vue / TagDistribution.vue / TrendChart.vue
    ExportButton.vue / ImportButton.vue
  views/
    TimerView.vue
    HistoryView.vue
    StatisticsView.vue
    SettingsView.vue
  router/index.ts
  stores/
    timer.ts               计时器状态，Tauri invoke + Web fallback
    settings.ts            用户设置，Tauri app_config_dir + Web fallback
  types/database.ts
  utils/
    runtime.ts             Tauri runtime 判断
    stats.ts               统计计算
    sessionEvents.ts       sessions-updated 自定义事件
  App.vue                  根组件，桌面计时器路由缩放
  main.ts
  index.css                全局样式、主题变量、字体栈

src-tauri/
  capabilities/default.json
  src/
    main.rs
    commands.rs
    database.rs
    settings.rs
    timer.rs
    models.rs
    stats.rs
    sound.rs
  resources/sounds/
  icons/
  Cargo.toml
  tauri.conf.json
  build.rs

docs/
  plans/
  changelog/
  test-results/
```

## 常用命令

```bash
# 安装依赖
pnpm install

# Web 预览，端口 5173
pnpm run dev

# 桌面应用开发模式
pnpm run tauri dev

# 生产构建，默认生成 macOS .app bundle
pnpm run tauri build

# 前端类型检查 + 构建
pnpm run build

# Rust 单元测试
cargo test --manifest-path src-tauri/Cargo.toml

# Rust 编译检查
cargo check --manifest-path src-tauri/Cargo.toml
```

默认构建产物：

```text
src-tauri/target/release/bundle/macos/FocusFlow.app
```

可直接打开：

```bash
open src-tauri/target/release/bundle/macos/FocusFlow.app
```

## 构建与打包注意事项

- `tauri.conf.json` 的 `bundle.targets` 当前为 `["app"]`。
- 不默认生成 DMG；此前 `targets: "all"` 在本机 DMG bundling 阶段失败。
- macOS bundle identifier 为 `com.focusflow.desktop`。
- `tauri.conf.json` 已启用 CSP。
- 已配置 `src-tauri/capabilities/default.json`，用于 dialog/fs 权限。

## 数据与缓存位置

桌面模式使用 Tauri 系统目录：

- 专注记录：SQLite `focusflow.db`，位于 `app_data_dir`
- 用户设置：`settings.json`，位于 `app_config_dir`
- 设置页会显示实际数据库路径和设置文件路径

Web 预览模式使用 `localStorage` fallback：

- `focusflow-sessions`
- `focusflow-settings`

## 前后端通信

前端通过 Tauri `invoke()` 调用 Rust 命令：

```typescript
import { invoke } from '@tauri-apps/api/core'
await invoke('start_session', { duration: 25, task: '专注工作', tags: ['工作'] })
```

Rust 命令定义在 `src-tauri/src/commands.rs`，必须在 `src-tauri/src/main.rs` 的 `invoke_handler` 中注册。

`src/stores/timer.ts` 和 `src/stores/settings.ts` 都包含 Web fallback；新增命令时注意不要破坏纯 Web 预览能力。

## 计时器数据流

1. 用户在 `TimerView.vue` 操作按钮
2. 调用 `useTimerStore().startSession()` / `pauseSession()` / `resumeSession()` / `stopSession()`
3. 桌面模式通过 `invoke()` 调用 Rust；Web 模式写入 localStorage fallback
4. Rust 更新 `TimerState`，停止时写入 SQLite
5. 前端通过 100ms interval 本地倒计时，保证进度环动画流畅

## 路由

使用 Hash 路由：

| 路径 | 名称 | 组件 |
| --- | --- | --- |
| `/` | `timer` | `TimerView.vue` |
| `/history` | `history` | `HistoryView.vue` |
| `/statistics` | `statistics` | `StatisticsView.vue` |
| `/settings` | `settings` | `SettingsView.vue` |

## 设计系统

全局 CSS 变量在 `src/index.css` 中维护。

当前主色方向：

```css
--pixel-bg: #19180f;
--pixel-panel: #3a3022;
--pixel-panel-solid: #3f3424;
--pixel-primary: #d8d0bf;
--pixel-secondary: #c78f5a;
--pixel-text: #ebe6dc;
--pixel-text-muted: #b9b0a2;
```

字体变量：

```css
--app-font-serif: Georgia, "Times New Roman", "Songti SC", "STSong",
  "Noto Serif CJK SC", "Noto Serif SC", "SimSun", serif;
--app-font-sans: var(--app-font-serif);
```

注意：不要再引入像素字体或外部 Web font；当前要求是全局衬线字体。

## 开发规范

- 必须使用 `pnpm`，不要引入 `package-lock.json` 或 `yarn.lock`。
- Vue 组件统一使用 Composition API + `<script setup lang="ts">`。
- Props / Emits 必须显式声明类型。
- 组件内样式优先使用 `<style scoped>`。
- `@/` 映射到 `./src`。
- `src/types/database.ts` 的 `FocusSession` 必须与 `src-tauri/src/models.rs` 保持一致。
- Rust `FocusSession` 使用 `serde(rename_all = "camelCase")` 对齐前端字段。
- 手动编辑文件优先保持小范围变更。

## 测试策略

当前已有 Rust 单元测试，前端没有 Vitest/Jest/Playwright 测试框架。

常规验证：

```bash
pnpm run build
cargo test --manifest-path src-tauri/Cargo.toml
cargo check --manifest-path src-tauri/Cargo.toml
pnpm tauri info
pnpm run tauri build
```

UI 相关改动建议额外验证：

- 1280 x 720 桌面视口
- 390 x 844 移动视口
- 计时开始 / 暂停 / 继续 / 停止
- 历史记录刷新
- 统计页刷新
- 设置页路径显示

重要验证记录写入：

```text
docs/test-results/
```

最近验证报告：

```text
docs/test-results/2026-05-21-polish-and-hardening.md
```

## 安全与数据操作

- CSP 已启用。
- 数据库和设置均存储在本地，无网络同步。
- 清除所有数据需要输入 `DELETE` 二次确认。
- 导入数据会替换现有记录。
- 音效文件在 `src-tauri/resources/sounds/`，缺失时后端会降级为系统提示音。

## 文档索引

- `README.md`：用户向说明
- `CLAUDE.md`：Claude Code 相关开发说明
- `SOUNDS.md`：音效文件说明
- `docs/plans/`：计划文档
- `docs/changelog/`：变更日志
- `docs/test-results/`：测试验证报告

---

如架构、缓存位置、构建目标或设计系统调整，请同步更新本文档和 `README.md`。
