# Clyde 宠物形象替换规格书

> 本文档用于指导美术/AI 完成宠物形象的全套替换工作。
> 所有文件必须保持现有文件名不变，替换同名文件即可。

---

## 一、总体要求

### 风格
- **像素风格**（pixel art），与现有角色风格统一
- 参考：https://github.com/marciogranzotto/clawd-tank 的 Clawd 角色
- 配色：暖色调为主，角色辨识度高
- 角色建议：小动物（猫、狗、兔、机器人等均可），需要有明确的"眼睛"和"身体"可分离部分

### 尺寸
- 所有 SVG 使用统一的 viewBox：`-15 -25 45 45`
- width/height 设为 `500`（代码会自动移除，不影响实际渲染）
- 角色在 viewBox 中居中，占约 60-70% 面积

### 文件格式
- SVG 文件，内含 CSS animation 或 SMIL 动画
- 每个文件是一个独立的动画帧/循环
- 文件大小建议 < 10KB（像素风 SVG 通常 2-5KB）

---

## 二、必须保留的 SVG 元素 ID

代码通过 JavaScript 操作以下 ID 的元素实现鼠标跟随效果。**必须在每个 SVG 中包含这些 ID 的元素**。

### `#eyes-js`（眼球/眼睛）
- **用途**：代码会通过 CSS `transform: translate(dx, dy)` 移动此元素，实现眼球跟随鼠标
- **要求**：必须是一个可独立移动的 `<g>` 元素，包含角色的眼睛部分
- **位置**：眼睛的默认位置应在角色脸部中心偏上
- **移动范围**：代码会施加 `translate(±3px, ±2px)` 的偏移

### `#body-js`（身体）
- **用途**：代码会通过 CSS `transform: translate(dx, 0)` 微移此元素，实现身体向鼠标方向倾斜
- **要求**：必须是一个 `<g>` 元素，包含角色的躯干/身体部分
- **偏移量**：代码会施加 `translate(±1.5px, 0)`

### `#shadow-js`（阴影）
- **用途**：代码会通过 CSS `transform: scaleX(...)` 拉伸阴影，与身体倾斜方向相反
- **要求**：必须是一个 `<g>` 元素，包含角色脚下的阴影/投影
- **形状**：椭圆形阴影，宽度约等于角色宽度的 60%

### 示例结构
```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="-15 -25 45 45" width="500" height="500">
  <defs>
    <style>
      /* 动画定义 */
    </style>
  </defs>

  <!-- 阴影（最底层） -->
  <g id="shadow-js">
    <ellipse cx="7.5" cy="18" rx="6" ry="2" fill="rgba(0,0,0,0.2)" />
  </g>

  <!-- 身体（中间层） -->
  <g id="body-js">
    <!-- 角色身体像素 -->
  </g>

  <!-- 眼睛（最上层，可独立移动） -->
  <g id="eyes-js">
    <!-- 角色眼睛像素 -->
  </g>
</svg>
```

---

## 三、35 个 SVG 动画文件清单

### 分组 A：基础状态（6 个）

#### 1. `clyde-idle-follow.svg` — 默认站立（眼球跟随鼠标）
- 角色站立，眼睛跟随鼠标方向移动
- 身体有轻微呼吸动画（上下浮动 1px，周期 2s）
- **这是最常显示的状态，优先级最高**

#### 2. `clyde-idle-living.svg` — 站立待机
- 角色站立，有呼吸动画
- 偶尔眨眼（每 3-4 秒一次）
- 无鼠标跟随

#### 3. `clyde-idle-look.svg` — 四处张望
- 角色头部左右转动（约 ±15°）
- 周期约 3 秒

#### 4. `clyde-static-base.svg` — 静态基础
- 完全静止的角色正面图
- 用于其他动画的基帧

#### 5. `clyde-notification.svg` — 惊跳反应
- 角色突然跳起（向上 3-4px）然后落下
- 眼睛睁大
- 持续约 600ms

#### 6. `clyde-error.svg` — 错误闪烁
- 角色身体快速闪烁红色/原色交替
- 周期约 200ms，持续约 1 秒
- 表情：惊讶/困惑

### 分组 B：工作状态（9 个）

#### 7. `clyde-working-typing.svg` — 打字
- 角色双手快速上下移动（模拟打字）
- 身体微微前倾
- 周期约 300ms

#### 8. `clyde-working-thinking.svg` — 思考
- 角色一手托下巴，头部微微倾斜
- 眼睛向上看
- 周期约 2s 的缓慢摆动

#### 9. `clyde-working-juggling.svg` — 杂耍（2 个会话同时工作）
- 角色双手交替抛接 2-3 个小球
- 小球在空中画弧线
- 周期约 1s

