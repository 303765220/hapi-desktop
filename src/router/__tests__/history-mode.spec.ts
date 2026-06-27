import { beforeEach, describe, expect, it, vi } from 'vitest'

import { createAppHistory } from '../history'
import { createWebHashHistory, createWebHistory } from 'vue-router'

vi.mock('vue-router', () => ({
  createWebHashHistory: vi.fn(() => ({ mode: 'hash' })),
  createWebHistory: vi.fn(() => ({ mode: 'web' }))
}))

describe('createAppHistory', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('uses hash history inside the Tauri desktop shell', () => {
    const history = createAppHistory({ __TAURI_INTERNALS__: {} })

    expect(history).toEqual({ mode: 'hash' })
    expect(createWebHashHistory).toHaveBeenCalledOnce()
    expect(createWebHistory).not.toHaveBeenCalled()
  })

  it('keeps web history for browser builds', () => {
    const history = createAppHistory({})

    expect(history).toEqual({ mode: 'web' })
    expect(createWebHistory).toHaveBeenCalledOnce()
    expect(createWebHashHistory).not.toHaveBeenCalled()
  })
})
