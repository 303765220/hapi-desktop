export type ImageCreationAspectRatio = '1:1' | '16:9' | '9:16' | '4:3' | '3:4' | '3:2' | '2:3'

export type ImageCreationJobStatus = 'queued' | 'running' | 'succeeded' | 'failed'

export interface ImageCreationDraft {
  keyId: number
  prompt: string
  model: string
  aspectRatio: ImageCreationAspectRatio
  count: number
  referenceImages: File[]
}

export interface ImageCreationReferenceImagePreview {
  id: string
  name: string
  previewUrl: string
  file: File
}

export interface ImageGenerationRequestPayload extends ImageCreationDraft {}

export interface ImageGenerationResponseItem {
  b64_json?: string
  url?: string
  revised_prompt?: string
  mime_type?: string
}

export interface ImageGenerationResponse {
  id?: string
  status?: ImageCreationJobStatus
  error?: string | null
  created_at?: string
  updated_at?: string
  created?: number
  data: ImageGenerationResponseItem[]
}

export interface ImageCreationResult {
  b64Json?: string
  url?: string
  mimeType?: string
  revisedPrompt?: string
}

export interface ImageCreationJob {
  id: string
  status: ImageCreationJobStatus
  keyId: number
  prompt: string
  model: string
  aspectRatio: ImageCreationAspectRatio
  count: number
  results: ImageCreationResult[]
  createdAt: string
  updatedAt: string
  error?: string | null
}

export interface ImageCreationHistoryItem {
  id: string
  keyId: number
  prompt: string
  model: string
  aspectRatio: ImageCreationAspectRatio
  count: number
  previewUrl: string
  createdAt: string
}
