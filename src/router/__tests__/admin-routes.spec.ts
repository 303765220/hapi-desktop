import { readFileSync } from 'node:fs'
import { fileURLToPath, URL } from 'node:url'

import { describe, expect, it } from 'vitest'

describe('application routes', () => {
  it('does not expose admin panel routes', () => {
    const source = readFileSync(fileURLToPath(new URL('../index.ts', import.meta.url)), 'utf8')

    expect(source).not.toContain("path: '/admin'")
  })

  it('exposes a public register route', () => {
    const source = readFileSync(fileURLToPath(new URL('../index.ts', import.meta.url)), 'utf8')

    expect(source).toContain("path: '/register'")
    expect(source).toContain("name: 'Register'")
    expect(source).toContain("@/views/auth/RegisterView.vue")
  })
})