#### 10. `clyde-working-building.svg` — 建造（3+ 个会话）
- 角色用锤子敲击
- 有小火花/碎片效果
- 周期约 500ms

#### 11. `clyde-working-conducting.svg` — 指挥（2+ 个子代理）
- 角色双手交替挥动（指挥棒动作）
- 身体左右摇摆
- 周期约 1s

#### 12. `clyde-working-sweeping.svg` — 扫地（上下文压缩）
- 角色手持扫帚左右扫动
- 有小灰尘粒子效果
- 周期约 1.5s

#### 13. `clyde-working-carrying.svg` — 搬箱子（创建 worktree）
- 角色抱着一个小箱子行走
- 身体左右摇晃
- 周期约 1s

#### 14. `clyde-working-debugger.svg` — 调试
- 角色手持放大镜，仔细观察
- 放大镜有微弱反光效果
- 周期约 2s

#### 15. `clyde-working-ultrathink.svg` — 深度思考
- 角色闭眼，头顶有蒸汽/光环效果
- 身体完全静止
- 头顶特效循环

### 分组 C：睡眠序列（6 个）

#### 16. `clyde-idle-yawn.svg` — 打哈欠
- 角色张嘴打哈欠，双手举起伸懒腰
- 持续约 3 秒（由代码控制时长）

#### 17. `clyde-idle-doze.svg` — 打盹
- 角色头部慢慢低下
- 身体微微前倾
- 持续约 4 秒

#### 18. `clyde-idle-collapse.svg` — 倒下
- 角色身体向前倒下
- 有轻微弹跳效果
- 持续约 3 秒

#### 19. `clyde-collapse-sleep.svg` — 倒下过渡
- 从倒下姿势过渡到睡觉姿势
- 平滑动画

#### 20. `clyde-sleeping.svg` — 睡觉
- 角色趴在地上睡觉
- 有 "Zzz" 气泡从头顶飘出
- 周期约 3s

#### 21. `clyde-wake.svg` — 醒来
- 角色从睡觉姿势坐起
- 有伸懒腰动作
- 持续约 1.5 秒

### 分组 D：反应动画（4 个）

#### 22. `clyde-react-double.svg` — 戳一下（单击/双击反应）
- 角色被戳后身体向后弹一下
- 眼睛睁大
- 持续约 500-800ms

#### 23. `clyde-react-drag.svg` — 拖拽反应
- 角色身体被拉伸（四肢张开）
- 表情：惊讶
- 持续显示直到拖拽结束

#### 24. `clyde-react-left.svg` — 向左看
- 角色头部转向左边
- 眼睛看向左方

#### 25. `clyde-react-right.svg` — 向右看
- 角色头部转向右边
- 眼睛看向右方

### 分组 E：特殊状态（2 个）

#### 26. `clyde-happy.svg` — 任务完成开心
- 角色跳跃庆祝
- 有小星星/彩带效果
- 持续约 1.5 秒

#### 27. `clyde-working-wizard.svg` — 工作中（巫师主题）
- 角色手持魔杖，有魔法粒子效果
- 替代打字动画的变体

### 分组 F：极简模式（8 个）

极简模式下角色只露出半身，贴在屏幕边缘。

#### 28. `clyde-mini-idle.svg` — 极简待机
- 半身角色，眼睛可移动
- 贴在屏幕左/右边缘

#### 29. `clyde-mini-peek.svg` — 探头
- 角色从边缘探出头来
- 表情：好奇

#### 30. `clyde-mini-enter.svg` — 进入极简
- 角色从全屏滑入边缘
- 过渡动画

#### 31. `clyde-mini-enter-sleep.svg` — 进入极简并睡觉
- 滑入边缘后闭眼睡觉

#### 32. `clyde-mini-sleep.svg` — 极简睡觉
- 半身角色闭眼睡觉
- 有 "Zzz" 效果

#### 33. `clyde-mini-alert.svg` — 极简警报
- 半身角色睁大眼睛
- 表情：警觉/惊讶

#### 34. `clyde-mini-happy.svg` — 极简开心
- 半身角色微笑/庆祝

#### 35. `clyde-mini-crabwalk.svg` — 极简横移
- 角色沿屏幕边缘横向移动
- 周期约 1s

---

## 四、动画技术规范

### CSS Animation 要求
```xml
<defs>
  <style>
    /* 使用 @keyframes 定义动画 */
    @keyframes breathe {
      0%, 100% { transform: translateY(0); }
      50% { transform: translateY(-1px); }
    }

    /* 应用到具体元素 */
    #body-js {
      animation: breathe 2s ease-in-out infinite;
      transform-origin: 7.5px 13px; /* 角色中心点 */
    }
  </style>
</defs>
```

