# FocusFlow - 复古像素风番茄钟实现计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**目标:** 构建一个跨平台桌面番茄钟应用，具有复古像素风格 UI，支持自定义专注时长、任务标签、历史记录统计和声音提醒。

**架构:** Tauri (Rust) 后端负责系统交互、数据持久化和通知；Vue 3 + TypeScript + Vite 前端负责 UI 渲染和用户交互；SQLite 本地数据库存储所有专注记录。

**技术栈:** Tauri 2.x, Vue 3, TypeScript, Tailwind CSS, SQLite (rusqlite), Chart.js

---

## Task 1: 项目初始化 - Tauri + Vue 3 基础架构

**目标:** 创建 Tauri 项目并配置 Vue 3 + TypeScript 开发环境

**文件:**
- Create: `package.json`
- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/src/main.rs`
- Create: `src/main.ts`
- Create: `src/App.vue`
- Create: `vite.config.ts`
- Create: `tsconfig.json`
- Create: `tailwind.config.js`
- Create: `src/index.css`
- Create: `.gitignore`

### Step 1: 创建 package.json

```json
{
  "name": "focusflow",
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vue-tsc && vite build",
    "tauri": "tauri"
  },
  "dependencies": {
    "vue": "^3.4.0",
    "pinia": "^2.1.7",
    "chart.js": "^4.4.0",
    "vue-chartjs": "^5.3.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.0.0",
    "@tauri-apps/api": "^2.0.0",
    "@vitejs/plugin-vue": "^5.0.0",
    "typescript": "^5.3.0",
    "vite": "^5.0.0",
    "vue-tsc": "^1.8.0",
    "tailwindcss": "^3.4.0",
    "autoprefixer": "^10.4.0",
    "postcss": "^8.4.0"
  }
}
```

### Step 2: 安装依赖

Run: `npm install`
Expected: 所有依赖成功安装

### Step 3: 创建 Tauri 配置文件

创建 `src-tauri/Cargo.toml`:

```toml
[package]
name = "focusflow"
version = "0.1.0"
edition = "2021"

[build-dependencies]
tauri-build = { version = "2.0", features = [] }

[dependencies]
tauri = { version = "2.0", features = ["shell-open"] }
tauri-plugin-shell = "2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rusqlite = { version = "0.32", features = ["bundled"] }
chrono = "0.4"

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]
```

### Step 4: 创建 Rust 主入口

创建 `src-tauri/src/main.rs`:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod database;
mod timer;
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // 初始化数据库
            database::init_db(app.path().app_data_dir())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_session,
            commands::pause_session,
            commands::resume_session,
            commands::stop_session,
            commands::get_sessions,
            commands::get_stats,
            commands::export_data,
            commands::import_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
```

### Step 5: 创建 build.rs

创建 `src-tauri/build.rs`:

```rust
fn main() {
    tauri_build::build()
}
```

### Step 6: 创建 Tauri 配置

创建 `src-tauri/tauri.conf.json`:

```json
{
  "$schema": "https://schema.tauri.app/config/2.0",
  "productName": "FocusFlow",
  "version": "0.1.0",
  "identifier": "com.focusflow.app",
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "FocusFlow",
        "width": 900,
        "height": 700,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

### Step 7: 创建 Vite 配置

创建 `vite.config.ts`:

```typescript
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path'

export default defineConfig({
  plugins: [vue()],
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"]
    }
  },
  envPrefix: ['VITE_', 'TAURI_'],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src')
    }
  }
})
```

### Step 8: 创建 TypeScript 配置

创建 `tsconfig.json`:

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "module": "ESNext",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "preserve",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["src/**/*.ts", "src/**/*.d.ts", "src/**/*.tsx", "src/**/*.vue"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

### Step 9: 创建 tsconfig.node.json

创建 `tsconfig.node.json`:

```json
{
  "compilerOptions": {
    "composite": true,
    "skipLibCheck": true,
    "module": "ESNext",
    "moduleResolution": "bundler",
    "allowSyntheticDefaultImports": true
  },
  "include": ["vite.config.ts"]
}
```

### Step 10: 配置 Tailwind CSS

创建 `tailwind.config.js`:

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'pixel-bg': '#2d1b4e',
        'pixel-green': '#39ff14',
        'pixel-pink': '#ff6ec7',
        'pixel-yellow': '#ffff00',
        'pixel-blue': '#00d9ff',
        'pixel-purple': '#b14eff',
      },
      fontFamily: {
        'pixel': ['"Press Start 2P"', 'monospace'],
      }
    },
  },
  plugins: [],
}
```

### Step 11: 创建 PostCSS 配置

创建 `postcss.config.js`:

```javascript
export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}
```

### Step 12: 创建主 CSS 文件

创建 `src/index.css`:

```css
@import url('https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap');

@tailwind base;
@tailwind components;
@tailwind utilities;

body {
  margin: 0;
  padding: 0;
  font-family: 'Press Start 2P', monospace;
  background-color: #2d1b4e;
  color: #ffffff;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

#app {
  width: 100%;
  height: 100vh;
}

/* Pixel art border utility */
.pixel-border {
  border: 4px solid #39ff14;
  box-shadow:
    4px 0 0 0 #39ff14,
    -4px 0 0 0 #39ff14,
    0 4px 0 0 #39ff14,
    0 -4px 0 0 #39ff14;
}

.pixel-button {
  transition: all 0.1s;
  image-rendering: pixelated;
}

.pixel-button:hover {
  transform: scale(1.05);
  box-shadow: 0 0 10px #39ff14;
}

.pixel-button:active {
  transform: scale(0.95);
}
```

### Step 13: 创建 Vue 入口文件

创建 `src/main.ts`:

```typescript
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import './index.css'

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.mount('#app')
```

### Step 14: 创建根组件

创建 `src/App.vue`:

```vue
<template>
  <div class="w-full h-full bg-pixel-bg text-white">
    <router-view />
  </div>
</template>

<script setup lang="ts">
// App root component
</script>
```

### Step 15: 创建 index.html

创建 `index.html`:

```html
<!DOCTYPE html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>FocusFlow - 番茄时钟</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
```

### Step 16: 创建 .gitignore

创建 `.gitignore`:

```gitignore
node_modules
dist
dist-electron
.DS_Store
*.log
.vscode
.idea
*.db
*.db-shm
*.db-wal
target
src-tauri/target
```

### Step 17: 测试项目启动

Run: `npm run tauri dev`
Expected: Tauri 窗口成功启动，显示紫色背景页面

### Step 18: 提交初始项目

```bash
git add .
git commit -m "feat: initialize Tauri + Vue 3 project with Tailwind CSS"
```

---

## Task 2: 数据库层实现

**目标:** 实现 SQLite 数据库层，用于存储专注会话记录

**文件:**
- Create: `src-tauri/src/database.rs`
- Create: `src-tauri/src/models.rs`
- Create: `src/types/database.ts`

### Step 1: 创建数据库模型

创建 `src-tauri/src/models.rs`:

```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct FocusSession {
    pub id: String,
    pub task: String,
    pub duration: i32,        // 分钟
    pub start_time: String,   // ISO 8601
    pub end_time: String,     // ISO 8601
    pub completed: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsData {
    pub today_total: i32,
    pub today_count: i32,
    pub week_total: i32,
    pub week_avg: i32,
    pub month_total: i32,
    pub month_count: i32,
}
```

### Step 2: 实现数据库模块

创建 `src-tauri/src/database.rs`:

