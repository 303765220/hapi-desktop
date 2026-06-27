import { describe, expect, it } from 'vitest'

import { isTauriDesktop } from '../desktop-env'

describe('isTauriDesktop', () => {
  it('detects Tauri only from the explicit global marker', () => {
    expect(isTauriDesktop({ __TAURI_INTERNALS__: {} })).toBe(true)
    expect(isTauriDesktop({})).toBe(false)
    expect(isTauriDesktop(undefined)).toBe(false)
  })
})
