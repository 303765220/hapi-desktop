# 一键配置页面设计

## 目标

把原来的 `API 教程` 与 `Codex 安装器` 合并为桌面端 `一键配置` 页面。页面帮助用户把当前 Hapi API Key 写入本机常见 AI 编程客户端配置，Web 端不执行本机检测或文件写入。

## 范围

- 支持 Gemini CLI、Codex、OpenCode、OpenClaw、Hermes。
- 先检测本机是否安装或是否存在默认配置目录。
- 未检测到客户端时提示先安装。
- 只有 Codex 提供一键安装能力，复用既有安装器。
- 用户必须显式选择一个 `active` Hapi API Key 后才能写配置。
- Base URL 固定为 `https://www.hapi666.com/api/v1`。

## 配置策略

- Codex：非破坏式更新 `~/.codex/auth.json` 和 `~/.codex/config.toml`。保留现有登录态、OAuth token、未知字段和其它 provider，只新增或更新 Hapi provider，并切换当前 provider 到 Hapi。
- Gemini CLI：更新 `~/.gemini/.env`，写入 `GOOGLE_GEMINI_BASE_URL`、`GEMINI_API_KEY` 和默认模型。
- OpenCode：更新 `~/.config/opencode/opencode.json`，追加或更新 `provider.hapi`。
- OpenClaw：更新 `~/.openclaw/openclaw.json`，追加或更新 `models.providers.hapi`。
- Hermes：更新 `~/.hermes/config.yaml`，追加或更新 `custom_providers` 中的 `hapi` 项。

## 边界

- 不引入本地网关、后台进程或端口监听。
- 不自动创建 Hapi API Key。
- 不把本机密钥、密码、配置文件内容提交到 Git。
- 写配置只在 Tauri 桌面端可用；Web 端只显示提示。

## 验证

- Rust 单测覆盖配置生成、Codex auth 合并、TOML/JSON/YAML 写入。
- 前端单测覆盖桌面端 invoke 包装。
- `npm run build` 验证 Web/Tauri 前端构建。
- `cargo test` 验证 Tauri 配置模块。
