import { describe, expect, it, vi } from 'vitest'

import {
  clearSavedCredentials,
  loadSavedCredentials,
  saveCredentialsAfterLogin,
  type SavedCredentials
} from '../saved-credentials'

describe('saved credentials service', () => {
  it('does nothing on web builds', async () => {
    const invoke = vi.fn()

    expect(await loadSavedCredentials(false, invoke)).toBeNull()
    await saveCredentialsAfterLogin(false, invoke, true, {
      email: 'user@example.com',
      password: 'secret'
    })
    await clearSavedCredentials(false, invoke)

    expect(invoke).not.toHaveBeenCalled()
  })

  it('loads credentials from desktop local storage', async () => {
    const saved: SavedCredentials = {
      email: 'user@example.com',
      password: 'secret'
    }
    const invoke = vi.fn().mockResolvedValue(saved)

    await expect(loadSavedCredentials(true, invoke)).resolves.toEqual(saved)
    expect(invoke).toHaveBeenCalledWith('load_saved_credentials')
  })

  it('saves credentials only when remember password is enabled', async () => {
    const invoke = vi.fn().mockResolvedValue(undefined)

    await saveCredentialsAfterLogin(true, invoke, true, {
      email: 'user@example.com',
      password: 'secret'
    })

    expect(invoke).toHaveBeenCalledWith('save_saved_credentials', {
      credentials: {
        email: 'user@example.com',
        password: 'secret'
      }
    })
  })

  it('clears credentials when remember password is disabled', async () => {
    const invoke = vi.fn().mockResolvedValue(undefined)

    await saveCredentialsAfterLogin(true, invoke, false, {
      email: 'user@example.com',
      password: 'secret'
    })

    expect(invoke).toHaveBeenCalledWith('clear_saved_credentials')
  })
})
