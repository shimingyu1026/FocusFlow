# 音效文件说明

## 默认行为

FocusFlow 现在已经内置了系统默认的 beep 音效作为后备，即使没有自定义音效文件，应用也能正常工作！

## 添加自定义 8-bit 音效

如果你想添加更复古的 8-bit 音效，可以按以下步骤操作：

### 方法 1: 使用在线音效资源

1. 访问免费音效网站下载 8-bit 风格的音效：
   - [Freesound](https://freesound.org/) - 搜索 "8-bit", "coin", "achievement"
   - [Zapsplat](https://zapsplat.com/) - 搜索 "retro", "pixel"

2. 下载以下音效文件：
   - `complete.mp3` - 专注完成音效（推荐：coin/achievement 声音）
   - `tick.mp3` - 倒计时提示音（可选）

3. 将下载的文件放到：
   ```
   src-tauri/resources/sounds/
   ```

### 方法 2: 使用在线生成工具

1. 访问 [sfxr.me](https://sfxr.me/) - 免费的在线音效生成器
2. 选择 8-bit / Retro 风格
3. 生成并下载 `complete.mp3`

### 方法 3: macOS 系统（最简单）

如果你使用 macOS，可以直接使用系统内置音效：

```bash
# 测试系统音效
afplay /System/Library/Sounds/Ping.aiff
```

## 音效文件推荐

**完成音效:**
- 🪙 coin 收集音效
- 🎵 level up 音效
- 🔔 achievement 解锁音效
- ⭐ power-up 音效

**提示音效:**
- ⏰ 短促的 tick 音
- 🔔 轻微的 reminder 音
- 📱 notification 音效

## 文件格式

- **格式:** MP3, WAV, AIFF, OGG
- **时长:** 1-3 秒
- **音量:** 不要太大，避免刺耳

## 验证音效

添加音效后，在应用中点击"🔔 测试音效"按钮即可听到效果。

## 故障排除

如果音效不工作：
1. 检查文件是否在正确位置：`src-tauri/resources/sounds/`
2. 检查文件名是否正确：`complete.mp3`, `tick.mp3`
3. 查看开发者控制台是否有错误信息

## 当前默认行为

如果没有自定义音效文件，应用会使用：
- macOS: 系统 Ping 音效
- Windows: 系统 beep
- Linux: 系统 beep

这意味着即使不添加任何文件，声音提醒功能也能正常工作！
