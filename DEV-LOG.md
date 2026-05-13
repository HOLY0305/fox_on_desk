# Clyde 魔改开发日志

> 开发周期：2026-05-12 ~ 2026-05-13
> 基于版本：Clyde v0.1.6 (commit 71a73c8)
> 当前版本：commit 1ae0f33（领先 origin/main 22 个提交）

---

## 一、功能开发总览

### 已完成功能

| # | 功能 | 提交 | 状态 |
|---|------|------|------|
| 1 | 音效系统 | e40aa33 | 完成（声音播放需系统音频支持） |
| 2 | 会话状态徽章 | 77cf431 → cee8b8e | 完成（纯视觉圆点） |
| 3 | 宠物尺寸 XL | b593acb | 完成 |
| 4 | 宠物召来 Alt+C | e26ab56 | 完成 |
| 5 | 双击选目录启动 | 5d9ed85 | 完成 |
| 6 | 子代理完成气泡 | dfc537d → 7119a49 | 完成（已移除气泡通知，保留音效） |
| 7 | 权限气泡高度调整 | e9f0109 → 183c169 → cbe5adf | 完成（900px） |
| 8 | AskUserQuestion 支持 | 815cde2 → 1ae0f33 | 完成（allow 后终端选择） |

---

## 二、文件变更清单

### 新增文件

| 文件 | 用途 |
|------|------|
| `src-tauri/src/sfx.rs` | 音效播放模块（rodio + OGG） |
| `src-tauri/sound/*.ogg` | 5 个像素风音效文件 |
| `DEV-PLAN.md` | 产品开发计划 |
| `DEV-LOG.md` | 本开发日志 |
| `src/windows/badge/*` | 徽章面板窗口（已弃用，保留文件） |

### 修改文件

| 文件 | 改动说明 |
|------|---------|
| `src-tauri/Cargo.toml` | 添加 rodio、symphonia、tauri-plugin-dialog、tauri-plugin-global-shortcut |
| `src-tauri/src/lib.rs` | 注册插件、音效初始化、快捷键、双击目录选择、菜单项 |
| `src-tauri/src/http_server.rs` | 音效触发、AskUserQuestion 检测、会话徽章事件 |
| `src-tauri/src/state_machine.rs` | 进程存活检测、会话清理 |
| `src-tauri/src/permission.rs` | ProgressNotice 类型、气泡高度、ask_questions 字段 |
| `src-tauri/src/permission_mode.rs` | BubbleData 新增字段适配 |
| `src-tauri/src/update_check.rs` | BubbleData 新增字段适配 |
| `src-tauri/src/tray.rs` | 音效开关菜单项、XL 尺寸选项 |
| `src-tauri/src/i18n.rs` | "sound" / "音效" 翻译 |
| `src-tauri/tauri.conf.json` | 打包 sound 目录为资源 |
| `src-tauri/capabilities/default.json` | 全局快捷键、对话框权限 |
| `src/windows/pet/App.svelte` | 会话状态圆点叠加层 |
| `src/windows/hit/App.svelte` | 还原为原始版本（徽章曾移入后又移出） |
| `src/windows/bubble/BubbleCard.svelte` | ProgressNotice 支持、AskUserQuestion UI、防御性检查 |
| `vite.config.ts` | badge 窗口入口（已弃用） |

---

## 三、Bug 记录与解决方案

### Bug 1：音效播放无声音

**现象**：日志显示 `sfx queued successfully` 但听不到声音

**排查过程**：
1. OGG 文件格式正确（`file` 命令确认 Vorbis）
2. rodio `Decoder::new()` 需要 `BufReader` 包装
3. Symphonia 后端未编译 Vorbis 解码器
4. `OutputStream` 未保持存活（`Box::leak` 后引用被丢弃）
5. `play_raw()` 可能有格式兼容问题

**解决方案**：
- 添加 `BufReader::new(Cursor::new(data))` 包装
- Cargo.toml 显式添加 `symphonia = { version = "0.5", features = ["vorbis"] }`
- `OutputStream` 存入全局 `static STREAM_KEEPALIVE: Mutex<StreamHolder>`
- 改用 `Sink` API 替代 `play_raw`（自动处理格式转换）
- 音量放大 3 倍（`source.amplify(3.0)`）

