import { describe, expect, it } from 'vitest'

import { getApiKeyStatusLabel, getToggleApiKeyStatusAction } from '../api-key-status'

describe('api key status helpers', () => {
  it('labels user-visible API key states', () => {
    expect(getApiKeyStatusLabel('active')).toBe('正常')
    expect(getApiKeyStatusLabel('inactive')).toBe('已禁用')
    expect(getApiKeyStatusLabel('quota_exhausted')).toBe('额度耗尽')
    expect(getApiKeyStatusLabel('expired')).toBe('已过期')
  })

  it('disables active keys and enables every non-active state', () => {
    expect(getToggleApiKeyStatusAction('active')).toEqual({
      nextStatus: 'inactive',
      label: '禁用',
      title: '禁用密钥',
    })
    expect(getToggleApiKeyStatusAction('inactive')).toEqual({
      nextStatus: 'active',
      label: '启用',
      title: '启用密钥',
    })
  })
})
