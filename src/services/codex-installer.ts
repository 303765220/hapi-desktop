import { isTauriDesktop } from '@/utils/desktop-env'

export interface CodexInstallStatus {
  installed: boolean
  platform: string
  architecture: string
  installUrl: string
  stagedPath: string | null
  installedPath: string | null
  version: string | null
  verified: boolean
  note: string
}

type InvokeFn = <T>(command: string, args?: Record<string, unknown>) => Promise<T>

async function defaultInvoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  const { invoke } = await import('@tauri-apps/api/core')
  return invoke<T>(command, args)
}

export async function loadCodexInstallStatus(
  desktop = isTauriDesktop(),
  invoke: InvokeFn = defaultInvoke
): Promise<CodexInstallStatus | null> {
  if (!desktop) {
    return null
  }

  return invoke<CodexInstallStatus>('codex_install_status')
}

export async function installCodex(
  desktop = isTauriDesktop(),
  invoke: InvokeFn = defaultInvoke
): Promise<CodexInstallStatus | null> {
  if (!desktop) {
    return null
  }

  return invoke<CodexInstallStatus>('install_codex')
}
