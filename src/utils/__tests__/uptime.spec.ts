import { describe, expect, it } from 'vitest'

import { getStableUptimeDays } from '../uptime'

describe('stable uptime', () => {
  it('counts days from 2026-04-28 Beijing time', () => {
    expect(getStableUptimeDays(new Date('2026-04-28T00:00:00+08:00'))).toBe(0)
    expect(getStableUptimeDays(new Date('2026-04-29T00:00:00+08:00'))).toBe(1)
    expect(getStableUptimeDays(new Date('2026-06-27T00:00:00+08:00'))).toBe(60)
  })

  it('does not show negative days before the stable start date', () => {
    expect(getStableUptimeDays(new Date('2026-04-27T23:59:59+08:00'))).toBe(0)
  })
})
