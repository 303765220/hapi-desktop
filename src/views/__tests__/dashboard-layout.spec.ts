import { readFileSync } from 'node:fs'
import { fileURLToPath, URL } from 'node:url'

import { describe, expect, it } from 'vitest'

describe('dashboard layout', () => {
  it('lets distribution cards span the full dashboard grid width', () => {
    const source = readFileSync(fileURLToPath(new URL('../DashboardView.vue', import.meta.url)), 'utf8')

    expect(source).toContain('lg:col-span-3 grid grid-cols-1 xl:grid-cols-2')
  })

  it('uses total_tokens for group distribution charts', () => {
    const source = readFileSync(fileURLToPath(new URL('../DashboardView.vue', import.meta.url)), 'utf8')

    expect(source).toContain('groupTokenTotal')
    expect(source).toContain('formatNumberKMB(groupTokenTotal(group))')
    expect(source).toContain("groupSortMode.value === 'token' ? groupTokenTotal(g) : g.actual_cost")
  })

  it('does not abbreviate request counts in distribution tables', () => {
    const source = readFileSync(fileURLToPath(new URL('../DashboardView.vue', import.meta.url)), 'utf8')

    expect(source).toContain('formatRequestCount(model.requests)')
    expect(source).toContain('formatRequestCount(group.requests)')
    expect(source).not.toContain('formatNumberKMB(model.requests)')
    expect(source).not.toContain('formatNumberKMB(group.requests)')
  })

  it('keeps dashboard trend date params in the backend date-only contract', () => {
    const source = readFileSync(fileURLToPath(new URL('../DashboardView.vue', import.meta.url)), 'utf8')

    expect(source).toContain('start_date: formatDate(startDate),')
    expect(source).toContain('end_date: formatDate(now),')
    expect(source).not.toContain('formatDate(startDate, range ===')
    expect(source).not.toContain('formatDate(now, range ===')
  })

  it('uses the shared polished distribution table style', () => {
    const source = readFileSync(fileURLToPath(new URL('../DashboardView.vue', import.meta.url)), 'utf8')

    expect(source.match(/distribution-table-shell/g)?.length).toBeGreaterThanOrEqual(3)
    expect(source.match(/class="distribution-table"/g)?.length).toBe(2)
    expect(source).toContain('class="distribution-name"')
    expect(source).toContain('class="distribution-cost text-right"')
  })
})
