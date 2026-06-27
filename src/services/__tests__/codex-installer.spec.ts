import { describe, expect, it, vi } from 'vitest'

import { installCodex, loadCodexInstallStatus, type CodexInstallStatus } from '../codex-installer'

describe('codex installer service', () => {
  const status: CodexInstallStatus = {
    installed: false,
    platform: 'macos',
    architecture: 'arm64',
    installUrl: 'https://codexapp.agentsmirror.com/latest/mac-arm64',
    stagedPath: null,
    installedPath: null,
    version: null,
    verified: false,
    note: '将读取 agentsmirror 的 Sparkle appcast, 下载校验后安装 Codex.app。',
  }

  it('does nothing on web builds', async () => {
    const invoke = vi.fn()

    await expect(loadCodexInstallStatus(false, invoke)).resolves.toBeNull()
    await expect(installCodex(false, invoke)).resolves.toBeNull()
    expect(invoke).not.toHaveBeenCalled()
  })

  it('loads desktop Codex install status through Tauri', async () => {
    const invoke = vi.fn().mockResolvedValue(status)

    await expect(loadCodexInstallStatus(true, invoke)).resolves.toEqual(status)
    expect(invoke).toHaveBeenCalledWith('codex_install_status')
  })

  it('installs Codex through Tauri', async () => {
    const invoke = vi.fn().mockResolvedValue(status)

    await expect(installCodex(true, invoke)).resolves.toEqual(status)
    expect(invoke).toHaveBeenCalledWith('install_codex')
  })
})