**关键代码**（`sfx.rs`）：
```rust
static STREAM_KEEPALIVE: Mutex<StreamHolder> = Mutex::new(StreamHolder(None));

fn play_bytes(&self, data: &[u8]) {
    let cursor = Cursor::new(data.to_vec());
    let buf_reader = BufReader::new(cursor);
    match Decoder::new(buf_reader) {
        Ok(source) => {
            match Sink::try_new(&self.handle) {
                Ok(sink) => {
                    sink.append(source.amplify(3.0));
                    sink.detach();
                }
                Err(e) => eprintln!("sink error: {e}"),
            }
        }
        Err(e) => eprintln!("decode error: {e}"),
    }
}
```

---

### Bug 2：徽章圆点点击无反应

**现象**：圆点显示正常但点击无反应

**根因**：宠物窗口设置了 `set_ignore_cursor_events(true)`，所有鼠标事件穿透到 hit 窗口。圆点在宠物窗口中，无法接收点击。

**尝试过的方案**：
1. 把圆点移到 hit 窗口 → 干扰了拖拽/双击/右键交互（hit 窗口的 pointer-events 冲突）
2. 创建独立 badge-panel 窗口 → 窗口无法关闭、体验割裂

**最终方案**：圆点放回宠物窗口，设为纯视觉（`pointer-events: none`），不接收点击。交互通过右键菜单的"会话"子菜单完成。

---

### Bug 3：徽章圆点过一会消失

**现象**：Claude Code 空闲时圆点消失

**根因**：`clean_stale()` 函数在会话无更新 10 分钟后删除会话。Claude Code 空闲时不发送事件，`updated_at` 不刷新。

**相关改动**：
- 曾将超时从 10 分钟改为 2 分钟（导致圆点快速消失）→ 改回 10 分钟
- 新增进程存活检测（`is_process_alive`）→ 加 30 秒宽限期避免误删

**结论**：这是正常行为。Claude Code 无心跳机制，Clyde 只能靠超时清理。

---

### Bug 4：气泡 HTML 结构嵌套错误

**现象**：气泡完全不渲染

**根因**：在 `BubbleCard.svelte` 中尝试添加 flex/scroll 布局时，`<div class="bubble-scroll-area">` 的关闭标签放在了 `{#if isElicitation}` 分支内部，导致非 elicitation 分支的 HTML 嵌套错误。

**解决方案**：还原 `BubbleCard.svelte` 到修改前版本（`git show HEAD~1:...`），放弃复杂的 flex 重构。

**教训**：Svelte 的 `{#if}` / `{:else}` 块中的 HTML 嵌套必须严格匹配，不能跨分支开关标签。

---

### Bug 5：PowerShell 窗格中 claude 找不到

**现象**：`wt -w 0 sp powershell` 成功开窗格，但报错 `系统找不到指定的文件`

**根因**：`wt` 命令的参数解析将 PowerShell 命令当作文件路径。变量赋值语法 `$c = Get-Command...` 被 `wt` 错误解析。

**尝试过的方案**：
1. `wt -w 0 sp powershell -Command "..."` → `wt` 把命令当文件路径
2. `wt -w 0 sp -- powershell -Command "..."` → 同样失败
3. 写临时 `.ps1` 脚本 + `wt -w 0 sp -- powershell -File script.ps1` → 最终方案

**最终方案**：
```rust
let script_path = std::env::temp_dir().join("clyde_launch.ps1");
std::fs::write(&script_path, &script);
let pane_ok = Command::new("wt")
    .args(["-w", "0", "sp", "--", "powershell", "-NoExit",
           "-ExecutionPolicy", "Bypass", "-File", &script_path.to_string()])
    .spawn().is_ok();
if !pane_ok {
    // fallback: 新窗口
    Command::new("powershell").args(["-NoExit", "-File", &script_path]).spawn();
}
```

---

### Bug 6：AskUserQuestion 选择题只显示 Allow/Deny

**现象**：Claude Code 给出选择题时，气泡只有 Allow/Deny 按钮

