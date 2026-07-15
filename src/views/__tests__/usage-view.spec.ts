import { readFileSync } from 'node:fs'
import { fileURLToPath, URL } from 'node:url'

import { describe, expect, it } from 'vitest'

describe('usage view', () => {
  it('includes cache tokens in each usage log token breakdown', () => {
    const source = readFileSync(fileURLToPath(new URL('../UsageView.vue', import.meta.url)), 'utf8')

    expect(source).toContain('getUsageLogTotalTokens(log)')
    expect(source).toContain('log.cache_read_tokens')
    expect(source).toContain('log.cache_creation_tokens')
    expect(source).toContain('log.cache_creation_5m_tokens')
    expect(source).toContain('log.cache_creation_1h_tokens')
    expect(source).toContain('hasCacheReadTokens(log)')
    expect(source).toContain('hasAggregateCacheCreationTokens(log)')
    expect(source).toContain('5m')
    expect(source).toContain('1h')
    expect(source).toContain('formatCacheTokenCount(log.cache_read_tokens)')
    expect(source).not.toContain('缓存命中 {{')
    expect(source).not.toContain('缓存写入 {{')
    expect(source).not.toContain('缓存写入 5m {{')
    expect(source).not.toContain('缓存写入 1h {{')
  })

  it('shows token details before the smaller total line', () => {
    const source = readFileSync(fileURLToPath(new URL('../UsageView.vue', import.meta.url)), 'utf8')
    const detailsIndex = source.indexOf('class="text-sm text-zinc-500 font-mono flex flex-wrap items-center gap-x-2 gap-y-1"')
    const totalIndex = source.indexOf('class="mt-1 text-xs text-zinc-500 font-mono"')

    expect(detailsIndex).toBeGreaterThan(-1)
    expect(totalIndex).toBeGreaterThan(-1)
    expect(detailsIndex).toBeLessThan(totalIndex)
  })
})
