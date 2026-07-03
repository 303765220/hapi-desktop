import { readFileSync } from 'node:fs'
import { fileURLToPath, URL } from 'node:url'

import { describe, expect, it } from 'vitest'

describe('available models view', () => {
  it('shows the configured domestic model cards', () => {
    const source = readFileSync(fileURLToPath(new URL('../ModelsView.vue', import.meta.url)), 'utf8')

    expect(source).toContain("name: 'DeepSeek-v4-pro'")
    expect(source).toContain("name: 'DeepSeek-v4-flash'")
    expect(source).toContain("name: 'glm-5.2'")
    expect(source).toContain("name: 'kimi-2.7'")
    expect(source).toContain("input_price: 1.4, output_price: 4.4, cache_read_price: 0.26")
    expect(source).toContain("input_price: 0.95, output_price: 4.0, cache_read_price: 0.19")
  })
})
