# 一键配置页面 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 新增桌面端一键配置页面，替代 API 教程和 Codex 安装器入口，并把 Hapi API Key 写入 Gemini CLI、Codex、OpenCode、OpenClaw、Hermes 配置。

**Architecture:** 前端新增 `ClientSetupView.vue` 和 `client-setup.ts` 服务，Web 端只提示需要桌面端。Tauri 新增 `client_setup` Rust 模块，负责检测客户端、生成配置、非破坏式写入文件，并复用现有 Codex 安装命令。

**Tech Stack:** Vue 3、TypeScript、Tauri 2、Rust、serde_json、toml_edit、serde_yaml、json5。

## Global Constraints

- 始终使用简体中文回复。
- 不影响原 Web 网页功能，本机检测和写配置只在 Tauri 桌面端执行。
- Codex 配置必须保留现有登录态，不能整份覆盖 `auth.json`。
- API Key、密码、密钥和本地私有配置禁止提交到 Git。
- 默认 Hapi Base URL 固定为 `https://www.hapi666.com/api/v1`。
- 只有 Codex 提供 agentsmirror 一键安装。

---

### Task 1: Rust 配置核心模块

**Files:**
- Create: `src-tauri/src/client_setup.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/Cargo.toml`

**Interfaces:**
- Produces: `client_setup_status() -> Result<Vec<ClientSetupStatus>, String>`
- Produces: `configure_client(client: ClientSetupClient, api_key: String) -> Result<ClientConfigureResult, String>`
- Produces: `configure_client_at_paths(client, api_key, paths)` helper for tests.

- [x] 写失败测试：Codex auth 合并保留原登录字段，只更新 `OPENAI_API_KEY`。
- [x] 写失败测试：Codex TOML 保留未知配置并追加 `model_providers.hapi`。
- [x] 写失败测试：Gemini/OpenCode/OpenClaw/Hermes 生成目标配置结构。
- [x] 实现最小配置模块。
- [x] 运行 `cd src-tauri && cargo test client_setup`。

### Task 2: 前端服务与页面

**Files:**
- Create: `src/services/client-setup.ts`
- Create: `src/views/ClientSetupView.vue`
- Create: `src/services/__tests__/client-setup.spec.ts`
- Modify: `src/layout/UserLayout.vue`
- Modify: `src/router/index.ts`

**Interfaces:**
- Consumes: Tauri commands `client_setup_status`、`configure_client`、`codex_install_status`、`install_codex`。
- Produces: `/client-setup` 路由和 `一键配置` 菜单项。

- [x] 写失败测试：非桌面端服务返回空状态，不调用 invoke。
- [x] 写失败测试：桌面端服务正确调用 Tauri command。
- [x] 实现 `client-setup.ts`。
- [x] 新增 `ClientSetupView.vue`，加载 active API Key、展示客户端状态、配置按钮和 Codex 安装按钮。
- [x] 删除菜单中的 `API 教程` 与 `Codex 安装器`，新增 `一键配置`。
- [x] 删除对应路由入口，新增 `/client-setup`。

### Task 3: 验证与提交

**Files:**
- Verify all touched files.

- [ ] 运行 `npm run build`。
- [ ] 运行 `cd src-tauri && cargo test`。
- [ ] 运行 `git diff --check`。
- [ ] 检查 `git status --short`，确认不 stage `src/views/BillingView.vue`。
- [ ] 只 stage 本次一键配置相关文件。
- [ ] 提交清晰 commit。
