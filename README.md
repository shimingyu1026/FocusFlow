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

```bash
# 安装依赖
pnpm install

# 开发模式
pnpm run tauri dev
```

## 构建

```bash
# 构建应用
pnpm run tauri build
```

## 快捷键

- `Space`: 开始/暂停/继续计时
- `Escape`: 停止计时

## 数据备份

应用数据会自动保存到本地。你可以通过设置页面导出数据作为备份。

## 许可证

MIT