### 动画时长参考
| 类型 | 建议时长 |
|------|---------|
| 呼吸/待机 | 2-3s 循环 |
| 工作动作 | 300ms-1s 循环 |
| 反应动画 | 500ms-1.5s 一次 |
| 睡眠序列 | 由代码控制，SVG 内定义过渡 |

### 像素画技巧
- 使用 `<rect>` 元素绘制像素（每个像素 1x1 单位）
- 角色尺寸约 15x20 像素（在 45x45 的 viewBox 中）
- 使用有限调色板（8-16 色）
- 边缘使用 1px 深色描边增加辨识度

---

## 五、图标文件

### 应用图标
| 文件 | 尺寸 | 说明 |
|------|------|------|
| `assets/icon.ico` | 多尺寸（16/32/48/256） | Windows 应用图标 |
| `assets/icon.png` | 512x512 | 通用图标 |
| `assets/icons/16x16.png` | 16x16 | 小图标 |
| `assets/icons/32x32.png` | 32x32 | 中图标 |
| `assets/icons/48x48.png` | 48x48 | 中图标 |
| `assets/icons/64x64.png` | 64x64 | 中图标 |
| `assets/icons/128x128.png` | 128x128 | 大图标 |
| `assets/icons/256x256.png` | 256x256 | 大图标 |
| `assets/icons/512x512.png` | 512x512 | 超大图标 |

### 托盘图标
| 文件 | 尺寸 | 说明 |
|------|------|------|
| `assets/tray-icon.png` | 16x16 | 系统托盘图标（彩色） |
| `assets/tray-iconTemplate.png` | 16x16 | macOS 托盘模板（黑白，系统自动着色） |
| `assets/tray-iconTemplate@2x.png` | 32x32 | macOS 高分屏托盘模板 |

### 图标要求
- 风格与 SVG 角色一致
- 背景透明（PNG）或纯色（ICO）
- ICO 文件需包含 16/32/48/256 四种尺寸

---

## 六、GIF 预览图（可选）

`assets/gif/` 目录下的 GIF 文件用于 README 展示，不影响运行。如需更新：

| 文件 | 说明 |
|------|------|
| `clawd-idle.gif` | 待机状态预览 |
| `clawd-thinking.gif` | 思考状态预览 |
| `clawd-typing.gif` | 打字状态预览 |
| `clawd-building.gif` | 建造状态预览 |
| `clawd-juggling.gif` | 杂耍状态预览 |
| `clawd-conducting.gif` | 指挥状态预览 |
| `clawd-error.gif` | 错误状态预览 |
| `clawd-happy.gif` | 开心状态预览 |
| `clawd-notification.gif` | 惊跳状态预览 |
| `clawd-sweeping.gif` | 扫地状态预览 |
| `clawd-sleeping.gif` | 睡觉状态预览 |
| `clawd-carrying.gif` | 搬箱子状态预览 |
| `clawd-mini-*.gif` | 极简模式各状态预览 |

尺寸建议：80x80 像素，帧率 10-15fps，循环播放。

---

## 七、交付清单

提交时请按以下目录结构组织：

```
替换素材/
├── svg/                    # 35 个 SVG 动画文件
│   ├── clyde-idle-follow.svg
│   ├── clyde-idle-living.svg
│   ├── ... (共 35 个)
│   └── clyde-mini-crabwalk.svg
├── icons/                  # 图标文件
│   ├── icon.ico
│   ├── icon.png
│   ├── 16x16.png
│   ├── 32x32.png
│   ├── 48x48.png
│   ├── 64x64.png
│   ├── 128x128.png
│   ├── 256x256.png
│   └── 512x512.png
├── tray/                   # 托盘图标
│   ├── tray-icon.png
│   ├── tray-iconTemplate.png
│   └── tray-iconTemplate@2x.png
└── gif/                    # 预览 GIF（可选）
    ├── clawd-idle.gif
    └── ...
```

替换方式：将 `svg/` 目录中的文件复制到项目的 `assets/svg/` 覆盖原文件；图标和托盘图标同理。

---

## 八、验收标准

- [ ] 35 个 SVG 文件全部提供，文件名与原文件一致
- [ ] 每个 SVG 包含 `#eyes-js`、`#body-js`、`#shadow-js` 三个 ID 元素
- [ ] viewBox 为 `-15 -25 45 45`
- [ ] 动画流畅，无卡顿
- [ ] 像素风格统一，配色协调
- [ ] 极简模式（mini-*) 角色只显示半身
- [ ] 图标文件尺寸正确，背景透明
- [ ] 替换后 `npm start` 能正常运行，所有动画状态正确显示
