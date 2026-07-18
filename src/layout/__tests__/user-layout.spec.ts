import { readFileSync } from 'node:fs'
import { fileURLToPath, URL } from 'node:url'

import { describe, expect, it } from 'vitest'

describe('user layout navigation', () => {
  it('does not show channel status in the user sidebar', () => {
    const source = readFileSync(fileURLToPath(new URL('../UserLayout.vue', import.meta.url)), 'utf8')

    expect(source).not.toContain("name: '渠道状态'")
    expect(source).not.toContain("path: '/status'")
  })
})