**根因**：Claude Code 的选择题使用 `AskUserQuestion` 工具通过 `PermissionRequest` hook 发送，不是 `Elicitation`。`tool_input` 中包含 `questions` 数组和 `options`，但 Clyde 没有识别这个工具。

**排查过程**：
1. 查阅 Claude Code 官方文档（code.claude.com/docs/en/hooks）
2. 确认 `AskUserQuestion` 是 `PermissionRequest` 的一种特殊工具
3. 确认 `Elicitation` 仅用于 MCP 服务器的用户输入请求

**解决方案**：
- 在 `post_permission` 中检测 `tool_name === "AskUserQuestion"`
- 从 `tool_input.questions[0].options` 提取选项作为 suggestions
- 气泡显示选项按钮供参考
- 点击 "Allow Once" 时直接 allow（不带 answers），Claude Code 在终端正常提示用户

---

### Bug 7：AskUserQuestion 响应导致 H.map 错误

**现象**：尝试自动回答时，工具内部报 `undefined is not an object (evaluating 'H.map')`

**根因**：`updatedInput` 会**替换整个 tool_input**。最初只发送 `answers` 字段，导致工具内部的 `questions` 变成 undefined，调用 `.map()` 崩溃。

**尝试过的修复**：
1. 在 `updatedInput` 中同时包含 `questions` 和 `answers` → 仍然报错
2. 存储原始 `questions` 数组并在响应中传递 → 仍然报错
3. 移除内部 `_questions` 字段避免序列化干扰 → 仍然报错

**最终方案**：放弃自动回答，直接 `behavior: "allow"` 不带 `updatedInput`。Claude Code 收到 allow 后会在终端正常提示用户选择。

---

### Bug 8：子代理完成气泡过于频繁

**现象**：用户未看到子代理运行，但频繁弹出"子代理已完成"气泡

**根因**：Claude Code 内部自动创建子代理处理并行操作（如同时搜索多个文件），用户不可见但 `SubagentStop` 事件会触发。

**解决方案**：移除子代理完成的气泡通知，仅保留音效。

---

## 四、架构决策记录

### 决策 1：音效使用 rodio 而非系统 API

选择 rodio（Rust 音频库）而非 Windows PlaySound API，原因：
- 跨平台兼容
- 支持 OGG Vorbis 格式（体积小）
- 可以精确控制播放时机和音量

代价：增加了 `rodio` + `symphonia` 依赖（约 200KB 体积增加）。

### 决策 2：徽章圆点纯视觉方案

选择在宠物窗口渲染圆点（pointer-events: none），不接收点击。原因：
- 宠物窗口设置了 `set_ignore_cursor_events(true)`，无法接收鼠标事件
- 在 hit 窗口添加元素会干扰现有交互
- 独立窗口方案体验割裂（无法关闭、定位问题）

### 决策 3：AskUserQuestion 采用 allow 策略

选择直接 allow 而非尝试自动回答。原因：
- `updatedInput` 的格式要求不明确（文档示例与实际行为不一致）
- 多次尝试自动回答均导致工具内部 JS 错误
- allow 后 Claude Code 在终端正常提示，用户体验可接受

### 决策 4：会话清理使用进程存活检测 + 超时双保险

- `is_process_alive(pid)`：通过 Windows API 检测进程是否存活（30 秒宽限期）
- `SESSION_STALE_SECS = 600`：10 分钟无更新则删除会话
- 过滤 `claude-monitor-` 前缀的会话避免重复

---

## 五、当前已知问题

| 问题 | 严重程度 | 说明 |
|------|---------|------|
| 音效在某些系统上可能无声 | 低 | 依赖系统音频设备和驱动 |
| 徽章圆点空闲时消失 | 低 | Claude Code 无心跳机制，10 分钟超时后清理 |
| AskUserQuestion 无自动回答 | 低 | allow 后用户需在终端手动选择 |
| 气泡内容过长时需滚动 | 低 | 已设置 900px 最大高度，但未实现 flex 固定按钮布局 |
| badge 窗口目录残留 | 无 | `src/windows/badge/` 目录保留但未使用 |

