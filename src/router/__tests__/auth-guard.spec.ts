import { describe, expect, it } from 'vitest'
import type { RouteRecordNormalized } from 'vue-router'

import { resolveAuthRedirect } from '../auth-guard'

function route(fullPath: string, meta: RouteRecordNormalized['meta'] = {}) {
  return {
    path: fullPath.split('?')[0] || '/',
    fullPath,
    matched: [{ meta }] as RouteRecordNormalized[],
  }
}

describe('resolveAuthRedirect', () => {
  it('redirects unauthenticated users from protected pages to login', () => {
    expect(
      resolveAuthRedirect(route('/keys', { requiresAuth: true }), {
        isAuthenticated: false,
        isAdmin: false,
      })
    ).toBe('/login?redirect=%2Fkeys')
  })

  it('keeps public pages accessible without auth', () => {
    expect(
      resolveAuthRedirect(route('/login'), {
        isAuthenticated: false,
        isAdmin: false,
      })
    ).toBeNull()
  })

  it('redirects non-admin users away from admin pages', () => {
    expect(
      resolveAuthRedirect(route('/admin/users', { requiresAuth: true, requiresAdmin: true }), {
        isAuthenticated: true,
        isAdmin: false,
      })
    ).toBe('/')
  })
})
