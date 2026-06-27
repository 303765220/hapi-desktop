import type { RouteLocationNormalized } from 'vue-router'

export interface AuthGuardState {
  isAuthenticated: boolean
  isAdmin: boolean
}

export function resolveAuthRedirect(
  to: Pick<RouteLocationNormalized, 'matched' | 'fullPath' | 'path'>,
  auth: AuthGuardState
): string | null {
  const requiresAuth = to.matched.some((record) => record.meta.requiresAuth)
  if (!requiresAuth) {
    return null
  }

  if (!auth.isAuthenticated) {
    const redirect = to.fullPath || to.path || '/'
    return `/login?redirect=${encodeURIComponent(redirect)}`
  }

  const requiresAdmin = to.matched.some((record) => record.meta.requiresAdmin)
  if (requiresAdmin && !auth.isAdmin) {
    return '/'
  }

  return null
}