```rust
use rusqlite::{Connection, Result as SqliteResult};
use std::path::{PathBuf, Path};
use crate::models::FocusSession;
use chrono::Utc;

const DB_NAME: &str = "focusflow.db";

pub fn get_db_path(app_data_dir: PathBuf) -> PathBuf {
    app_data_dir.join(DB_NAME)
}

pub fn init_db(app_data_dir: PathBuf) -> SqliteResult<()> {
    let db_path = get_db_path(app_data_dir);

    // 确保目录存在
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            task TEXT NOT NULL,
            duration INTEGER NOT NULL,
            start_time TEXT NOT NULL,
            end_time TEXT NOT NULL,
            completed BOOLEAN NOT NULL,
            tags TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

pub fn save_session(session: &FocusSession, db_path: &Path) -> SqliteResult<()> {
    let conn = Connection::open(db_path)?;

    let tags_json = serde_json::to_string(&session.tags).unwrap();

    conn.execute(
        "INSERT INTO sessions (id, task, duration, start_time, end_time, completed, tags)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        [
            &session.id,
            &session.task,
            &session.duration.to_string(),
            &session.start_time,
            &session.end_time,
            &session.completed.to_string(),
            &tags_json,
        ],
    )?;

    Ok(())
}

pub fn get_sessions(db_path: &Path, limit: Option<i32>) -> SqliteResult<Vec<FocusSession>> {
    let conn = Connection::open(db_path)?;

    let query = if let Some(limit) = limit {
        format!("SELECT * FROM sessions ORDER BY start_time DESC LIMIT {}", limit)
    } else {
        "SELECT * FROM sessions ORDER BY start_time DESC".to_string()
    };

    let mut stmt = conn.prepare(&query)?;
    let rows = stmt.query_map([], |row| {
        let tags_str: String = row.get(6)?;
        let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();

        Ok(FocusSession {
            id: row.get(0)?,
            task: row.get(1)?,
            duration: row.get(2)?,
            start_time: row.get(3)?,
            end_time: row.get(4)?,
            completed: row.get(5)?,
            tags,
        })
    })?;

    let mut sessions = Vec::new();
    for row in rows {
        sessions.push(row?);
    }

    Ok(sessions)
}

pub fn delete_session(db_path: &Path, id: &str) -> SqliteResult<()> {
    let conn = Connection::open(db_path)?;
    conn.execute("DELETE FROM sessions WHERE id = ?1", [id])?;
    Ok(())
}

pub fn export_data(db_path: &Path) -> SqliteResult<String> {
    let sessions = get_sessions(db_path, None)?;
    let json = serde_json::to_string(&sessions).map_err(|e| {
        rusqlite::Error::ToSqlConversionFailure(Box::new(e))
    })?;
    Ok(json)
}

pub fn import_data(db_path: &Path, json_data: &str) -> SqliteResult<usize> {
    let sessions: Vec<FocusSession> = serde_json::from_str(json_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    let conn = Connection::open(db_path)?;
    let mut count = 0;

    for session in sessions {
        let tags_json = serde_json::to_string(&session.tags).unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO sessions (id, task, duration, start_time, end_time, completed, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            [
                &session.id,
                &session.task,
                &session.duration.to_string(),
                &session.start_time,
                &session.end_time,
                &session.completed.to_string(),
                &tags_json,
            ],
        )?;
        count += 1;
    }

    Ok(count)
}
```

### Step 3: 更新 main.rs 以使用数据库

修改 `src-tauri/src/main.rs`，在 setup 中添加:

```rust
.setup(|app| {
    let app_data_dir = app.path().app_data_dir()
        .expect("Failed to get app data dir");
    database::init_db(app_data_dir)?;
    Ok(())
})
```

### Step 4: 创建 TypeScript 类型定义

创建 `src/types/database.ts`:

```typescript
export interface FocusSession {
  id: string
  task: string
  duration: number
  startTime: string
  endTime: string
  completed: boolean
  tags: string[]
}
```

### Step 5: 测试数据库初始化

Run: `npm run tauri dev`
Expected: 应用启动，数据库文件在 `~/Library/Application Support/com.focusflow.app/focusflow.db` (macOS)

### Step 6: 验证数据库表创建

手动检查数据库:
```bash
sqlite3 ~/Library/Application\ Support/com.focusflow.app/focusflow.db ".schema"
```
Expected: 显示 sessions 表结构

### Step 7: 提交数据库层

```bash
git add src-tauri/src/ src/types/
git commit -m "feat: implement SQLite database layer for focus sessions"
```

---

## Task 3: Tauri Commands 实现

**目标:** 实现 Tauri 命令，连接前端和后端

**文件:**
- Create: `src-tauri/src/commands.rs`
- Create: `src-tauri/src/timer.rs`
- Create: `src/stores/timer.ts`

### Step 1: 实现计时器逻辑

创建 `src-tauri/src/timer.rs`:

```rust
use std::sync::Mutex;
use tauri::State;
use chrono::Utc;

pub struct TimerState {
    pub is_running: Mutex<bool>,
    pub remaining_seconds: Mutex<i32>,
    pub current_task: Mutex<Option<String>>,
    pub start_time: Mutex<Option<String>>,
}

impl TimerState {
    pub fn new() -> Self {
        Self {
            is_running: Mutex::new(false),
            remaining_seconds: Mutex::new(0),
            current_task: Mutex::new(None),
            start_time: Mutex::new(None),
        }
    }
}

pub fn start_timer(state: &TimerState, duration_minutes: i32, task: String) {
    let mut is_running = state.is_running.lock().unwrap();
    let mut remaining = state.remaining_seconds.lock().unwrap();
    let mut current_task = state.current_task.lock().unwrap();
    let mut start_time = state.start_time.lock().unwrap();

    *is_running = true;
    *remaining = duration_minutes * 60;
    *current_task = Some(task);
    *start_time = Some(Utc::now().to_rfc3339());
}

pub fn pause_timer(state: &TimerState) {
    let mut is_running = state.is_running.lock().unwrap();
    *is_running = false;
}

pub fn resume_timer(state: &TimerState) {
    let mut is_running = state.is_running.lock().unwrap();
    *is_running = true;
}

pub fn stop_timer(state: &TimerState) {
    let mut is_running = state.is_running.lock().unwrap();
    let mut remaining = state.remaining_seconds.lock().unwrap();
    let mut current_task = state.current_task.lock().unwrap();
    let mut start_time = state.start_time.lock().unwrap();

    *is_running = false;
    *remaining = 0;
    *current_task = None;
    *start_time = None;
}
```

### Step 2: 实现 Tauri 命令

创建 `src-tauri/src/commands.rs`:

```rust
use crate::timer::{TimerState, start_timer, pause_timer, resume_timer, stop_timer};
use crate::models::{FocusSession, StatsData};
use crate::database::{self, get_db_path};
use tauri::{State, Manager};
use chrono::Utc;
use uuid::Uuid;

#[tauri::command]
pub async fn start_session(
    state: State<'_, TimerState>,
    duration: i32,
    task: String,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    start_timer(&state, duration, task);
    Ok(())
}

#[tauri::command]
pub async fn pause_session(state: State<'_, TimerState>) -> Result<(), String> {
    pause_timer(&state);
    Ok(())
}

#[tauri::command]
pub async fn resume_session(state: State<'_, TimerState>) -> Result<(), String> {
    resume_timer(&state);
    Ok(())
}

#[tauri::command]
pub async fn stop_session(
    state: State<'_, TimerState>,
    completed: bool,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let task = state.current_task.lock().unwrap().clone().unwrap_or_default();
    let start_time = state.start_time.lock().unwrap().clone().unwrap_or_default();
    let remaining = *state.remaining_seconds.lock().unwrap();

    let session = FocusSession {
        id: Uuid::new_v4().to_string(),
        task,
        duration: (remaining / 60).max(1),
        start_time,
        end_time: Utc::now().to_rfc3339(),
        completed,
        tags: vec![],
    };

    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = database::get_db_path(app_data_dir);

    database::save_session(&session, &db_path)
        .map_err(|e| e.to_string())?;

    stop_timer(&state);
    Ok(())
}

#[tauri::command]
pub async fn get_sessions(
    limit: Option<i32>,
    app_handle: tauri::AppHandle,
) -> Result<Vec<FocusSession>, String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = database::get_db_path(app_data_dir);

    database::get_sessions(&db_path, limit)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_session(
    id: String,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = database::get_db_path(app_data_dir);

    database::delete_session(&db_path, &id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn export_data(
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = database::get_db_path(app_data_dir);

    database::export_data(&db_path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_data(
    json_data: String,
    app_handle: tauri::AppHandle,
) -> Result<usize, String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = database::get_db_path(app_data_dir);

    database::import_data(&db_path, &json_data)
        .map_err(|e| e.to_string())
}
```

### Step 3: 更新 Cargo.toml 添加依赖

修改 `src-tauri/Cargo.toml` 添加:

```toml
uuid = { version = "1.6", features = ["v4"] }
```

### Step 4: 更新 main.rs 注册 TimerState

