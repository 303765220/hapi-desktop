import { readFileSync } from 'node:fs'
import { fileURLToPath, URL } from 'node:url'

import { describe, expect, it } from 'vitest'

describe('image creation view', () => {
  it('embeds the local infinite canvas with Hapi API key configuration', () => {
    const source = readFileSync(fileURLToPath(new URL('../ImageCreationView.vue', import.meta.url)), 'utf8')

    expect(source).toContain('http://127.0.0.1:3000/')
    expect(source).toContain('https://www.hapi666.com/v1')
    expect(source).toContain('apiKey')
    expect(source).toContain('getKeysList')
  })
})
