import { isTauriDesktop } from '@/utils/desktop-env'
import type { ApiKey, GroupPlatform } from '@/types'

export type ClientSetupClient = 'geminiCli' | 'codex' | 'opencode' | 'openclaw' | 'hermes'
type ClientKeyPlatform = Extract<GroupPlatform, 'openai' | 'gemini'>

export interface ClientSetupStatus {
  client: ClientSetupClient
  name: string
  installed: boolean
  configured: boolean
  command: string
  configPath: string
  note: string
}

export interface ClientConfigureResult {
  client: ClientSetupClient
  configured: boolean
  configPath: string
  backupPath: string | null
  note: string
}

type InvokeFn = <T>(command: string, args?: Record<string, unknown>) => Promise<T>

async function defaultInvoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  const { invoke } = await import('@tauri-apps/api/core')
  return invoke<T>(command, args)
}

export async function loadClientSetupStatus(
  desktop = isTauriDesktop(),
  invoke: InvokeFn = defaultInvoke
): Promise<ClientSetupStatus[]> {
  if (!desktop) {
    return []
  }

  return invoke<ClientSetupStatus[]>('client_setup_status')
}

export async function configureClient(
  client: ClientSetupClient,
  apiKey: string,
  keyPlatform: string | null = null,
  desktop = isTauriDesktop(),
  invoke: InvokeFn = defaultInvoke
): Promise<ClientConfigureResult | null> {
  if (!desktop) {
    return null
  }

  return invoke<ClientConfigureResult>('configure_client', {
    client,
    apiKey,
    keyPlatform,
  })
}

export async function clearClientConfig(
  client: ClientSetupClient,
  desktop = isTauriDesktop(),
  invoke: InvokeFn = defaultInvoke
): Promise<ClientConfigureResult | null> {
  if (!desktop) {
    return null
  }

  return invoke<ClientConfigureResult>('clear_client_config', {
    client,
  })
}

export function requiredPlatformForClient(client: ClientSetupClient): ClientKeyPlatform | null {
  if (client === 'codex') {
    return 'openai'
  }
  if (client === 'geminiCli') {
    return 'gemini'
  }
  return null
}

export function filterApiKeysForClient(keys: ApiKey[], client: ClientSetupClient): ApiKey[] {
  const platform = requiredPlatformForClient(client)
  return keys.filter((key) => {
    if (key.status !== 'active') {
      return false
    }
    if (!platform) {
      return true
    }
    return key.group?.platform === platform
  })
}
