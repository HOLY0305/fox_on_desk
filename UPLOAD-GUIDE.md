# 项目上传操作报告

> 本文档指导如何将基于 Clyde 二次开发的项目合法合规地发布到 GitHub。

---

## 一、许可证分析

| 项目 | 许可证 | 说明 |
|------|--------|------|
| **clawd-on-desk**（原项目） | AGPL-3.0 | 代码开源，角色美术版权归 Anthropic |
| **Clyde**（QingJ01 的 fork） | AGPL-3.0 | 继承 AGPL-3.0，NOTICE 中保留了原项目授权声明 |
| **你的二次修改** | 需要保持 AGPL-3.0 | AGPL-3.0 要求衍生作品必须使用相同许可证 |

### 许可证继承链

```
clawd-on-desk (rullerzhou-afk, AGPL-3.0)
  └── Clyde (QingJ01, AGPL-3.0)
        └── 你的项目 (必须 AGPL-3.0)
```

---

## 二、AGPL-3.0 核心要求

1. **必须保持 AGPL-3.0 许可证** — 不能改为 MIT、Apache 等
2. **必须保留原始版权声明和许可证文本**
3. **必须注明修改内容** — 说明你改了什么
4. **源码必须公开** — 如果分发二进制，必须同时提供源码
5. **网络使用也受约束** — 如果通过网络提供服务，用户有权获取源码

---

## 三、必须保留的文件

这些文件**必须原样保留**，不能删除或修改：

| 文件 | 内容 |
|------|------|
| `LICENSE` | AGPL-3.0 许可证全文 |
| `NOTICE` | 版权声明 + clawd-on-desk 授权 + 原始贡献者列表 |

---

## 四、需要修改的文件

### 1. `NOTICE` — 添加你的版权声明

在现有内容**前面**加上你的信息：

```
你的项目名
Copyright (c) 2026 你的名字/GitHub用户名

This project is derived from Clyde on Desk:
  https://github.com/QingJ01/Clyde

Clyde on Desk is licensed under AGPL-3.0.

---
```

### 2. `README.md` — 更新项目说明

保留原有的致谢段落（Forked from / Credits），并加上你自己的修改说明。

### 3. `package.json` / `Cargo.toml` — 可以改项目名和版本

```json
{
  "name": "你的项目名",
  "version": "0.1.0"
}
```

---

## 五、不能做的事

| 禁止事项 | 原因 |
|---------|------|
| 删除 `LICENSE` 文件 | AGPL-3.0 要求 |
| 删除 `NOTICE` 中的原始版权声明 | 违反许可证 |
| 改为非开源许可证 | AGPL-3.0 衍生作品必须保持同许可证 |
| 声称代码是你原创的 | 必须注明上游来源 |
| 使用 "Clawd" 角色形象做商业用途 | 美术版权归 Anthropic |

---

## 六、操作步骤

### 步骤 1：修改 NOTICE 文件

在 `NOTICE` 文件顶部添加你的版权声明（见第四节）。

### 步骤 2：修改 README.md

更新项目名称、描述、改动说明，保留原有致谢段落。

### 步骤 3：修改项目配置（可选）

更新 `package.json` 和 `src-tauri/Cargo.toml` 中的项目名和版本号。

### 步骤 4：重新配置 Git 远程仓库

```bash
cd "D:\project\github_project\claude on desk\Clyde"

# 将原仓库重命名为 upstream（保留上游引用）
git remote rename origin upstream

# 添加你的仓库作为新的 origin
git remote add origin https://github.com/你的用户名/你的仓库名.git

# 推送到你的仓库
git push -u origin main
```

### 步骤 5：验证

- 确认 GitHub 仓库页面显示正确的许可证（AGPL-3.0）
- 确认 `LICENSE` 和 `NOTICE` 文件存在且内容正确
- 确认 README 中有上游项目致谢

---

## 七、建议的 README 模板

```markdown
# 你的项目名

基于 [Clyde on Desk](https://github.com/QingJ01/Clyde) 的二次开发版本。

## 改动说明
- 添加了 XXX 功能
- 修复了 XXX 问题
- ...

## 致谢
- [Clyde on Desk](https://github.com/QingJ01/Clyde) by [@QingJ01](https://github.com/QingJ01)
- [Clawd on Desk](https://github.com/rullerzhou-afk/clawd-on-desk) by [@rullerzhou-afk](https://github.com/rullerzhou-afk)

## 许可证
本项目基于 AGPL-3.0 许可证开源。详见 [LICENSE](LICENSE)。
```

---

## 八、常见问题

### Q: 我可以把项目改为闭源吗？
A: 不可以。AGPL-3.0 要求衍生作品必须保持开源。

### Q: 我可以商用吗？
A: 代码可以商用（AGPL-3.0 允许），但角色美术形象（Clawd）版权归 Anthropic，不能用于商业用途。

### Q: 我需要保留原项目的提交历史吗？
A: 不是强制要求，但建议保留，方便追溯变更。

### Q: 我可以删除原项目的贡献者列表吗？
A: 不建议。保留贡献者列表是对上游工作的尊重，也是开源社区的惯例。

### Q: 如果我只修改了一小部分代码，也需要 AGPL-3.0 吗？
A: 是的。只要你的代码是基于 AGPL-3.0 项目的衍生作品，就必须使用 AGPL-3.0。
