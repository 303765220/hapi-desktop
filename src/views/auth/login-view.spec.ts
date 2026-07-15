import { readFileSync } from 'node:fs'
import { fileURLToPath, URL } from 'node:url'

import { describe, expect, it } from 'vitest'

describe('login view', () => {
  it('links users to the registration page', () => {
    const source = readFileSync(fileURLToPath(new URL('./LoginView.vue', import.meta.url)), 'utf8')

    expect(source).toContain('还没有账号？')
    expect(source).toContain('to="/register"')
    expect(source).toContain('立即注册')
  })
})
