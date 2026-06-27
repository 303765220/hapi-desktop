import { isTauriDesktop } from '@/utils/desktop-env'

export interface SavedCredentials {
  email: string
  password: string
}

type InvokeFn = <T>(command: string, args?: Record<string, unknown>) => Promise<T>

async function defaultInvoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  const { invoke } = await import('@tauri-apps/api/core')
  return invoke<T>(command, args)
}

export async function loadSavedCredentials(
  desktop = isTauriDesktop(),
  invoke: InvokeFn = defaultInvoke
): Promise<SavedCredentials | null> {
  if (!desktop) {
    return null
  }

  return invoke<SavedCredentials | null>('load_saved_credentials')
}

export async function saveCredentialsAfterLogin(
  desktop = isTauriDesktop(),
  invoke: InvokeFn = defaultInvoke,
  rememberPassword: boolean,
  credentials: SavedCredentials
): Promise<void> {
  if (!desktop) {
    return
  }

  if (!rememberPassword) {
    await clearSavedCredentials(desktop, invoke)
    return
  }

  await invoke<void>('save_saved_credentials', { credentials })
}

export async function clearSavedCredentials(
  desktop = isTauriDesktop(),
  invoke: InvokeFn = defaultInvoke
): Promise<void> {
  if (!desktop) {
    return
  }

  await invoke<void>('clear_saved_credentials')
}
