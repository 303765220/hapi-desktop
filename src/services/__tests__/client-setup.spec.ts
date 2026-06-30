import { describe, expect, it, vi } from 'vitest'

import {
  configureClient,
  filterApiKeysForClient,
  loadClientSetupStatus,
  requiredPlatformForClient,
  type ClientConfigureResult,
  type ClientSetupStatus,
} from '../client-setup'

describe('client setup service', () => {
  const status: ClientSetupStatus[] = [
    {
      client: 'codex',
      name: 'Codex',
      installed: true,
      configured: false,
      command: 'codex',
      configPath: '/Users/demo/.codex/config.toml',
      note: '已检测到客户端，可写入 Hapi 配置。',
    },
  ]

  const result: ClientConfigureResult = {
    client: 'codex',
    configured: true,
    configPath: '/Users/demo/.codex/config.toml',
    backupPath: '/Users/demo/.codex/config.toml.hapi.bak',
    note: '已写入 Codex 的 Hapi 配置。',
  }

  it('does nothing on web builds', async () => {
    const invoke = vi.fn()

    await expect(loadClientSetupStatus(false, invoke)).resolves.toEqual([])
    await expect(configureClient('codex', 'sk-test', false, invoke)).resolves.toBeNull()
    expect(invoke).not.toHaveBeenCalled()
  })

  it('loads desktop client setup status through Tauri', async () => {
    const invoke = vi.fn().mockResolvedValue(status)

    await expect(loadClientSetupStatus(true, invoke)).resolves.toEqual(status)
    expect(invoke).toHaveBeenCalledWith('client_setup_status')
  })

  it('configures a desktop client through Tauri', async () => {
    const invoke = vi.fn().mockResolvedValue(result)

    await expect(configureClient('codex', 'sk-test', true, invoke)).resolves.toEqual(result)
    expect(invoke).toHaveBeenCalledWith('configure_client', {
      client: 'codex',
      apiKey: 'sk-test',
    })
  })

  it('restricts Codex and Gemini keys to their matching platforms', () => {
    const keys = [
      { id: 1, key: 'openai-key', status: 'active', group: { platform: 'openai' } },
      { id: 2, key: 'gemini-key', status: 'active', group: { platform: 'gemini' } },
      { id: 3, key: 'inactive-gemini-key', status: 'inactive', group: { platform: 'gemini' } },
      { id: 4, key: 'anthropic-key', status: 'active', group: { platform: 'anthropic' } },
    ] as any

    expect(requiredPlatformForClient('codex')).toBe('openai')
    expect(requiredPlatformForClient('geminiCli')).toBe('gemini')
    expect(requiredPlatformForClient('opencode')).toBeNull()
    expect(filterApiKeysForClient(keys, 'codex').map((key) => key.key)).toEqual(['openai-key'])
    expect(filterApiKeysForClient(keys, 'geminiCli').map((key) => key.key)).toEqual(['gemini-key'])
    expect(filterApiKeysForClient(keys, 'openclaw').map((key) => key.key)).toEqual([
      'openai-key',
      'gemini-key',
      'anthropic-key',
    ])
  })
})
