type DesktopScope = {
  __TAURI_INTERNALS__?: unknown
}

export function isTauriDesktop(scope: DesktopScope | undefined = globalThis as DesktopScope): boolean {
  return Boolean(scope?.__TAURI_INTERNALS__)
}

export function openExternalUrl(url: string) {
  if (isTauriDesktop()) {
    import('@tauri-apps/plugin-shell').then(({ open }) => {
      open(url).catch(e => {
        console.error('Failed to open external URL in Tauri:', e)
        window.open(url, '_blank')
      })
    })
  } else {
    window.open(url, '_blank')
  }
}
