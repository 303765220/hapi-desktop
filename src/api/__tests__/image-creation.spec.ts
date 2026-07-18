import { describe, expect, it, vi } from 'vitest'

vi.mock('../client', () => ({
  apiClient: {
    post: vi.fn()
  }
}))

import { apiClient } from '../client'
import {
  buildImageGenerationFormData,
  createImageGenerationJob,
  resolveImageDownloadUrl,
} from '../image-creation'

const makeReferenceImage = (name = 'ref.png') =>
  new File([new Uint8Array([1, 2, 3])], name, { type: 'image/png' })

describe('image creation api', () => {
  it('builds a multipart payload with prompt, key id, model, and reference images', () => {
    const formData = buildImageGenerationFormData({
      keyId: 12,
      prompt: 'a cat in a blue coat',
      model: 'gpt-image-1',
      aspectRatio: '1:1',
      count: 1,
      referenceImages: [makeReferenceImage()]
    })

    expect(formData.get('key_id')).toBe('12')
    expect(formData.get('prompt')).toBe('a cat in a blue coat')
    expect(formData.get('model')).toBe('gpt-image-1')
    expect(formData.get('aspect_ratio')).toBe('1:1')
    expect(formData.get('n')).toBe('1')
    expect(formData.getAll('image[]')).toHaveLength(1)
  })

  it('routes reference image jobs through the edits endpoint', async () => {
    const postSpy = vi.spyOn(apiClient, 'post').mockResolvedValueOnce({
      data: {
        id: 'job_1',
        status: 'completed',
        created_at: '2026-07-18T00:00:00Z',
        updated_at: '2026-07-18T00:00:01Z',
        data: [
          {
            b64_json: Buffer.from('image-bytes').toString('base64'),
            mime_type: 'image/png'
          }
        ]
      }
    } as never)

    await createImageGenerationJob({
      keyId: 12,
      prompt: 'a cat in a blue coat',
      model: 'gpt-image-1',
      aspectRatio: '1:1',
      count: 1,
      referenceImages: [makeReferenceImage()]
    })

    expect(postSpy).toHaveBeenCalledTimes(1)
    expect(postSpy).toHaveBeenCalledWith(
      '/images/edits',
      expect.any(FormData),
      expect.objectContaining({
        headers: {
          'Content-Type': 'multipart/form-data'
        }
      })
    )
  })

  it('turns base64 output into a download URL', () => {
    const url = resolveImageDownloadUrl({
      b64_json: Buffer.from('image-bytes').toString('base64'),
      mime_type: 'image/png'
    })

    expect(url.startsWith('data:image/png;base64,')).toBe(true)
  })
})