修改 `src-tauri/src/main.rs`:

```rust
use tauri::Manager;
use crate::timer::TimerState;

mod database;
mod timer;
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let timer_state = TimerState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(timer_state)
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()
                .expect("Failed to get app data dir");
            database::init_db(app_data_dir)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_session,
            commands::pause_session,
            commands::resume_session,
            commands::stop_session,
            commands::get_sessions,
            commands::delete_session,
            commands::export_data,
            commands::import_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Step 5: 创建 Pinia store

创建 `src/stores/timer.ts`:

```typescript
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { FocusSession } from '@/types/database'

export const useTimerStore = defineStore('timer', () => {
  const isRunning = ref(false)
  const remainingSeconds = ref(0)
  const currentTask = ref('')

  async function startSession(duration: number, task: string) {
    await invoke('start_session', { duration, task })
    isRunning.value = true
    remainingSeconds.value = duration * 60
    currentTask.value = task
  }

  async function pauseSession() {
    await invoke('pause_session')
    isRunning.value = false
  }

  async function resumeSession() {
    await invoke('resume_session')
    isRunning.value = true
  }

  async function stopSession(completed: boolean) {
    await invoke('stop_session', { completed })
    isRunning.value = false
    remainingSeconds.value = 0
    currentTask.value = ''
  }

  async function loadSessions(limit?: number): Promise<FocusSession[]> {
    return await invoke('get_sessions', { limit })
  }

  async function deleteSession(id: string) {
    await invoke('delete_session', { id })
  }

  return {
    isRunning,
    remainingSeconds,
    currentTask,
    startSession,
    pauseSession,
    resumeSession,
    stopSession,
    loadSessions,
    deleteSession,
  }
})
```

### Step 6: 测试命令

Run: `npm run tauri dev`
Expected: 应用成功编译，无报错

### Step 7: 提交命令实现

```bash
git add src-tauri/ src/stores/
git commit -m "feat: implement Tauri commands and timer state management"
```

---

---

## Task 4: Vue Router 设置和页面结构

**目标:** 设置 Vue Router 并创建应用页面结构

**文件:**
- Create: `src/router/index.ts`
- Create: `src/views/TimerView.vue`
- Create: `src/views/HistoryView.vue`
- Create: `src/views/StatisticsView.vue`
- Create: `src/views/SettingsView.vue`
- Create: `src/components/AppHeader.vue`
- Create: `src/components/AppNav.vue`

### Step 1: 安装 Vue Router

Run: `npm install vue-router@4`
Expected: Router 依赖安装成功

### Step 2: 创建路由配置

创建 `src/router/index.ts`:

```typescript
import { createRouter, createWebHashHistory, type RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'timer',
    component: () => import('@/views/TimerView.vue'),
  },
  {
    path: '/history',
    name: 'history',
    component: () => import('@/views/HistoryView.vue'),
  },
  {
    path: '/statistics',
    name: 'statistics',
    component: () => import('@/views/StatisticsView.vue'),
  },
  {
    path: '/settings',
    name: 'settings',
    component: () => import('@/views/SettingsView.vue'),
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router
```

### Step 3: 更新 main.ts 使用路由

修改 `src/main.ts`:

```typescript
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import './index.css'

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.use(router)
app.mount('#app')
```

### Step 4: 更新 App.vue

修改 `src/App.vue`:

```vue
<template>
  <div class="w-full h-full bg-pixel-bg text-white flex flex-col">
    <AppHeader />
    <main class="flex-1 overflow-hidden">
      <router-view />
    </main>
    <AppNav />
  </div>
</template>

<script setup lang="ts">
import AppHeader from '@/components/AppHeader.vue'
import AppNav from '@/components/AppNav.vue'
</script>
```

### Step 5: 创建导航组件

创建 `src/components/AppHeader.vue`:

```vue
<template>
  <header class="h-16 pixel-border border-b-4 border-pixel-green flex items-center justify-center bg-pixel-bg">
    <h1 class="text-2xl text-pixel-green font-pixel">FOCUS FLOW</h1>
  </header>
</template>

<script setup lang="ts">
// Header component
</script>
```

创建 `src/components/AppNav.vue`:

```vue
<template>
  <nav class="h-20 pixel-border border-t-4 border-pixel-green flex items-center justify-around bg-pixel-bg">
    <router-link
      to="/"
      class="pixel-button flex flex-col items-center gap-2 p-4"
      :class="{ 'text-pixel-green': $route.name === 'timer' }"
    >
      <span class="text-2xl">⏱️</span>
      <span class="text-xs font-pixel">计时器</span>
    </router-link>

    <router-link
      to="/history"
      class="pixel-button flex flex-col items-center gap-2 p-4"
      :class="{ 'text-pixel-green': $route.name === 'history' }"
    >
      <span class="text-2xl">📝</span>
      <span class="text-xs font-pixel">历史</span>
    </router-link>

    <router-link
      to="/statistics"
      class="pixel-button flex flex-col items-center gap-2 p-4"
      :class="{ 'text-pixel-green': $route.name === 'statistics' }"
    >
      <span class="text-2xl">📊</span>
      <span class="text-xs font-pixel">统计</span>
    </router-link>

    <router-link
      to="/settings"
      class="pixel-button flex flex-col items-center gap-2 p-4"
      :class="{ 'text-pixel-green': $route.name === 'settings' }"
    >
      <span class="text-2xl">⚙️</span>
      <span class="text-xs font-pixel">设置</span>
    </router-link>
  </nav>
</template>

<script setup lang="ts">
// Navigation component
</script>
```

### Step 6: 创建页面占位符

创建 `src/views/TimerView.vue`:

```vue
<template>
  <div class="h-full flex items-center justify-center">
    <p class="text-pixel-green font-pixel">计时器页面</p>
  </div>
</template>

<script setup lang="ts">
// Timer view
</script>
```

创建 `src/views/HistoryView.vue`:

```vue
<template>
  <div class="h-full flex items-center justify-center">
    <p class="text-pixel-green font-pixel">历史记录页面</p>
  </div>
</template>

<script setup lang="ts">
// History view
</script>
```

创建 `src/views/StatisticsView.vue`:

```vue
<template>
  <div class="h-full flex items-center justify-center">
    <p class="text-pixel-green font-pixel">统计页面</p>
  </div>
</template>

<script setup lang="ts">
// Statistics view
</script>
```

创建 `src/views/SettingsView.vue`:

```vue
<template>
  <div class="h-full flex items-center justify-center">
    <p class="text-pixel-green font-pixel">设置页面</p>
  </div>
</template>

<script setup lang="ts">
// Settings view
</script>
```

### Step 7: 测试路由

Run: `npm run tauri dev`
Expected: 应用启动，底部显示导航栏，可点击切换页面

### Step 8: 提交路由实现

```bash
git add src/
git commit -m "feat: add Vue Router and page structure"
```

---

## Task 5: 计时器 UI 组件

**目标:** 实现复古像素风格的计时器界面

**文件:**
- Create: `src/components/TimerDisplay.vue`
- Create: `src/components/TimerControls.vue`
- Create: `src/components/TaskInput.vue`
- Modify: `src/views/TimerView.vue`

### Step 1: 创建计时器显示组件

创建 `src/components/TimerDisplay.vue`:

```vue
<template>
  <div class="flex flex-col items-center gap-8">
    <!-- 任务描述 -->
    <div class="pixel-border p-4 w-full max-w-md bg-pixel-bg">
      <input
        v-model="taskInput"
        type="text"
        placeholder="输入当前专注任务..."
        class="w-full bg-transparent text-white font-pixel text-sm outline-none placeholder-gray-500"
        :disabled="isRunning"
      />
    </div>

    <!-- 圆形计时器 -->
    <div class="relative w-72 h-72">
      <!-- 外圈像素边框 -->
      <div class="absolute inset-0 rounded-full pixel-border border-8 border-pixel-green"></div>

      <!-- 时间显示 -->
      <div class="absolute inset-0 flex flex-col items-center justify-center">
        <p class="text-6xl font-pixel text-pixel-green">{{ formattedTime }}</p>
        <p v-if="isRunning" class="text-sm font-pixel text-pixel-pink mt-4">专注中...</p>
        <p v-else class="text-sm font-pixel text-gray-400 mt-4">准备开始</p>
      </div>

      <!-- 像素装饰星星 -->
      <div class="absolute -top-4 -left-4 text-2xl">⭐</div>
      <div class="absolute -top-4 -right-4 text-2xl">⭐</div>
      <div class="absolute -bottom-4 -left-4 text-2xl">⭐</div>
      <div class="absolute -bottom-4 -right-4 text-2xl">⭐</div>
    </div>

    <!-- 时长选择 -->
    <div v-if="!isRunning && remainingSeconds === 0" class="flex gap-4">
      <button
        v-for="duration in [15, 25, 45, 60]"
        :key="duration"
        @click="selectDuration(duration)"
        class="pixel-button px-4 py-2 pixel-border border-pixel-blue text-sm font-pixel"
        :class="{ 'bg-pixel-blue text-black': selectedDuration === duration }"
      >
        {{ duration }}分钟
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'

const props = defineProps<{
  isRunning: boolean
  remainingSeconds: number
}>()

const emit = defineEmits<{
  'update:task': [task: string]
  'select-duration': [duration: number]
}>()

const taskInput = ref('')
const selectedDuration = ref(25)

const formattedTime = computed(() => {
  const minutes = Math.floor(props.remainingSeconds / 60)
  const seconds = props.remainingSeconds % 60
  return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`
})

watch(taskInput, (newTask) => {
  emit('update:task', newTask)
})

function selectDuration(duration: number) {
  selectedDuration.value = duration
  emit('select-duration', duration)
}
</script>
```

### Step 2: 创建控制按钮组件

创建 `src/components/TimerControls.vue`:

```vue
<template>
  <div class="flex gap-6 items-center justify-center">
    <!-- 开始按钮 -->
    <button
      v-if="!isRunning && remainingSeconds === 0"
      @click="handleStart"
      class="pixel-button pixel-border border-pixel-green bg-pixel-green text-black px-8 py-4 font-pixel text-lg hover:shadow-[0_0_20px_#39ff14]"
    >
      ▶ 开始
    </button>

    <!-- 暂停按钮 -->
    <button
      v-if="isRunning"
      @click="handlePause"
      class="pixel-button pixel-border border-pixel-yellow bg-pixel-yellow text-black px-8 py-4 font-pixel text-lg hover:shadow-[0_0_20px_#ffff00]"
    >
      ⏸ 暂停
    </button>

    <!-- 继续按钮 -->
    <button
      v-if="!isRunning && remainingSeconds > 0"
      @click="handleResume"
      class="pixel-button pixel-border border-pixel-green bg-pixel-green text-black px-8 py-4 font-pixel text-lg hover:shadow-[0_0_20px_#39ff14]"
    >
      ▶ 继续
    </button>

    <!-- 停止按钮 -->
    <button
      v-if="remainingSeconds > 0"
      @click="handleStop"
      class="pixel-button pixel-border border-pixel-pink bg-pixel-pink text-black px-8 py-4 font-pixel text-lg hover:shadow-[0_0_20px_#ff6ec7]"
    >
      ⏹ 停止
    </button>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  isRunning: boolean
  remainingSeconds: number
}>()

const emit = defineEmits<{
  start: []
  pause: []
  resume: []
  stop: [completed: boolean]
}>()

function handleStart() {
  emit('start')
}

function handlePause() {
  emit('pause')
}

function handleResume() {
  emit('resume')
}

function handleStop() {
  const confirmed = confirm('确定要停止当前专注吗？')
  if (confirmed) {
    emit('stop', false)
  }
}
</script>
```

### Step 3: 实现计时器视图

修改 `src/views/TimerView.vue`:

```vue
<template>
  <div class="h-full flex flex-col items-center justify-center gap-12 p-8">
    <TimerDisplay
      :is-running="isRunning"
      :remaining-seconds="remainingSeconds"
      @update:task="handleTaskUpdate"
      @select-duration="handleDurationSelect"
    />

    <TimerControls
      :is-running="isRunning"
      :remaining-seconds="remainingSeconds"
      @start="handleStart"
      @pause="handlePause"
      @resume="handleResume"
      @stop="handleStop"
    />

    <!-- 专注提示 -->
    <div v-if="isRunning" class="pixel-border p-4 bg-pixel-bg max-w-md text-center">
      <p class="text-sm font-pixel text-pixel-green">💪 保持专注，你可以的！</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useTimerStore } from '@/stores/timer'
import TimerDisplay from '@/components/TimerDisplay.vue'
import TimerControls from '@/components/TimerControls.vue'

const timerStore = useTimerStore()
const isRunning = ref(false)
const remainingSeconds = ref(0)
const selectedDuration = ref(25)
let timerInterval: number | null = null

function handleTaskUpdate(task: string) {
  // 保存任务描述
}

function handleDurationSelect(duration: number) {
  selectedDuration.value = duration
}

async function handleStart() {
  await timerStore.startSession(selectedDuration.value, '')
  isRunning.value = true
  remainingSeconds.value = selectedDuration.value * 60
  startTimer()
}

async function handlePause() {
  await timerStore.pauseSession()
  isRunning.value = false
  if (timerInterval) {
    clearInterval(timerInterval)
    timerInterval = null
  }
}

async function handleResume() {
  await timerStore.resumeSession()
  isRunning.value = true
  startTimer()
}

async function handleStop(completed: boolean) {
  await timerStore.stopSession(completed)
  isRunning.value = false
  remainingSeconds.value = 0
  if (timerInterval) {
    clearInterval(timerInterval)
    timerInterval = null
  }

  if (completed) {
    alert('🎉 恭喜！完成了一次专注！')
  }
}

function startTimer() {
  timerInterval = setInterval(() => {
    if (remainingSeconds.value > 0) {
      remainingSeconds.value--
    } else {
      // 计时结束
      handleStop(true)
    }
  }, 1000)
 as any
}

onUnmounted(() => {
  if (timerInterval) {
    clearInterval(timerInterval)
  }
})
</script>
```

### Step 4: 测试计时器功能

Run: `npm run tauri dev`
Expected:
- 显示像素风格计时器
- 点击"开始"按钮启动计时
- 时间每秒递减
- 可以暂停/继续/停止

### Step 5: 提交计时器 UI

```bash
git add src/
git commit -m "feat: implement pixel-style timer UI with controls"
```

---

## Task 6: 历史记录页面

**目标:** 实现历史记录展示和管理

**文件:**
- Create: `src/components/SessionList.vue`
- Create: `src/components/SessionCard.vue`
- Modify: `src/views/HistoryView.vue`

### Step 1: 创建会话卡片组件

创建 `src/components/SessionCard.vue`:

```vue
<template>
  <div class="pixel-border p-4 bg-pixel-bg mb-4">
    <div class="flex justify-between items-start gap-4">
      <div class="flex-1">
        <h3 class="font-pixel text-pixel-green text-sm mb-2">{{ session.task || '未命名任务' }}</h3>
        <div class="flex gap-4 text-xs font-pixel text-gray-400">
          <span>{{ formatDate(session.startTime) }}</span>
          <span>{{ session.duration }}分钟</span>
          <span v-if="session.completed" class="text-pixel-green">✓ 完成</span>
          <span v-else class="text-pixel-pink">✗ 中断</span>
        </div>
        <div v-if="session.tags.length > 0" class="flex gap-2 mt-2">
          <span
            v-for="tag in session.tags"
            :key="tag"
            class="text-xs px-2 py-1 pixel-border border-pixel-purple text-pixel-purple"
          >
            {{ tag }}
          </span>
        </div>
      </div>
      <button
        @click="handleDelete"
        class="pixel-button text-pixel-pink hover:text-red-500 text-lg"
      >
        🗑️
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { FocusSession } from '@/types/database'

const props = defineProps<{
  session: FocusSession
}>()

const emit = defineEmits<{
  delete: [id: string]
}>()

function formatDate(dateString: string): string {
  const date = new Date(dateString)
  const month = (date.getMonth() + 1).toString().padStart(2, '0')
  const day = date.getDate().toString().padStart(2, '0')
  const hours = date.getHours().toString().padStart(2, '0')
  const minutes = date.getMinutes().toString().padStart(2, '0')
  return `${month}-${day} ${hours}:${minutes}`
}

function handleDelete() {
  if (confirm('确定删除这条记录吗？')) {
    emit('delete', props.session.id)
  }
}
</script>
```

### Step 2: 创建会话列表组件

创建 `src/components/SessionList.vue`:

```vue
<template>
  <div class="h-full flex flex-col">
    <!-- 筛选栏 -->
    <div class="pixel-border p-4 mb-4 bg-pixel-bg">
      <div class="flex gap-4 items-center">
        <span class="font-pixel text-sm text-pixel-green">筛选:</span>
        <button
          @click="filterTag = ''"
          class="pixel-button px-3 py-1 text-xs font-pixel"
          :class="filterTag === '' ? 'bg-pixel-green text-black' : 'pixel-border border-pixel-green'"
        >
          全部
        </button>
        <button
          v-for="tag in allTags"
          :key="tag"
          @click="filterTag = tag"
          class="pixel-button px-3 py-1 text-xs font-pixel"
          :class="filterTag === tag ? 'bg-pixel-green text-black' : 'pixel-border border-pixel-green'"
        >
          {{ tag }}
        </button>
      </div>
    </div>

    <!-- 会话列表 -->
    <div class="flex-1 overflow-y-auto px-4 pb-4">
      <div v-if="filteredSessions.length === 0" class="text-center text-gray-500 font-pixel text-sm mt-12">
        暂无记录
      </div>
      <SessionCard
        v-for="session in filteredSessions"
        :key="session.id"
        :session="session"
        @delete="handleDelete"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import SessionCard from './SessionCard.vue'
import type { FocusSession } from '@/types/database'

const props = defineProps<{
  sessions: FocusSession[]
}>()

const emit = defineEmits<{
  delete: [id: string]
}>()

const filterTag = ref('')

const allTags = computed(() => {
  const tags = new Set<string>()
  props.sessions.forEach(session => {
    session.tags.forEach(tag => tags.add(tag))
  })
  return Array.from(tags)
})

const filteredSessions = computed(() => {
  if (!filterTag.value) {
    return props.sessions
  }
  return props.sessions.filter(session =>
    session.tags.includes(filterTag.value)
  )
})

function handleDelete(id: string) {
  emit('delete', id)
}
</script>
```

### Step 3: 实现历史记录视图

修改 `src/views/HistoryView.vue`:

```vue
<template>
  <div class="h-full p-4">
    <SessionList
      :sessions="sessions"
      @delete="handleDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useTimerStore } from '@/stores/timer'
import SessionList from '@/components/SessionList.vue'
import type { FocusSession } from '@/types/database'

const timerStore = useTimerStore()
const sessions = ref<FocusSession[]>([])

async function loadSessions() {
  sessions.value = await timerStore.loadSessions()
}

async function handleDelete(id: string) {
  await timerStore.deleteSession(id)
  await loadSessions()
}

onMounted(() => {
  loadSessions()
})
</script>
```

### Step 4: 测试历史记录

Run: `npm run tauri dev`
Expected: 显示历史记录列表，可以筛选和删除

### Step 5: 提交历史记录页面

```bash
git add src/
git commit -m "feat: implement history page with session management"
```

---

继续编写剩余任务（统计图表、声音提醒等）吗？## Task 7: 统计图表页面

**目标:** 实现统计数据展示和图表可视化

**文件:**
- Create: `src/components/StatsCards.vue`
- Create: `src/components/TrendChart.vue`
- Create: `src/components/TagDistribution.vue`
- Create: `src/utils/stats.ts`
- Modify: `src/views/StatisticsView.vue`
- Create: `src-tauri/src/stats.rs`

### Step 1: 实现统计计算模块

创建 `src-tauri/src/stats.rs`:

```rust
use crate::models::FocusSession;
use chrono::{Utc, Duration};

pub struct DailyStats {
    pub date: String,
    pub total_minutes: i32,
    pub count: i32,
}

pub struct TagStats {
    pub tag: String,
    pub total_minutes: i32,
    pub percentage: f64,
}

pub fn calculate_daily_stats(sessions: &[FocusSession], days: i32) -> Vec<DailyStats> {
    let mut stats: std::collections::HashMap<String, (i32, i32)> = std::collections::HashMap::new();
    let now = Utc::now();

    for i in 0..days {
        let date = now - Duration::days(i as i64);
        let date_key = date.format("%Y-%m-%d").to_string();
        stats.insert(date_key, (0, 0));
    }

    for session in sessions {
        if let Ok(start_time) = session.start_time.parse::<chrono::DateTime<Utc>>() {
            let date_key = start_time.format("%Y-%m-%d").to_string();
            if let Some((total, count)) = stats.get_mut(&date_key) {
                *total += session.duration;
                *count += 1;
            }
        }
    }

    let mut result: Vec<DailyStats> = stats.into_iter()
        .map(|(date, (total, count))| DailyStats {
            date,
            total_minutes: total,
            count,
        })
        .collect();

    result.sort_by(|a, b| a.date.cmp(&b.date));
    result
}

pub fn calculate_tag_stats(sessions: &[FocusSession]) -> Vec<TagStats> {
    let mut tag_totals: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    let mut total_all: i32 = 0;

    for session in sessions {
        for tag in &session.tags {
            *tag_totals.entry(tag.clone()).or_insert(0) += session.duration;
            total_all += session.duration;
        }
    }

    tag_totals.into_iter()
        .map(|(tag, total)| TagStats {
            tag,
            total_minutes: total,
            percentage: if total_all > 0 {
                (total as f64 / total_all as f64) * 100.0
            } else {
                0.0
            },
        })
        .collect()
}
```

### Step 2: 添加统计命令

修改 `src-tauri/src/commands.rs` 添加:

```rust
use crate::stats::{self, DailyStats, TagStats};

#[tauri::command]
pub async fn get_stats(
    app_handle: tauri::AppHandle,
) -> Result<Vec<DailyStats>, String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = database::get_db_path(app_data_dir);

    let sessions = database::get_sessions(&db_path, None)
        .map_err(|e| e.to_string())?;

    Ok(stats::calculate_daily_stats(&sessions, 30))
}

#[tauri::command]
pub async fn get_tag_stats(
    app_handle: tauri::AppHandle,
) -> Result<Vec<TagStats>, String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = database::get_db_path(app_data_dir);

    let sessions = database::get_sessions(&db_path, None)
        .map_err(|e| e.to_string())?;

    Ok(stats::calculate_tag_stats(&sessions))
}
```

### Step 3: 更新 main.rs 注册新命令

修改 `src-tauri/src/main.rs`，添加 stats 模块和命令:

```rust
mod stats;

// 在 invoke_handler 中添加:
commands::get_stats,
commands::get_tag_stats,
```

### Step 4: 创建前端统计工具函数

创建 `src/utils/stats.ts`:

```typescript
import type { FocusSession } from '@/types/database'

export interface DailyStats {
  date: string
  total_minutes: number
  count: number
}

export interface TagStats {
  tag: string
  total_minutes: number
  percentage: number
}

export function formatMinutes(minutes: number): string {
  const hours = Math.floor(minutes / 60)
  const mins = minutes % 60
  if (hours > 0) {
    return `${hours}小时${mins}分钟`
  }
  return `${mins}分钟`
}

export function calculateTagStats(sessions: FocusSession[]): TagStats[] {
  const tagTotals: Record<string, number> = {}
  let totalAll = 0

  sessions.forEach(session => {
    session.tags.forEach(tag => {
      tagTotals[tag] = (tagTotals[tag] || 0) + session.duration
      totalAll += session.duration
    })
  })

  return Object.entries(tagTotals).map(([tag, total_minutes]) => ({
    tag,
    total_minutes,
    percentage: totalAll > 0 ? (total_minutes / totalAll) * 100 : 0
  }))
}
```

### Step 5: 创建统计卡片组件

创建 `src/components/StatsCards.vue`:

```vue
<template>
  <div class="grid grid-cols-2 gap-4 mb-6">
    <div class="pixel-border p-4 bg-pixel-bg">
      <p class="text-xs font-pixel text-gray-400 mb-2">今日专注</p>
      <p class="text-2xl font-pixel text-pixel-green">{{ formatMinutes(todayTotal) }}</p>
      <p class="text-xs font-pixel text-gray-500 mt-1">{{ todayCount }}次</p>
    </div>

    <div class="pixel-border p-4 bg-pixel-bg">
      <p class="text-xs font-pixel text-gray-400 mb-2">本周专注</p>
      <p class="text-2xl font-pixel text-pixel-blue">{{ formatMinutes(weekTotal) }}</p>
      <p class="text-xs font-pixel text-gray-500 mt-1">日均 {{ formatMinutes(Math.round(weekTotal / 7)) }}</p>
    </div>

    <div class="pixel-border p-4 bg-pixel-bg">
      <p class="text-xs font-pixel text-gray-400 mb-2">本月专注</p>
      <p class="text-2xl font-pixel text-pixel-pink">{{ formatMinutes(monthTotal) }}</p>
      <p class="text-xs font-pixel text-gray-500 mt-1">{{ monthCount }}次</p>
    </div>

    <div class="pixel-border p-4 bg-pixel-bg">
      <p class="text-xs font-pixel text-gray-400 mb-2">历史总计</p>
      <p class="text-2xl font-pixel text-pixel-yellow">{{ formatMinutes(totalAll) }}</p>
      <p class="text-xs font-pixel text-gray-500 mt-1">加油！</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { formatMinutes } from '@/utils/stats'

const props = defineProps<{
  sessions: any[]
}>()

const now = new Date()
const todayStart = new Date(now.getFullYear(), now.getMonth(), now.getDate())

const todaySessions = computed(() =>
  props.sessions.filter(s => new Date(s.startTime) >= todayStart)
)

const weekAgo = new Date(now.getTime() - 7 * 24 * 60 * 60 * 1000)
const weekSessions = computed(() =>
  props.sessions.filter(s => new Date(s.startTime) >= weekAgo)
)

const monthAgo = new Date(now.getTime() - 30 * 24 * 60 * 60 * 1000)
const monthSessions = computed(() =>
  props.sessions.filter(s => new Date(s.startTime) >= monthAgo)
)

const todayTotal = computed(() =>
  todaySessions.value.reduce((sum, s) => sum + s.duration, 0)
)
const todayCount = computed(() => todaySessions.value.length)

const weekTotal = computed(() =>
  weekSessions.value.reduce((sum, s) => sum + s.duration, 0)
)

const monthTotal = computed(() =>
  monthSessions.value.reduce((sum, s) => sum + s.duration, 0)
)
const monthCount = computed(() => monthSessions.value.length)

const totalAll = computed(() =>
  props.sessions.reduce((sum, s) => sum + s.duration, 0)
)
</script>
```

### Step 6: 创建趋势图表组件

创建 `src/components/TrendChart.vue`:

```vue
<template>
  <div class="pixel-border p-4 bg-pixel-bg mb-6">
    <h3 class="text-sm font-pixel text-pixel-green mb-4">📈 30天趋势</h3>
    <div class="h-48">
      <canvas ref="chartCanvas"></canvas>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import Chart from 'chart.js/auto'

const chartCanvas = ref<HTMLCanvasElement>()
let chartInstance: Chart | null = null

const props = defineProps<{
  sessions: any[]
}>()

async function renderChart() {
  if (!chartCanvas.value) return

  const stats = await invoke<any[]>('get_stats')

  const labels = stats.map((s: any) => {
    const date = new Date(s.date)
    return `${date.getMonth() + 1}/${date.getDate()}`
  })
  const data = stats.map((s: any) => Math.round(s.total_minutes / 60 * 10) / 10)

  if (chartInstance) {
    chartInstance.destroy()
  }

  const ctx = chartCanvas.value.getContext('2d')
  if (!ctx) return

  chartInstance = new Chart(ctx, {
    type: 'line',
    data: {
      labels,
      datasets: [{
        label: '专注时长(小时)',
        data,
        borderColor: '#39ff14',
        backgroundColor: 'rgba(57, 255, 20, 0.1)',
        borderWidth: 2,
        tension: 0,
        pointRadius: 3,
        pointBackgroundColor: '#39ff14',
      }]
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: {
        legend: { display: false }
      },
      scales: {
        y: {
          beginAtZero: true,
          grid: { color: 'rgba(255, 255, 255, 0.1)' },
          ticks: {
            color: '#9ca3af',
            font: { family: '"Press Start 2P"' }
          }
        },
        x: {
          grid: { display: false },
          ticks: {
            color: '#9ca3af',
            font: { family: '"Press Start 2P"', size: 8 }
          }
        }
      }
    }
  })
}

onMounted(() => renderChart())
watch(() => props.sessions, () => renderChart(), { deep: true })
</script>
```

### Step 7: 创建标签分布组件

创建 `src/components/TagDistribution.vue`:

```vue
<template>
  <div class="pixel-border p-4 bg-pixel-bg">
    <h3 class="text-sm font-pixel text-pixel-green mb-4">🏷️ 标签分布</h3>
    <div class="h-64">
      <canvas ref="chartCanvas"></canvas>
    </div>
    <div v-if="tagStats.length === 0" class="text-center text-gray-500 font-pixel text-xs mt-8">
      暂无标签数据
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue'
import Chart from 'chart.js/auto'
import { calculateTagStats } from '@/utils/stats'

const chartCanvas = ref<HTMLCanvasElement>()
let chartInstance: Chart | null = null

const props = defineProps<{
  sessions: any[]
}>()

const tagStats = computed(() => calculateTagStats(props.sessions))

function renderChart() {
  if (!chartCanvas.value || tagStats.value.length === 0) return

  if (chartInstance) {
    chartInstance.destroy()
  }

  const ctx = chartCanvas.value.getContext('2d')
  if (!ctx) return

  const colors = [
    '#39ff14', '#ff6ec7', '#ffff00', '#00d9ff', '#b14eff',
    '#ff6b35', '#f7931a', '#7b68ee', '#00ced1', '#ffd700'
  ]

  chartInstance = new Chart(ctx, {
    type: 'doughnut',
    data: {
      labels: tagStats.value.map(s => s.tag),
      datasets: [{
        data: tagStats.value.map(s => s.total_minutes),
        backgroundColor: colors,
        borderColor: '#2d1b4e',
        borderWidth: 2
      }]
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: {
        legend: {
          position: 'right',
          labels: {
            color: '#ffffff',
            font: { family: '"Press Start 2P"', size: 10 }
          }
        }
      }
    }
  })
}

onMounted(() => renderChart())
watch(tagStats, () => renderChart(), { deep: true })
</script>
```

### Step 8: 实现统计视图

修改 `src/views/StatisticsView.vue`:

```vue
<template>
  <div class="h-full overflow-y-auto p-4">
    <StatsCards :sessions="sessions" />
    <TrendChart :sessions="sessions" />
    <TagDistribution :sessions="sessions" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useTimerStore } from '@/stores/timer'
import StatsCards from '@/components/StatsCards.vue'
import TrendChart from '@/components/TrendChart.vue'
import TagDistribution from '@/components/TagDistribution.vue'
import type { FocusSession } from '@/types/database'

const timerStore = useTimerStore()
const sessions = ref<FocusSession[]>([])

async function loadSessions() {
  sessions.value = await timerStore.loadSessions()
}

onMounted(() => loadSessions())
</script>
```

### Step 9: 测试统计页面

Run: `npm run tauri dev`
Expected: 显示统计数据卡片、趋势图和标签分布图

### Step 10: 提交统计功能

```bash
git add src-tauri/src/stats.rs src-tauri/src/commands.rs src/
git commit -m "feat: implement statistics page with charts"
```

---

## Task 8: 声音提醒系统

**目标:** 实现 8-bit 风格的声音提醒

**文件:**
- Create: `src-tauri/src/sound.rs`
- Create: `src-tauri/resources/sounds/`
- Modify: `src-tauri/src/commands.rs`
- Create: `src/stores/settings.ts`

### Step 1: 创建音频文件目录

Run: `mkdir -p src-tauri/resources/sounds`
Expected: 目录创建成功

### Step 2: 下载或创建 8-bit 音效

在 `src-tauri/resources/sounds/` 目录添加以下音频文件:
- `complete.mp3` - 专注完成音效（8-bit coin/achievement 声音）
- `tick.mp3` - 倒计时提示音

可以使用在线工具生成 8-bit 音效或下载免费资源

### Step 3: 实现声音播放模块

创建 `src-tauri/src/sound.rs`:

```rust
use std::path::PathBuf;

pub fn play_sound(sound_path: &PathBuf) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("afplay")
            .arg(sound_path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("powershell")
            .args(&["-c", &(format!(r#"(New-Object Media.SoundPlayer '{}').PlaySync()"#, sound_path.to_string_lossy()))])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("aplay")
            .arg(sound_path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn get_resource_sound_path(sound_name: &str) -> PathBuf {
    PathBuf::from(format!("../src-tauri/resources/sounds/{}", sound_name))
}

pub fn play_completion_sound() -> Result<(), String> {
    let sound_path = get_resource_sound_path("complete.mp3");
    play_sound(&sound_path)
}

pub fn play_tick_sound() -> Result<(), String> {
    let sound_path = get_resource_sound_path("tick.mp3");
    play_sound(&sound_path)
}
```

### Step 4: 添加声音命令

修改 `src-tauri/src/commands.rs`:

```rust
use crate::sound;

#[tauri::command]
pub async fn play_completion_sound() -> Result<(), String> {
    sound::play_completion_sound()
}

#[tauri::command]
pub async fn play_tick_sound() -> Result<(), String> {
    sound::play_tick_sound()
}
```

### Step 5: 注册声音命令

修改 `src-tauri/src/main.rs`:

```rust
mod sound;

// 在 invoke_handler 中添加:
commands::play_completion_sound,
commands::play_tick_sound,
```

### Step 6: 创建设置 Store

创建 `src/stores/settings.ts`:

```typescript
import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useSettingsStore = defineStore('settings', () => {
  const soundEnabled = ref(true)
  const soundVolume = ref(0.7)
  const defaultDuration = ref(25)

  function toggleSound() {
    soundEnabled.value = !soundEnabled.value
  }

  function setVolume(volume: number) {
    soundVolume.value = Math.max(0, Math.min(1, volume))
  }

  return {
    soundEnabled,
    soundVolume,
    defaultDuration,
    toggleSound,
    setVolume,
  }
})
```

### Step 7: 在计时结束时播放声音

修改 `src/views/TimerView.vue` 在 handleStop 函数中添加声音:

```typescript
import { invoke } from '@tauri-apps/api/core'
import { useSettingsStore } from '@/stores/settings'

const settingsStore = useSettingsStore()

async function handleStop(completed: boolean) {
  await timerStore.stopSession(completed)
  isRunning.value = false
  remainingSeconds.value = 0
  if (timerInterval) {
    clearInterval(timerInterval)
    timerInterval = null
  }

  if (completed && settingsStore.soundEnabled) {
    await invoke('play_completion_sound')
    alert('🎉 恭喜！完成了一次专注！')
  }
}
```

### Step 8: 测试声音功能

Run: `npm run tauri dev`
Expected: 专注完成时播放 8-bit 风格提示音

### Step 9: 提交声音功能

```bash
git add src-tauri/src/sound.rs src/stores/settings.ts src/
git commit -m "feat: add 8-bit sound notifications"
```

---

## Task 9: 设置页面和数据导出

**目标:** 实现设置页面和数据导出/导入功能

**文件:**
- Modify: `src/views/SettingsView.vue`
- Create: `src/components/ExportButton.vue`
- Create: `src/components/ImportButton.vue`

### Step 1: 安装 Tauri 插件

Run: `npm install @tauri-apps/plugin-dialog @tauri-apps/plugin-fs`

修改 `src-tauri/Cargo.toml` 添加:

```toml
tauri-plugin-dialog = "2.0"
tauri-plugin-fs = "2.0"
```

### Step 2: 创建导出按钮组件

创建 `src/components/ExportButton.vue`:

```vue
<template>
  <button
    @click="handleExport"
    class="pixel-button pixel-border border-pixel-green text-pixel-green px-6 py-3 font-pixel text-sm hover:bg-pixel-green hover:text-black"
  >
    📤 导出数据
  </button>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { save } from '@tauri-apps/plugin-dialog'
import { writeTextFile } from '@tauri-apps/plugin-fs'

async function handleExport() {
  try {
    const filePath = await save({
      defaultPath: 'focusflow-export.json',
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }]
    })

    if (filePath) {
      const jsonData = await invoke<string>('export_data')
      await writeTextFile(filePath, jsonData)
      alert('✅ 数据导出成功！')
    }
  } catch (error) {
    alert('❌ 导出失败: ' + error)
  }
}
</script>
```

### Step 3: 创建导入按钮组件

创建 `src/components/ImportButton.vue`:

```vue
<template>
  <button
    @click="handleImport"
    class="pixel-button pixel-border border-pixel-blue text-pixel-blue px-6 py-3 font-pixel text-sm hover:bg-pixel-blue hover:text-black"
  >
    📥 导入数据
  </button>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { readTextFile } from '@tauri-apps/plugin-fs'

const emit = defineEmits<{
  imported: []
}>()

async function handleImport() {
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'JSON',
        extensions: ['json']
      }]
    })

    if (selected && typeof selected === 'string') {
      const confirmed = confirm('⚠️ 导入将覆盖现有数据，确定继续吗？')
      if (!confirmed) return

      const jsonData = await readTextFile(selected)
      const count = await invoke<number>('import_data', { jsonData })
      alert(`✅ 成功导入 ${count} 条记录！`)
      emit('imported')
    }
  } catch (error) {
    alert('❌ 导入失败: ' + error)
  }
}
</script>
```

### Step 4: 实现设置页面

修改 `src/views/SettingsView.vue`:

```vue
<template>
  <div class="h-full overflow-y-auto p-6">
    <h2 class="text-xl font-pixel text-pixel-green mb-8 text-center">⚙️ 设置</h2>

    <!-- 声音设置 -->
    <div class="pixel-border p-6 mb-6 bg-pixel-bg">
      <h3 class="text-sm font-pixel text-pixel-green mb-4">🔊 声音</h3>

      <div class="flex items-center justify-between mb-4">
        <span class="font-pixel text-sm">启用提示音</span>
        <button
          @click="settingsStore.toggleSound"
          class="pixel-button px-4 py-2 font-pixel text-xs"
          :class="settingsStore.soundEnabled ? 'bg-pixel-green text-black' : 'bg-gray-700 text-gray-400'"
        >
          {{ settingsStore.soundEnabled ? 'ON' : 'OFF' }}
        </button>
      </div>

      <div v-if="settingsStore.soundEnabled" class="mb-4">
        <label class="font-pixel text-xs block mb-2">音量: {{ Math.round(settingsStore.soundVolume * 100) }}%</label>
        <input
          type="range"
          min="0"
          max="100"
          :value="settingsStore.soundVolume * 100"
          @input="handleVolumeChange"
          class="w-full"
        />
      </div>

      <button
        @click="testSound"
        class="pixel-button pixel-border border-pixel-yellow text-pixel-yellow px-4 py-2 font-pixel text-xs hover:bg-pixel-yellow hover:text-black"
      >
        🔔 测试音效
      </button>
    </div>

    <!-- 默认时长 -->
    <div class="pixel-border p-6 mb-6 bg-pixel-bg">
      <h3 class="text-sm font-pixel text-pixel-green mb-4">⏱️ 默认时长</h3>

      <div class="flex gap-3">
        <button
          v-for="duration in [15, 25, 45, 60]"
          :key="duration"
          @click="settingsStore.defaultDuration = duration"
          class="pixel-button px-4 py-2 font-pixel text-xs"
          :class="settingsStore.defaultDuration === duration ? 'bg-pixel-green text-black' : 'pixel-border border-pixel-green'"
        >
          {{ duration }}分钟
        </button>
      </div>
    </div>

    <!-- 数据管理 -->
    <div class="pixel-border p-6 mb-6 bg-pixel-bg">
      <h3 class="text-sm font-pixel text-pixel-green mb-4">💾 数据管理</h3>

      <div class="flex gap-4 mb-4">
        <ExportButton />
        <ImportButton @imported="handleImported" />
      </div>

      <button
        @click="handleClearData"
        class="pixel-button pixel-border border-pixel-pink text-pixel-pink px-6 py-3 font-pixel text-sm hover:bg-pixel-pink hover:text-black"
      >
        🗑️ 清除所有数据
      </button>
    </div>

    <!-- 关于 -->
    <div class="pixel-border p-6 bg-pixel-bg text-center">
      <h3 class="text-lg font-pixel text-pixel-green mb-2">FOCUS FLOW</h3>
      <p class="text-xs font-pixel text-gray-400 mb-4">版本 0.1.0</p>
      <p class="text-xs font-pixel text-gray-500">复古像素风番茄钟</p>
      <p class="text-xs font-pixel text-gray-500 mt-2">保持专注，成就梦想 💪</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { useSettingsStore } from '@/stores/settings'
import ExportButton from '@/components/ExportButton.vue'
import ImportButton from '@/components/ImportButton.vue'

const settingsStore = useSettingsStore()

function handleVolumeChange(event: Event) {
  const target = event.target as HTMLInputElement
  settingsStore.setVolume(parseInt(target.value) / 100)
}

async function testSound() {
  await invoke('play_completion_sound')
}

function handleImported() {
  alert('数据已更新，请刷新页面查看')
}

async function handleClearData() {
  const confirmed = confirm('⚠️ 确定要清除所有数据吗？此操作不可恢复！')
  if (confirmed) {
    const doubleConfirm = prompt('请输入 "DELETE" 确认删除')
    if (doubleConfirm === 'DELETE') {
      alert('功能开发中...')
    }
  }
}
</script>
```

### Step 5: 注册插件

修改 `src-tauri/src/main.rs`:

```rust
.use(tauri_plugin_dialog::init())
.use(tauri_plugin_fs::init())
```

### Step 6: 测试设置页面

Run: `npm run tauri dev`
Expected: 可以切换声音、调整音量、导出/导入数据

### Step 7: 提交设置页面

```bash
git add src-tauri/ src/
git commit -m "feat: implement settings page with data export/import"
```

---

## Task 10: 最终优化和测试

**目标:** 完善功能、修复 bug、优化体验

### Step 1: 创建应用图标

在 `src-tauri/icons/` 添加应用图标文件:
- `32x32.png`
- `128x128.png`
- `128x128@2x.png`
- `icon.icns` (macOS)
- `icon.ico` (Windows)

可以使用在线工具生成复古像素风格的图标

### Step 2: 优化计时器精度

修改 `src/views/TimerView.vue` 使用更精确的计时:

```typescript
function startTimer() {
  const endTime = Date.now() + remainingSeconds.value * 1000

  timerInterval = setInterval(() => {
    const now = Date.now()
    const diff = Math.max(0, endTime - now)
    remainingSeconds.value = Math.floor(diff / 1000)

    if (diff <= 0) {
      handleStop(true)
    }
  }, 100) as any
}
```

### Step 3: 添加本地存储持久化设置

修改 `src/stores/settings.ts`:

```typescript
import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export const useSettingsStore = defineStore('settings', () => {
  const soundEnabled = ref(true)
  const soundVolume = ref(0.7)
  const defaultDuration = ref(25)

  // 从 localStorage 加载
  const saved = localStorage.getItem('focusflow-settings')
  if (saved) {
    try {
      const parsed = JSON.parse(saved)
      soundEnabled.value = parsed.soundEnabled ?? true
      soundVolume.value = parsed.soundVolume ?? 0.7
      defaultDuration.value = parsed.defaultDuration ?? 25
    } catch (e) {
      console.error('Failed to load settings', e)
    }
  }

  // 自动保存到 localStorage
  watch([soundEnabled, soundVolume, defaultDuration], () => {
    localStorage.setItem('focusflow-settings', JSON.stringify({
      soundEnabled: soundEnabled.value,
      soundVolume: soundVolume.value,
      defaultDuration: defaultDuration.value,
    }))
  }, { deep: true })

  function toggleSound() {
    soundEnabled.value = !soundEnabled.value
  }

  function setVolume(volume: number) {
    soundVolume.value = Math.max(0, Math.min(1, volume))
  }

  return {
    soundEnabled,
    soundVolume,
    defaultDuration,
    toggleSound,
    setVolume,
  }
})
```

### Step 4: 添加错误边界

创建 `src/AppError.vue`:

```vue
<template>
  <div class="h-full flex items-center justify-center bg-pixel-bg">
    <div class="pixel-border p-8 text-center">
      <p class="text-4xl mb-4">😵</p>
      <p class="font-pixel text-pixel-pink mb-4">出错了！</p>
      <p class="font-pixel text-sm text-gray-400 mb-6">{{ error?.message }}</p>
      <button
        @click="reload"
        class="pixel-button pixel-border border-pixel-green text-pixel-green px-6 py-3 font-pixel text-sm"
      >
        重新加载
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onErrorCaptured, ref } from 'vue'

const error = ref<Error | null>(null)

onErrorCaptured((err) => {
  error.value = err
  return true
})

function reload() {
  window.location.reload()
}
</script>
```

### Step 5: 添加键盘快捷键

在 `src/views/TimerView.vue` 添加:

```typescript
import { onMounted, onUnmounted } from 'vue'

function handleKeyPress(event: KeyboardEvent) {
  // Space: 开始/暂停
  if (event.code === 'Space' && !event.repeat) {
    event.preventDefault()
    if (!isRunning.value && remainingSeconds.value === 0) {
      handleStart()
    } else if (isRunning.value) {
      handlePause()
    } else if (remainingSeconds.value > 0) {
      handleResume()
    }
  }

  // Escape: 停止
  if (event.code === 'Escape' && remainingSeconds.value > 0) {
    event.preventDefault()
    handleStop(false)
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeyPress)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyPress)
})
```

### Step 6: 完善响应式布局

修改 `src/components/TimerDisplay.vue` 添加响应式:

```vue
<template>
  <div class="flex flex-col items-center gap-8 px-4">
    <!-- 响应式尺寸 -->
    <div class="relative" :class="isSmallScreen ? 'w-56 h-56' : 'w-72 h-72'">
      <!-- ... -->
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const isSmallScreen = ref(false)

onMounted(() => {
  isSmallScreen.value = window.innerWidth < 768
})
</script>
```

### Step 7: 跨平台测试

在 macOS、Windows、Linux 上测试:
- 计时器功能
- 数据存储
- 声音播放
- 文件导出/导入

### Step 8: 性能优化

- 添加虚拟滚动处理大量历史记录
- 优化图表渲染
- 减少不必要的重渲染

### Step 9: 最终构建测试

Run: `npm run tauri build`
Expected: 成功构建各平台安装包

### Step 10: 创建 README

创建 `README.md`:

```markdown
# FocusFlow - 复古像素风番茄钟

一款跨平台的番茄钟应用，拥有复古像素风格的界面。

## 特性

- ⏱️ 自定义专注时长
- 🏷️ 任务标签系统
- 📝 历史记录管理
- 📊 统计图表展示
- 🔔 8-bit 风格声音提醒
- 💾 数据导出/导入
- 🖥️ 跨平台支持

## 技术栈

- Tauri 2.x
- Vue 3
- TypeScript
- Tailwind CSS
- SQLite
- Chart.js

## 开发

\`\`\`bash
npm install
npm run tauri dev
\`\`\`

## 构建

\`\`\`bash
npm run tauri build
\`\`\`

## 许可证

MIT
```

### Step 11: 最终提交

```bash
git add .
git commit -m "feat: complete FocusFlow pomodoro timer app"
git tag v0.1.0
```

---

## 🎉 计划完成

**总任务数:** 10 个主要任务
**预计开发时间:** 20-30 小时
**技术栈:** Tauri 2.x + Vue 3 + TypeScript + SQLite + Chart.js

**核心特性:**
✅ 复古像素风格 UI
✅ 自定义专注时长
✅ 任务标签系统
✅ 历史记录管理
✅ 统计图表展示
✅ 8-bit 声音提醒
✅ 数据导出/导入
✅ 跨平台支持

**开发顺序建议:**
1. Task 1-3: 基础架构和后端
2. Task 4-6: 核心 UI 和功能
3. Task 7-9: 高级功能
4. Task 10: 优化和完善

**准备好开始实施！** 🚀

---

**Plan complete and saved to `docs/plans/2026-01-21-focusflow-pomodoro-timer.md`**
