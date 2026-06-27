import { isTauriDesktop } from '@/utils/desktop-env'

const WEB_API_BASE_URL = '/api/v1'
const DESKTOP_API_BASE_URL = 'https://www.hapi666.com/api/v1'

type DesktopScope = Parameters<typeof isTauriDesktop>[0]

export function resolveApiBaseUrl(
  explicitBaseUrl: string | undefined = import.meta.env.VITE_API_BASE_URL,
  scope: DesktopScope = globalThis as DesktopScope
): string {
  const trimmed = explicitBaseUrl?.trim()
  if (trimmed) {
    return trimmed
  }

  return isTauriDesktop(scope) ? DESKTOP_API_BASE_URL : WEB_API_BASE_URL
}
