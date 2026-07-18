import { describe, expect, it, vi } from 'vitest'

vi.mock('@/api/keys', () => ({
  list: vi.fn()
}))

vi.mock('@/api/image-creation', () => ({
  createImageGenerationJob: vi.fn(),
  getImageGenerationJob: vi.fn(),
  listImageGenerationJobs: vi.fn()
}))

import { list as listApiKeys } from '@/api/keys'
import { createImageGenerationJob, getImageGenerationJob, listImageGenerationJobs } from '@/api/image-creation'
import {
  hydrateDraftFromJob,
  loadImageCreationBootstrap,
  pollImageCreationJob,
  selectDefaultImageCreationKey,
  submitImageCreationDraft,
  validateImageCreationDraft,
} from '../image-creation'

const makeReferenceImages = (count: number) =>
  Array.from({ length: count }, (_, index) => new File([`file-${index}`], `ref-${index}.png`, { type: 'image/png' }))

describe('image creation service', () => {
  it('selects the first active key and rejects more than four reference images', () => {
    const keys = [
      { id: 1, status: 'inactive' },
      { id: 2, status: 'active' },
      { id: 3, status: 'active' }
    ] as any[]

    expect(selectDefaultImageCreationKey(keys)?.id).toBe(2)
    expect(() =>
      validateImageCreationDraft({
        keyId: 2,
        prompt: 'sunset',
        model: 'gpt-image-1',
        aspectRatio: '1:1',
        count: 1,
        referenceImages: makeReferenceImages(5)
      })
    ).toThrow('referenceImages')
  })

  it('loads bootstrap state from active keys and recent jobs', async () => {
    vi.mocked(listApiKeys).mockResolvedValueOnce({
      items: [
        { id: 1, status: 'inactive' },
        { id: 2, status: 'active', key: 'sk-2', name: 'Second' },
        { id: 3, status: 'active', key: 'sk-3', name: 'Third' }
      ]
    } as never)
    vi.mocked(listImageGenerationJobs).mockResolvedValueOnce([
      {
        id: 'job-1',
        keyId: 2,
        prompt: 'sunset',
        model: 'gpt-image-1',
        aspectRatio: '1:1',
        previewUrl: 'https://example.com/out.png',
        createdAt: '2026-07-18T00:00:00Z'
      }
    ] as never)

    await expect(loadImageCreationBootstrap()).resolves.toMatchObject({
      keys: [
        { id: 1, status: 'inactive' },
        { id: 2, status: 'active', key: 'sk-2', name: 'Second' },
        { id: 3, status: 'active', key: 'sk-3', name: 'Third' }
      ],
      selectedKey: { id: 2, status: 'active', key: 'sk-2', name: 'Second' },
      history: [
        {
          id: 'job-1',
          previewUrl: 'https://example.com/out.png'
        }
      ]
    })
  })

  it('hydrates a retry draft from a finished job', () => {
    expect(
      hydrateDraftFromJob({
        id: 'job-1',
        status: 'succeeded',
        keyId: 2,
        prompt: 'sunset',
        model: 'gpt-image-1',
        aspectRatio: '1:1',
        count: 2,
        results: [],
        createdAt: '2026-07-18T00:00:00Z',
        updatedAt: '2026-07-18T00:00:01Z'
      })
    ).toMatchObject({
      keyId: 2,
      prompt: 'sunset',
      model: 'gpt-image-1',
      aspectRatio: '1:1',
      count: 2,
      referenceImages: []
    })
  })

  it('submits a normalized draft and polls a running job until it finishes', async () => {
    vi.mocked(createImageGenerationJob).mockResolvedValueOnce({
      id: 'job-2',
      status: 'running',
      keyId: 2,
      prompt: 'sunset',
      model: 'gpt-image-1',
      aspectRatio: '1:1',
      count: 1,
      results: [],
      createdAt: '2026-07-18T00:00:00Z',
      updatedAt: '2026-07-18T00:00:00Z'
    } as never)
    vi.mocked(getImageGenerationJob).mockResolvedValueOnce({
      id: 'job-2',
      status: 'succeeded',
      keyId: 2,
      prompt: 'sunset',
      model: 'gpt-image-1',
      aspectRatio: '1:1',
      count: 1,
      results: [
        {
          b64Json: Buffer.from('image-bytes').toString('base64'),
          mimeType: 'image/png'
        }
      ],
      createdAt: '2026-07-18T00:00:00Z',
      updatedAt: '2026-07-18T00:00:02Z'
    } as never)

    const draft = {
      keyId: 2,
      prompt: 'sunset',
      model: 'gpt-image-1',
      aspectRatio: '1:1',
      count: 1,
      referenceImages: [new File(['ref'], 'ref.png', { type: 'image/png' })]
    }

    await expect(submitImageCreationDraft(draft)).resolves.toMatchObject({
      id: 'job-2',
      status: 'running'
    })

    await expect(pollImageCreationJob('job-2', { intervalMs: 0, maxAttempts: 2 })).resolves.toMatchObject({
      id: 'job-2',
      status: 'succeeded'
    })
  })
})
