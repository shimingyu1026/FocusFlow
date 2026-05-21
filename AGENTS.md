# FocusFlow - AI Agent 开发指南

> 本文件面向 AI 编程助手。阅读本文档前，默认你对本项目一无所知。

## 项目概览

**FocusFlow** 是一款跨平台的番茄钟桌面应用，主打现代化复古像素风格 UI。当前版本为 **v0.2.0**。

- **应用类型**: Tauri 2.x 桌面应用（Windows / macOS / Linux）
- **界面语言**: 中文
- **核心功能**: 自定义专注时长、任务标签、历史记录、统计图表、8-bit 音效提醒、数据导出/导入、主题切换

### 技术栈

| 层级 | 技术 |
|------|------|
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

---

## 项目结构

```
├── src/                          # Vue 前端源码
│   ├── components/               # 可复用 UI 组件
│   │   ├── TimerDisplay.vue      # 计时器主显示区（SVG 进度环）
│   │   ├── TimerControls.vue     # 开始/暂停/继续/停止按钮
│   │   ├── CompletionAnimation.vue # 完成庆祝动画
│   │   ├── AppHeader.vue         # 页面顶部标题
│   │   ├── AppNav.vue            # 底部导航栏
│   │   ├── SessionCard.vue / SessionList.vue  # 历史记录
│   │   ├── StatsCards.vue / TagDistribution.vue / TrendChart.vue  # 统计图表
│   │   └── ExportButton.vue / ImportButton.vue  # 数据导入导出
│   ├── views/                    # 页面级组件
│   │   ├── TimerView.vue         # 计时器首页
│   │   ├── HistoryView.vue       # 历史记录页
│   │   ├── StatisticsView.vue    # 统计页
│   │   └── SettingsView.vue      # 设置页
│   ├── router/index.ts           # Vue Router 配置（4 个路由）
│   ├── stores/                   # Pinia Store
│   │   ├── timer.ts              # 计时器状态（封装 Tauri invoke）
│   │   └── settings.ts           # 用户设置（音效、主题、默认时长）
│   ├── types/database.ts         # TypeScript 类型定义
│   ├── utils/                    # 工具函数
│   │   ├── stats.ts              # 统计计算辅助
│   │   └── sessionEvents.ts      # 全局自定义事件（sessions-updated）
│   ├── App.vue                   # 根组件（含计时器路由的响应式缩放）
│   ├── main.ts                   # 前端入口
│   └── index.css                 # 全局样式 + 像素风设计系统
├── src-tauri/                    # Tauri / Rust 后端
│   ├── src/                      # Rust 源码
│   │   ├── main.rs               # Tauri 应用入口、命令注册、全局状态
│   │   ├── commands.rs           # #[tauri::command] 前端可调用的命令
│   │   ├── database.rs           # SQLite 数据库操作
│   │   ├── timer.rs              # TimerState 结构体及计时器状态管理
│   │   ├── models.rs             # Rust 数据模型（FocusSession）
│   │   ├── stats.rs              # 统计计算（DailyStats / TagStats）
│   │   └── sound.rs              # 跨平台音效播放
│   ├── resources/sounds/         # 音效文件（complete.mp3 / tick.mp3，可选）
│   ├── icons/                    # 应用图标资源
│   ├── Cargo.toml                # Rust 依赖配置
│   ├── tauri.conf.json           # Tauri 应用配置（窗口 900×700）
│   └── build.rs                  # Tauri 构建脚本
├── docs/                         # 项目文档
│   ├── plans/                    # 实施计划文档
│   ├── changelog/                # 变更日志
│   └── test-results/             # 测试报告
├── package.json                  # Node 依赖与脚本
├── vite.config.ts                # Vite 配置（端口 5173，@ 别名）
├── tsconfig.json                 # TypeScript 配置（严格模式）
├── tailwind.config.js            # Tailwind 自定义颜色/字体
└── postcss.config.js             # PostCSS 配置
```

---

## 常用命令

```bash
# 安装依赖（必须使用 pnpm）
pnpm install

# Web 开发模式（热更新，端口 5173）
pnpm run dev

# 桌面应用开发模式（启动 Tauri 窗口）
pnpm run tauri dev

# 生产构建（先编译前端，再打包桌面应用）
pnpm run tauri build

# 仅前端构建
pnpm run build   # 等价于 vue-tsc && vite build

# TypeScript 类型检查
vue-tsc
```

**构建产物位置**: `src-tauri/target/release/`（桌面可执行文件）

---

## 架构说明

### 前后端通信

前端通过 Tauri 的 `invoke()` 调用 Rust 后端命令：

```typescript
import { invoke } from '@tauri-apps/api/core'
await invoke('start_session', { duration: 25, task: '专注工作', tags: ['工作'] })
```

Rust 命令定义在 `src-tauri/src/commands.rs`，必须在 `src-tauri/src/main.rs` 的 `invoke_handler` 中注册。

### 计时器数据流

1. 用户在 `TimerView.vue` 点击按钮
2. 调用 `useTimerStore().startSession()`（`src/stores/timer.ts`）
3. Store 通过 `invoke('start_session', ...)` 调用 Rust 后端
4. Rust 更新 `TimerState` 全局状态，并在停止时将记录写入 SQLite
5. 前端通过 `setInterval`（每 100ms）本地倒计时，保证进度环动画流畅

### 路由

使用 Hash 路由（`createWebHashHistory`）：

