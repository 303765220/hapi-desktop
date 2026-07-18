import { apiClient } from './client'
import type {
  ImageCreationDraft,
  ImageCreationHistoryItem,
  ImageCreationJob,
  ImageCreationResult,
  ImageGenerationResponse,
  ImageGenerationResponseItem,
} from '@/types/image-creation'

const IMAGE_GENERATIONS_PATH = '/images/generations'
const IMAGE_EDITS_PATH = '/images/edits'

function appendCommonFields(formData: FormData, draft: ImageCreationDraft): void {
  formData.append('key_id', String(draft.keyId))
  formData.append('prompt', draft.prompt)
  formData.append('model', draft.model)
  formData.append('aspect_ratio', draft.aspectRatio)
  formData.append('n', String(draft.count))
}

function normalizeResult(item: ImageGenerationResponseItem): ImageCreationResult {
  return {
    b64Json: item.b64_json,
    url: item.url,
    mimeType: item.mime_type,
    revisedPrompt: item.revised_prompt
  }
}

function normalizeJob(
  draft: ImageCreationDraft,
  response: ImageGenerationResponse,
  fallbackStatus: ImageCreationJob['status'] = 'succeeded',
): ImageCreationJob {
  const createdAt = response.created_at || new Date().toISOString()
  const updatedAt = response.updated_at || createdAt
  const results = (response.data || []).map(normalizeResult)
  const status = response.status || (results.length > 0 ? 'succeeded' : fallbackStatus)

  return {
    id: response.id || `${draft.keyId}-${Date.now()}`,
    status,
    keyId: draft.keyId,
    prompt: draft.prompt,
    model: draft.model,
    aspectRatio: draft.aspectRatio,
    count: draft.count,
    results,
    createdAt,
    updatedAt
  }
}

export function buildImageGenerationFormData(draft: ImageCreationDraft): FormData {
  const formData = new FormData()
  appendCommonFields(formData, draft)

  draft.referenceImages.forEach((image) => {
    formData.append('image[]', image, image.name)
  })

  return formData
}

export async function createImageGenerationJob(draft: ImageCreationDraft): Promise<ImageCreationJob> {
  if (draft.referenceImages.length > 0) {
    const { data } = await apiClient.post<ImageGenerationResponse>(
      IMAGE_EDITS_PATH,
      buildImageGenerationFormData(draft),
      {
        headers: {
          'Content-Type': 'multipart/form-data'
        }
      }
    )
    return normalizeJob(draft, data)
  }

  const payload = {
    key_id: draft.keyId,
    prompt: draft.prompt,
    model: draft.model,
    aspect_ratio: draft.aspectRatio,
    n: draft.count,
  }
  const { data } = await apiClient.post<ImageGenerationResponse>(IMAGE_GENERATIONS_PATH, payload)
  return normalizeJob(draft, data)
}

export async function getImageGenerationJob(jobId: string): Promise<ImageCreationJob> {
  const { data } = await apiClient.get<ImageGenerationResponse>(`${IMAGE_GENERATIONS_PATH}/${jobId}`)
  return normalizeJob(
    {
      keyId: 0,
      prompt: '',
      model: '',
      aspectRatio: '1:1',
      count: 1,
      referenceImages: []
    },
    data,
    data.data.length > 0 ? 'succeeded' : 'running',
  )
}

export async function listImageGenerationJobs(): Promise<ImageCreationHistoryItem[]> {
  const { data } = await apiClient.get<{ items?: ImageCreationJob[] }>(IMAGE_GENERATIONS_PATH)
  return (data.items || []).map((job) => ({
    id: job.id,
    keyId: job.keyId,
    prompt: job.prompt,
    model: job.model,
    aspectRatio: job.aspectRatio,
    count: job.count,
    previewUrl: resolveImageDownloadUrl(job.results[0]),
    createdAt: job.createdAt
  }))
}

export function resolveImageDownloadUrl(
  result?: ImageCreationResult | ImageGenerationResponseItem | null,
): string {
  if (!result) return ''
  const rawResult = result as ImageCreationResult & ImageGenerationResponseItem
  const url = rawResult.url
  if (url) return url
  const b64Json = rawResult.b64Json || rawResult.b64_json
  if (!b64Json) return ''
  const mimeType = rawResult.mimeType || rawResult.mime_type
  return `data:${mimeType || 'image/png'};base64,${b64Json}`
}

export const imageCreationAPI = {
  buildImageGenerationFormData,
  createImageGenerationJob,
  getImageGenerationJob,
  listImageGenerationJobs,
  resolveImageDownloadUrl,
}

export default imageCreationAPI