---

## 六、构建与运行

### 环境要求
- Node.js v18+
- Rust stable
- Tauri v2 依赖（见 Tauri 官方文档）

### 常用命令
```bash
# 安装依赖
npm install

# 开发模式（前端热更新 + Rust 重编译）
npm start

# 编译检查
cd src-tauri && cargo check

# 运行测试
cargo test --manifest-path src-tauri/Cargo.toml
```

### 音效文件位置
`src-tauri/sound/` 目录下 5 个 OGG 文件：
- `permission_request.ogg` — 权限请求提示音
- `task_complete.ogg` — 任务完成音
- `error.ogg` — 错误警告音
- `session_start.ogg` — 新会话开始音
- `subagent_complete.ogg` — 子代理完成音

替换音效时保持文件名不变即可。

---

## 七、提交历史（本次开发）

```
1ae0f33 fix(ask): simplify AskUserQuestion to allow without answers
bc75bed fix: pass original questions in updatedInput for AskUserQuestion
101bd45 fix: include questions in updatedInput for AskUserQuestion
1f4b025 fix(bubble): defensive checks for suggestions array
815cde2 fix(bubble): support AskUserQuestion choice prompts
c5e0131 fix(bubble): extract choice options from permission requests
cee8b8e fix: move badge dots closer to pet body
61c2eed fix: badge dots as visual-only overlay, remove badge-panel window
4bbe94e fix: move badge to hit window for click support, fix claude path
cbe5adf fix(bubble): increase max height to 900px
7119a49 fix: remove subagent completion bubble notification
183c169 fix(bubble): increase bubble height to 700px
e9f0109 fix(bubble): increase bubble window height
ac8709c fix(bubble): restore BubbleCard to working state
35f971a fix: badge panel, bubble scroll layout, terminal pane launch
d9f7644 fix: badge positioning, PowerShell launch, bubble overflow
dfc537d feat(progress): show progress bubble on subagent completion
5d9ed85 feat(dialog): double-click pet opens folder picker for new Claude session
e26ab56 feat(shortcut): add Alt+C to summon pet to cursor
b593acb feat(size): add XL (480px) pet size option
77cf431 feat(badge): add session status badge dots on pet
e40aa33 feat(sfx): add pixel-style sound effect system
```

---

## 八、后续可优化方向

1. **音效替换**：从 kenney.nl 下载更好的像素风音效替换当前占位音
2. **新动画 SVG**：挠头、喝咖啡、挥手、鼓掌、派活（需美术制作）
3. **气泡 flex 布局**：操作按钮固定底部，中间内容可滚动
4. **徽章交互**：如果未来 Tauri 支持更灵活的窗口事件穿透，可以实现可点击圆点
5. **会话心跳**：通过 claude_monitor 定期刷新 updated_at，避免空闲时圆点消失

---

## 九、打包构建

### 构建命令
```bash
npm run tauri build
```

### 构建产物

| 格式 | 路径 | 大小 |
|------|------|------|
| NSIS 安装包 | `src-tauri/target/release/bundle/nsis/Clyde on Desk_0.1.6_x64-setup.exe` | 2.5 MB |
| MSI 安装包 | `src-tauri/target/release/bundle/msi/Clyde on Desk_0.1.6_x64_en-US.msi` | 3.7 MB |
| 独立可执行文件 | `src-tauri/target/release/clyde.exe` | — |

### 构建流程
1. Vite 编译前端（HTML/CSS/JS → `dist/`）
2. Rust release 编译（LTO + strip + opt-level="s"）
3. 前端资源嵌入 Rust 二进制
4. NSIS/MSI 打包成安装程序

### 构建环境要求
- Node.js v18+
- Rust stable（target: x86_64-pc-windows-msvc）
- Tauri CLI v2（`npx tauri`）
- WiX Toolset（MSI 打包，自动下载）
- NSIS（exe 打包，自动下载）

### 注意事项
- 首次构建需下载 WiX 和 NSIS 工具（约 30MB）
- release 编译约 3 分钟（取决于 CPU）
- 产物体积 ~2.5MB（含前端 + 后端 + 音效 + SVG 资源）
