import { describe, expect, it, vi } from 'vitest'

async function importFreshClient(desktop: boolean) {
  vi.resetModules()
  const store = new Map<string, string>()
  Object.defineProperty(globalThis, 'localStorage', {
    value: {
      getItem: (key: string) => store.get(key) ?? null,
      setItem: (key: string, value: string) => store.set(key, value),
      removeItem: (key: string) => store.delete(key),
    },
    configurable: true,
  })
  if (desktop) {
    ;(globalThis as { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__ = {}
  } else {
    delete (globalThis as { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__
  }

  const { apiClient } = await import('../client')
  return apiClient
}

describe('apiClient desktop HTTP transport', () => {
  it('uses the browser axios adapter on the web', async () => {
    const client = await importFreshClient(false)

    expect(typeof client.defaults.adapter).not.toBe('function')
  })

  it('uses a native Tauri HTTP adapter in the desktop shell', async () => {
    const client = await importFreshClient(true)

    expect(typeof client.defaults.adapter).toBe('function')
  })
})
