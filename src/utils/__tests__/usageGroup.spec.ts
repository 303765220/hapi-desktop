import { describe, expect, it } from 'vitest'

import { formatUsageGroup } from '../usageGroup'

describe('formatUsageGroup', () => {
  it('prefers the loaded group name', () => {
    expect(formatUsageGroup({ group_id: 7, group: { name: 'Claude 专用组' } })).toBe('Claude 专用组')
  })

  it('falls back to group id when group name is not loaded', () => {
    expect(formatUsageGroup({ group_id: 7, group: null })).toBe('#7')
  })

  it('shows a dash when usage has no group', () => {
    expect(formatUsageGroup({ group_id: null, group: null })).toBe('-')
  })
})
