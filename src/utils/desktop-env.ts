type DesktopScope = {
  __TAURI_INTERNALS__?: unknown
}

export function isTauriDesktop(scope: DesktopScope | undefined = globalThis as DesktopScope): boolean {
  return Boolean(scope?.__TAURI_INTERNALS__)
}
