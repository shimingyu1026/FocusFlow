# FocusFlow - 复古像素风番茄钟

一款跨平台的番茄钟应用，拥有现代化复古像素风格的界面。

![Version](https://img.shields.io/badge/version-v0.2.0-blue)
![Vue](https://img.shields.io/badge/Vue-3.4-42b883)
![Tauri](https://img.shields.io/badge/Tauri-2.0-FFC131)

## ✨ 特性

### 核心功能
- ⏱️ 自定义专注时长（15/25/45/60 分钟）
- 🎯 SVG 进度环实时显示剩余时间
- 🏷️ 任务标签系统
- 📝 历史记录管理
- 📊 统计图表展示
- 🔔 8-bit 风格声音提醒
- 💾 数据导出/导入
- 🖥️ 跨平台支持（Windows/macOS/Linux）

### v0.2.0 新特性
- 🎨 **现代化像素风格 UI**
  - VT323 优化像素字体，解决文字溢出问题
  - 电光青 + 珊瑚色现代配色方案
  - 3D 立体按钮效果（悬停上浮，点击下沉）
- 📱 **完全响应式设计**
  - 自动缩放适配不同窗口尺寸
  - 优化的移动端和平板体验
- 🎉 **优雅的完成动画**
  - 彩纸粒子庆祝效果
  - 随机激励消息
  - 每日完成次数统计
- ⚡ **性能优化**
  - GPU 加速动画（60fps）
  - 流畅的进度环过渡效果
- ⌨️ **键盘快捷键**
  - `Space`: 开始/暂停/继续计时
  - `Escape`: 停止计时

## 🛠️ 技术栈

### 前端框架
- **Vue 3** - 渐进式 JavaScript 框架（Composition API）
- **TypeScript** - 类型安全的 JavaScript
- **Tailwind CSS** - 原子化 CSS 框架
- **Chart.js** - 数据可视化图表库

### 桌面应用
- **Tauri 2.x** - 跨平台桌面应用框架
- **SQLite** - 轻量级本地数据库

### 开发工具
- **Vite** - 下一代前端构建工具
- **pnpm** - 快速的、节省磁盘空间的包管理器

## 🚀 开发

```bash
# 安装依赖
pnpm install

# 开发模式（Web）
pnpm run dev

# 开发模式（桌面应用）
pnpm run tauri dev
```

## 📦 构建

```bash
# 构建桌面应用
pnpm run tauri build

# 构建产物位于 src-tauri/target/release/
```

## 🎮 快捷键

| 按键 | 功能 |
|------|------|
| `Space` | 开始/暂停/继续计时 |
| `Escape` | 停止计时 |

## 💾 数据备份

应用数据会自动保存到本地。你可以通过设置页面导出数据作为备份。

## 📈 版本历史

### v0.2.0 - Modern Pixel UI (2026-01-21)
- 🎨 全面升级为现代化像素风格
- ✨ 修复计时器数字溢出问题
- 📱 实现完全响应式布局
- 🎯 添加 SVG 进度环动画
- 🎉 新增完成庆祝动画
- ⚡ 性能优化（GPU 加速）

**详细变更**: [docs/changelog/ui-redesign.md](docs/changelog/ui-redesign.md)

### v0.1.0 - Initial Release (2026-01-20)
- ⏱️ 基础番茄钟功能
- 📊 统计和历史记录
- 💾 数据导入/导出

## 🙏 致谢

本项目的 v0.2.0 版本 UI 现代化重构由 **GLM** 与 **Claude Code** 协作完成。

- **GLM (智谱 AI)** - 提供 AI 辅助编程能力
- **Claude Code** - Anthropic 出品的 AI 编程助手
  - 使用 Superpowers 工作流进行系统化开发
  - 采用 Subagent-Driven Development 模式确保代码质量
  - 通过 TDD (Test-Driven Development) 方法保证功能可靠性

**开发亮点**:
- 📋 完整的设计规范和实施计划（1090 行文档）
- 🔍 代码质量审查流程（规范审查 + 质量审查）
- ✅ 全面的测试覆盖（响应式、功能、性能测试）
- 📚 详尽的文档记录（变更日志、测试报告）

## 📄 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📮 联系方式

如有问题或建议，欢迎通过 GitHub Issues 联系。

---

**FocusFlow** - 让专注更简单 🎯⏱️
