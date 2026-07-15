import { readFileSync } from 'node:fs'
import { fileURLToPath, URL } from 'node:url'

import { describe, expect, it } from 'vitest'

describe('keys view', () => {
  it('shows the API endpoint address on the left side of the header', () => {
    const source = readFileSync(fileURLToPath(new URL('../KeysView.vue', import.meta.url)), 'utf8')

    expect(source).toContain('API 端点地址')
    expect(source).toContain('https://www.hapi666.com/')
    expect(source).toContain('copyToClipboard(API_ENDPOINT_URL)')
  })

  it('adds CC Switch import operation for each API key', () => {
    const source = readFileSync(fileURLToPath(new URL('../KeysView.vue', import.meta.url)), 'utf8')

    expect(source).toContain('handleImportToCcSwitch(item)')
    expect(source).toContain('buildCcSwitchImportDeeplink')
    expect(source).toContain('showCcsClientSelect')
    expect(source).toContain('CC_SWITCH_BASE_URL')
  })

  it('adds a usage column with today and last 30 days costs', () => {
    const source = readFileSync(fileURLToPath(new URL('../KeysView.vue', import.meta.url)), 'utf8')

    expect(source).toContain('>用量<')
    expect(source).toContain('今日:')
    expect(source).toContain('近30天:')
    expect(source).toContain('getDashboardApiKeysUsage')
    expect(source).toContain('usageStats[item.id]?.today_actual_cost')
    expect(source).toContain('usageStats[item.id]?.total_actual_cost')
  })
})
