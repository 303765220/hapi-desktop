import { readFileSync } from 'node:fs'
import { fileURLToPath, URL } from 'node:url'

import { describe, expect, it } from 'vitest'

describe('image creation view', () => {
  it('renders the native image creation workbench instead of the infinite canvas iframe', () => {
    const source = readFileSync(fileURLToPath(new URL('../ImageCreationView.vue', import.meta.url)), 'utf8')

    expect(source).not.toContain('http://127.0.0.1:3000/')
    expect(source).not.toContain('<iframe')
    expect(source).toContain('ImageCreationComposer')
    expect(source).toContain('ImageCreationHistoryList')
    expect(source).toContain('ImageCreationResultGrid')
    expect(source).toContain('referenceImages')
    expect(source).toContain("gpt-image-2")
  })
})
