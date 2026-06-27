import { describe, expect, it } from 'vitest'

import { resolveApiBaseUrl } from '../base-url'

describe('resolveApiBaseUrl', () => {
  it('uses explicit environment value first', () => {
    expect(resolveApiBaseUrl('https://api.example.com/api/v1', { __TAURI_INTERNALS__: {} })).toBe(
      'https://api.example.com/api/v1'
    )
  })

  it('keeps the browser default on the web so Vite proxy and existing deploys still work', () => {
    expect(resolveApiBaseUrl('', {})).toBe('/api/v1')
  })

  it('uses the Hapi production API in the Tauri desktop shell', () => {
    expect(resolveApiBaseUrl('', { __TAURI_INTERNALS__: {} })).toBe(
      'https://www.hapi666.com/api/v1'
    )
  })
})