| 路径 | 名称 | 组件 |
|------|------|------|
| `/` | `timer` | `TimerView.vue` |
| `/history` | `history` | `HistoryView.vue` |
| `/statistics` | `statistics` | `StatisticsView.vue` |
| `/settings` | `settings` | `SettingsView.vue` |

### 状态持久化

- **专注记录**: SQLite 数据库，文件位于系统应用数据目录（macOS: `~/Library/Application Support/com.focusflow.app/focusflow.db`）
- **用户设置**: `localStorage` 键名为 `focusflow-settings`

---

## 设计系统（像素风 UI）

### CSS 变量

全局 CSS 变量定义在 `src/index.css`：

```css
--pixel-bg: #1e1b4b;           /* 深靛蓝背景 */
--pixel-primary: #14b8a6;      /* 电光青 */
--pixel-primary-dark: #0f766e; /* 深青色 */
--pixel-secondary: #f97316;    /* 珊瑚色 */
--pixel-text: #f8fafc;         /* 柔白色 */
--pixel-text-muted: #94a3b8;   /*  slate 灰 */
--pixel-success: #39ff14;
--pixel-warning: #facc15;
--pixel-danger: #ff4444;
```

支持主题切换：
- `data-theme-mode`: `dark` | `light`
- `data-theme-accent`: `ocean` | `sunset` | `arcade`

### 通用样式类

- `.pixel-border` — 3px 边框 + 12px 圆角 + 3D 阴影面板
- `.pixel-button` — 3D 立体按钮，带 hover/active 位移动画
- `.font-pixel` — VT323 字体（主像素字体）
- `.font-pixel-old` — Press Start 2P（备用）

### Tailwind 扩展

`tailwind.config.js` 中注册了自定义颜色和字体，可直接在模板中使用：

```html
<div class="bg-pixel-bg text-pixel-primary font-pixel">...</div>
```

---

## 开发规范

### 包管理器

**强制使用 `pnpm`**。`package.json` 和 `tauri.conf.json` 均配置为 pnpm。不要引入 `package-lock.json` 或 `yarn.lock`。

### Vue 组件规范

- 统一使用 **Composition API** + `<script setup lang="ts">`
- Props / Emits 必须显式声明类型
- 组件内样式优先使用 **`<style scoped>`**
- 响应式缩放逻辑集中在 `App.vue`（仅 `timer` 路由启用）

### 路径别名

`@/` 映射到 `./src`，在 `vite.config.ts` 和 `tsconfig.json` 中均已配置：

```typescript
import TimerDisplay from '@/components/TimerDisplay.vue'
```

### 添加新的 Tauri 命令

1. 在 `src-tauri/src/commands.rs` 中创建函数：

```rust
#[tauri::command]
pub async fn my_command(arg: String) -> Result<String, String> {
    Ok(format!("Hello {}", arg))
}
```

2. 在 `src-tauri/src/main.rs` 的 `invoke_handler` 中注册：

```rust
.invoke_handler(tauri::generate_handler![
    // ... 已有命令
    commands::my_command,
])
```

3. 前端调用：

```typescript
await invoke('my_command', { arg: 'world' })
```

### 类型一致性

`src/types/database.ts` 中的 `FocusSession` 接口必须与 `src-tauri/src/models.rs` 中的 `FocusSession` 结构体字段保持一致（注意命名转换：Rust 用 `start_time`，TS 用 `startTime`）。

---

## 测试策略

**本项目目前没有配置自动化测试框架**（无 Jest、Vitest、Playwright、ESLint、Prettier）。

测试方式：
- **手动测试**: 通过 `pnpm run tauri dev` 启动桌面应用进行功能验证
- **类型检查**: 运行 `vue-tsc` 确保无 TypeScript 编译错误
- **文档记录**: 重要迭代会在 `docs/test-results/` 留下验证报告

若你新增复杂功能，建议：
1. 在 `docs/test-results/` 新增测试记录
2. 验证响应式布局（320px ~ 1920px）
3. 验证键盘快捷键（Space / Escape）

---

## 性能与安全注意事项

### 性能

- 动画优先使用 `transform` 和 `opacity`（GPU 加速），避免修改 `width/height/top/left`
- 计时器倒计时 interval 为 **100ms**（而非 1s），以保证 SVG 进度环动画流畅
- `App.vue` 对计时器路由使用 `ResizeObserver` 实现动态缩放，注意避免内存泄漏

### 安全

- `tauri.conf.json` 中 `security.csp` 设为 `null`，未启用内容安全策略
- 数据库和设置均存储在本地，无网络传输或云端同步
- 数据清除操作需要用户输入 `DELETE` 二次确认（`SettingsView.vue`）
- 导入数据时会先清空现有记录再写入（替换式导入）

---

## 音效说明

音效文件（`complete.mp3`、`tick.mp3`）放在 `src-tauri/resources/sounds/`，是**可选的**。若缺失，应用会自动降级为系统默认提示音：

- macOS: `afplay /System/Library/Sounds/Ping.aiff`
- Windows: PowerShell beep
- Linux: `beep` 命令

更多说明见 `SOUNDS.md`。

---

## 文档索引

- `README.md` — 项目简介与功能特性（面向用户）
- `CLAUDE.md` — 面向 Claude Code 的详细开发指南
- `SOUNDS.md` — 音效文件添加说明
- `docs/plans/` — 功能实施计划
- `docs/changelog/` — 版本变更日志
- `docs/test-results/` — 测试验证报告

---

**最后更新**: 基于项目实际内容整理。如有架构调整，请同步更新本文件。
