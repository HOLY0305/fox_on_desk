<p align="center">
  <img src="assets/icon.png" width="200" alt="Fox on Desk">
</p>
<h1 align="center">Fox on Desk</h1>
<p align="center">
  轻量级 AI 编程桌宠，实时映射助手工作状态
  <br>
  <a href="README.md">English</a>
</p>
<p align="center">
  <img src="https://img.shields.io/badge/v0.1.7-blue" alt="version">
  <img src="https://img.shields.io/badge/Tauri_v2-orange" alt="Tauri v2">
  <img src="https://img.shields.io/badge/Svelte_5-red" alt="Svelte 5">
  <img src="https://img.shields.io/badge/Rust-black" alt="Rust">
  <img src="https://img.shields.io/badge/Windows-grey" alt="Windows">
</p>

Fox 是一只住在桌面上的宠物，能实时感知 AI 编程助手在做什么：提问时思考，跑工具时打字，子代理工作时杂耍，弹卡片审批权限，任务完成时庆祝，你离开时睡觉。

支持 **Claude Code**、**Codex CLI** 和 **Copilot CLI**，三者可同时运行。

> 基于 [Clyde on Desk](https://github.com/QingJ01/Clyde) 二次开发。改进内容见[下方](#相比原版的改进)。

## 快速开始

```bash
git clone https://github.com/HOLY0305/fox_on_desk.git
cd fox_on_desk
npm install
npm start        # Tauri 开发模式，前端热更新
```

**前置条件** — [Node.js](https://nodejs.org/) v18+、[Rust](https://rustup.rs/) stable、Windows 平台 [Tauri 依赖](https://v2.tauri.app/start/prerequisites/)。

**Agent 配置** — 全部零配置：
- **Claude Code** — 启动时自动注册 hooks
- **Codex CLI** — 自动轮询 `~/.codex/sessions/` 日志
- **Copilot CLI** — 检测到 `~/.copilot` 时自动配置

## 相比原版的改进

| 改进 | 说明 |
|------|------|
| **Fox 角色** | 全新狐狸图标和皮肤系统，透明背景 |
| **多皮肤支持** | 右键菜单可切换 Clyde / Fox 皮肤 |
| **XL 尺寸** | 新增 480px 宠物尺寸（S / M / L / XL） |
| **音效系统** | 关键事件播放像素风格音效 |
| **Alt+C 快捷键** | 一键将宠物召唤到鼠标位置 |
| **双击启动** | 双击宠物打开文件夹选择器，启动新 Claude 会话 |
| **AskUserQuestion** | 权限气泡支持 Claude 的结构化输入提示 |
| **AlwaysOnTop 修复** | 其他窗口抢焦点后宠物自动回到顶层 |

## 功能

### 动画

12 种动画状态，由实时 Agent 事件驱动 — 眼球跟踪、思考、打字、建造、杂耍、指挥、报错闪烁、开心弹跳、通知、扫地、搬箱子、睡觉。

### 交互

- **拖拽** — 任何状态下都能拖动，Pointer Capture 防止快甩丢失
- **双击** 戳一下，**连点 4 下** 东张西望
- **右键菜单** — 会话列表、免打扰、极简模式、大小、语言、皮肤切换
- **系统托盘** — 调大小 (S/M/L/XL)、免打扰、极简模式、语言、自动启动、退出

### 极简模式

拖到左/右屏幕边缘（或右键"极简模式"），Fox 藏到边缘只露半身，悬停时探头，收起状态下仍能显示迷你通知和庆祝动画。

### 权限审批气泡

Claude Code 请求工具权限时，Fox 在宠物旁弹出浮动卡片 — 允许、拒绝或选择建议规则。同时支持 **AskUserQuestion** 结构化输入（枚举选择、布尔值、文本框等）。

### 会话智能

- **多会话优先级** — 所有会话中最高优先级的状态胜出
- **子代理感知** — 1 个子代理杂耍，2 个以上指挥
- **终端聚焦** — 右键某个会话可直接跳转到对应终端
- **自动清理** — 10 分钟无更新删除会话
- **免打扰** — 静默所有事件，右键或托盘切换

## 技术栈

| 层 | 技术 | 为什么选它 |
|---|---|---|
| **桌面框架** | [Tauri v2](https://v2.tauri.app/) | ~5 MB 打包；原生系统 API；Rust 后端 |
| **后端** | [Rust](https://www.rust-lang.org/) | 无 GC；多会话状态机；CPU 占用趋近于零 |
| **前端** | [Svelte 5](https://svelte.dev/) | 编译时生成；三个窗口 JS 合计 < 30 KB |
| **HTTP** | [Axum](https://github.com/tokio-rs/axum) | 共享 Tokio 运行时的异步框架 |
| **构建** | [Vite](https://vitejs.dev/) | 毫秒级热更新；生产构建极致精简 |

## 已知限制

| 限制 | 说明 |
|---|---|
| 仅支持 Windows | 仅在 Windows 上测试，macOS/Linux 未验证 |
| Codex: 无终端聚焦 | JSONL 轮询不携带终端 PID |
| Copilot: 无权限气泡 | Copilot hook 协议仅支持拒绝 |
| 无自动更新 | 请从 GitHub Releases 下载新版本 |

## 故障排除

### 权限气泡不弹出

1. 在 Claude Code 中运行 `/hooks`，检查 `PermissionRequest` 是否有 `[http]` hook
2. 如果缺失，重启 Fox — 启动时会自动重新注册
3. 如果仍有问题，手动运行 `node hooks/install.js`
4. 最后手段：删除 `~/.claude/settings.json` 中的 `PermissionRequest` 条目，重启

## 贡献

欢迎 Issue、建议和 PR — [提交 Issue](https://github.com/HOLY0305/fox_on_desk/issues) 或直接提 PR。

## 致谢

- 由 [Clyde on Desk](https://github.com/QingJ01/Clyde) ([@QingJ01](https://github.com/QingJ01)) 二次开发而来
- 原项目 [Clawd on Desk](https://github.com/rullerzhou-afk/clawd-on-desk) by [@rullerzhou-afk](https://github.com/rullerzhou-afk)
- 感谢 [LINUX DO](https://linux.do/) 社区的反馈与支持

## 许可证

[AGPL-3.0](LICENSE)
