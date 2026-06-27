import type { ApiKey } from '@/types'

export type ApiKeyStatus = ApiKey['status']

export function getApiKeyStatusLabel(status: ApiKeyStatus): string {
  if (status === 'active') {
    return '正常'
  }
  if (status === 'inactive') {
    return '已禁用'
  }
  if (status === 'quota_exhausted') {
    return '额度耗尽'
  }
  return '已过期'
}

export function getToggleApiKeyStatusAction(status: ApiKeyStatus): {
  nextStatus: 'active' | 'inactive'
  label: string
  title: string
} {
  if (status === 'active') {
    return {
      nextStatus: 'inactive',
      label: '禁用',
      title: '禁用密钥',
    }
  }

  return {
    nextStatus: 'active',
    label: '启用',
    title: '启用密钥',
  }
}
