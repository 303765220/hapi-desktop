import { isTauriDesktop } from '@/utils/desktop-env'

export type ClientSetupClient = 'geminiCli' | 'codex' | 'opencode' | 'openclaw' | 'hermes'

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
  desktop = isTauriDesktop(),
  invoke: InvokeFn = defaultInvoke
): Promise<ClientConfigureResult | null> {
  if (!desktop) {
    return null
  }

  return invoke<ClientConfigureResult>('configure_client', {
    client,
    apiKey,
  })
}
