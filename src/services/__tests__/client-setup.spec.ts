import { describe, expect, it, vi } from 'vitest'

import {
  clearClientConfig,
  configureClient,
  filterApiKeysForClient,
  isClientConfiguredForSelectedKey,
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
      configuredKey: null,
      configuredKeys: [],
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
    await expect(configureClient('codex', 'sk-test', null, false, invoke)).resolves.toBeNull()
    await expect(clearClientConfig('codex', null, false, invoke)).resolves.toBeNull()
    expect(invoke).not.toHaveBeenCalled()
  })

  it('loads desktop client setup status through Tauri', async () => {
    const invoke = vi.fn().mockResolvedValue([
      ...status,
      {
        client: 'opencode',
        name: 'OpenCode',
        installed: true,
        configured: false,
        configuredKey: null,
        configuredKeys: [],
        command: 'opencode',
        configPath: '/Users/demo/.config/opencode/opencode.json',
        note: '历史客户端不应再出现在一键配置页面。',
      },
    ])

    await expect(loadClientSetupStatus(true, invoke)).resolves.toEqual(status)
    expect(invoke).toHaveBeenCalledWith('client_setup_status')
  })

  it('configures a desktop client through Tauri', async () => {
    const invoke = vi.fn().mockResolvedValue(result)

    await expect(configureClient('codex', 'sk-test', 'openai', true, invoke)).resolves.toEqual(result)
    expect(invoke).toHaveBeenCalledWith('configure_client', {
      client: 'codex',
      apiKey: 'sk-test',
      keyPlatform: 'openai',
    })
  })

  it('clears a desktop client config through Tauri', async () => {
    const invoke = vi.fn().mockResolvedValue(result)

    await expect(clearClientConfig('codex', null, true, invoke)).resolves.toEqual(result)
    expect(invoke).toHaveBeenCalledWith('clear_client_config', {
      client: 'codex',
      apiKey: null,
    })
  })

  it('restricts Codex keys to OpenAI platform', () => {
    const keys = [
      { id: 1, key: 'openai-key', status: 'active', group: { platform: 'openai' } },
      { id: 2, key: 'gemini-key', status: 'active', group: { platform: 'gemini' } },
      { id: 3, key: 'inactive-gemini-key', status: 'inactive', group: { platform: 'gemini' } },
      { id: 4, key: 'anthropic-key', status: 'active', group: { platform: 'anthropic' } },
    ] as any

    expect(requiredPlatformForClient('codex')).toBe('openai')
    expect(filterApiKeysForClient(keys, 'codex').map((key) => key.key)).toEqual(['openai-key'])
  })

  it('treats a client as configured only when the selected key matches the stored key', () => {
    const configuredStatus: ClientSetupStatus = {
      ...status[0],
      configured: true,
      configuredKey: 'configured-key',
      configuredKeys: ['configured-key'],
    }

    expect(isClientConfiguredForSelectedKey(configuredStatus, 'configured-key')).toBe(true)
    expect(isClientConfiguredForSelectedKey(configuredStatus, 'other-key')).toBe(false)
    expect(isClientConfiguredForSelectedKey(configuredStatus, '')).toBe(false)
  })
})
