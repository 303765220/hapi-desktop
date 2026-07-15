import { readFileSync } from 'node:fs'
import { fileURLToPath, URL } from 'node:url'

import { describe, expect, it } from 'vitest'

describe('client setup view', () => {
  it('removes the top intro panel while keeping setup actions', () => {
    const source = readFileSync(fileURLToPath(new URL('../ClientSetupView.vue', import.meta.url)), 'utf8')

    expect(source).not.toContain('告别繁琐的手动复制粘贴')
    expect(source).not.toContain('Codex 支持自动安装和配置')
    expect(source).toContain('重新检测')
    expect(source).toContain('安装 Codex')
  })
})
