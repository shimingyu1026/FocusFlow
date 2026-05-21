# FocusFlow - 暖色极简番茄钟

FocusFlow 是一款基于 Tauri 2 和 Vue 3 的跨平台桌面番茄钟应用。界面采用暖黑背景、棕色卡片、克制边线和全局衬线字体：英文与数字优先使用 Georgia / Times New Roman，中文使用 Songti SC / STSong 等宋体 fallback。

![Version](https://img.shields.io/badge/version-v0.2.0-blue)
![Vue](https://img.shields.io/badge/Vue-3.4-42b883)
![Tauri](https://img.shields.io/badge/Tauri-2.x-FFC131)

## 功能

- 自定义专注时长：15 / 25 / 45 / 60 分钟
- 任务名称与标签记录
- SVG 进度环与 100ms 本地倒计时
- 暂停、继续、停止与完成动画
- 历史记录、统计卡片、30 天趋势图、标签分布图
- 数据导出 / 导入
- 主题模式、强调色、音效、默认时长、完成动画样式设置
- Web 预览 fallback：不在 Tauri 环境中也可以试用计时、历史、统计、导入导出
- 本地优先：无网络同步，数据只保存在本机

## 运行

安装依赖：

```bash
pnpm install
```

开发模式：

```bash
# Web 预览，端口 5173
pnpm run dev

# 桌面应用开发模式
pnpm run tauri dev
```

打开当前已构建的 macOS 应用：

```bash
open src-tauri/target/release/bundle/macos/FocusFlow.app
```

## 构建

```bash
pnpm run tauri build
```

默认构建 macOS `.app` bundle：

```text
src-tauri/target/release/bundle/macos/FocusFlow.app
```

说明：当前默认 bundle target 为 `["app"]`，不默认生成 DMG。此前 `targets: "all"` 在本机 DMG bundling 阶段会失败，因此先保证项目级 `pnpm run tauri build` 稳定产出 `.app`。

## 测试

```bash
# 前端类型检查 + 生产构建
pnpm run build

# Rust 单元测试
cargo test --manifest-path src-tauri/Cargo.toml

# Rust 编译检查
cargo check --manifest-path src-tauri/Cargo.toml

# Tauri 环境信息
pnpm tauri info
```

最近的验证记录：

- `docs/test-results/2026-05-21-polish-and-hardening.md`

## 数据位置

FocusFlow 使用 Tauri 的系统应用目录：

- 专注记录：SQLite 数据库 `focusflow.db`，位于 `app_data_dir`
- 用户设置：`settings.json`，位于 `app_config_dir`
- 设置页会显示当前机器上的实际数据库路径和设置文件路径

macOS 上应用 identifier 为：

```text
com.focusflow.desktop
```

Web 预览模式使用浏览器 `localStorage` 作为 fallback：

- `focusflow-sessions`
- `focusflow-settings`

## 快捷键

| 按键 | 功能 |
| --- | --- |
| `Space` | 开始 / 暂停 / 继续 |
| `Escape` | 停止当前计时 |

## 技术栈

| 层级 | 技术 |
| --- | --- |
| 前端 | Vue 3, TypeScript, Pinia, Vue Router |
| 样式 | Tailwind CSS, CSS variables |
| 图表 | Chart.js, vue-chartjs |
| 桌面 | Tauri 2.x |
| 后端 | Rust |
| 数据库 | SQLite, rusqlite bundled |
| 构建 | Vite, pnpm |

## 项目结构

```text
src/                 Vue 前端源码
src/components/      可复用组件
src/views/           页面级组件
src/stores/          Pinia stores
src/utils/           统计、运行时判断、事件工具
src-tauri/src/       Rust 后端命令、数据库、设置、计时器、音效
src-tauri/capabilities/
                     Tauri 权限配置
docs/test-results/   测试和验证记录
```

## 许可证

MIT License
