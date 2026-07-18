import { list as listApiKeys } from '@/api/keys'
import {
  createImageGenerationJob as createImageGenerationJobApi,
  getImageGenerationJob as getImageGenerationJobApi,
  listImageGenerationJobs as listImageGenerationJobsApi,
} from '@/api/image-creation'
import type { ApiKey } from '@/types'
import type {
  ImageCreationDraft,
  ImageCreationHistoryItem,
  ImageCreationJob,
  ImageCreationResult,
} from '@/types/image-creation'

const MAX_REFERENCE_IMAGES = 4
// 图片创作页只允许选择 113 分组的 Key，因为这套工作台只面向这一组独立的 OpenAI key 池；改成其他分组会把不兼容的 key 暴露出来，导致生成接口直接失败。是否仍然正确，应通过页面下拉选项和实际提交接口成功率一起验证。
const IMAGE_CREATION_KEY_GROUP_ID = 113
// 轮询间隔不能太短，否则前端会不断打扰后端；默认 1.5 秒可以兼顾反馈速度和请求量。改更小会增加抖动，改更大则会让用户明显感觉卡住。后续需要用真实生成任务的返回节奏验证。 
const DEFAULT_POLL_INTERVAL_MS = 1500
// 最大轮询次数不能无限大，否则卡住的任务会把页面挂死；默认 20 次约等于 30 秒，足以覆盖大多数图片生成。改更小会更容易误判失败，改更大会拖长失败反馈。后续要根据真实生成耗时和失败样本再调。
const DEFAULT_POLL_ATTEMPTS = 20

export interface ImageCreationBootstrap {
  keys: ApiKey[]
  selectedKey: ApiKey | null
  history: ImageCreationHistoryItem[]
}

export interface PollImageCreationJobOptions {
  intervalMs?: number
  maxAttempts?: number
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

export function selectDefaultImageCreationKey(keys: ApiKey[]): ApiKey | null {
  return keys.find((key) => key.status === 'active') || null
}

export function filterImageCreationKeys(keys: ApiKey[]): ApiKey[] {
  return keys.filter((key) => key.status === 'active' && key.group_id === IMAGE_CREATION_KEY_GROUP_ID)
}

export function validateImageCreationDraft(draft: ImageCreationDraft): ImageCreationDraft {
  const prompt = draft.prompt.trim()
  const model = draft.model.trim()

  if (!draft.keyId || draft.keyId <= 0) {
    throw new Error('请选择有效的站内 API Key。')
  }
  if (!prompt) {
    throw new Error('请输入图片提示词。')
  }
  if (!model) {
    throw new Error('请选择图片模型。')
  }
  if (draft.count < 1) {
    throw new Error('生成数量至少为 1。')
  }
  if (draft.count > MAX_REFERENCE_IMAGES) {
    throw new Error('生成数量暂时不超过 4。')
  }
  if (draft.referenceImages.length > MAX_REFERENCE_IMAGES) {
    throw new Error('referenceImages 最多允许 4 张。')
  }

  return {
    ...draft,
    prompt,
    model,
    referenceImages: draft.referenceImages.slice(0, MAX_REFERENCE_IMAGES),
  }
}

export async function loadImageCreationBootstrap(): Promise<ImageCreationBootstrap> {
  const [keyPage, history] = await Promise.all([
    listApiKeys(1, 100, {
      status: 'active',
      group_id: IMAGE_CREATION_KEY_GROUP_ID,
      sort_by: 'created_at',
      sort_order: 'desc',
    }),
    listImageGenerationJobsApi(),
  ])

  const keys = filterImageCreationKeys(keyPage.items || [])

  return {
    keys,
    selectedKey: selectDefaultImageCreationKey(keys),
    history: [...history].sort((left, right) => right.createdAt.localeCompare(left.createdAt)),
  }
}

export async function submitImageCreationDraft(
  draft: ImageCreationDraft,
): Promise<ImageCreationJob> {
  return createImageGenerationJobApi(validateImageCreationDraft(draft))
}

export async function pollImageCreationJob(
  jobId: string,
  options: PollImageCreationJobOptions = {},
): Promise<ImageCreationJob> {
  const intervalMs = options.intervalMs ?? DEFAULT_POLL_INTERVAL_MS
  const maxAttempts = options.maxAttempts ?? DEFAULT_POLL_ATTEMPTS

  let latestJob: ImageCreationJob | null = null

  for (let attempt = 0; attempt < maxAttempts; attempt += 1) {
    latestJob = await getImageGenerationJobApi(jobId)
    if (latestJob.status === 'succeeded' || latestJob.status === 'failed') {
      return latestJob
    }

    if (attempt < maxAttempts - 1 && intervalMs > 0) {
      await sleep(intervalMs)
    }
  }

  return latestJob || getImageGenerationJobApi(jobId)
}

export function hydrateDraftFromJob(job: ImageCreationJob): ImageCreationDraft {
  return {
    keyId: job.keyId,
    prompt: job.prompt,
    model: job.model,
    aspectRatio: job.aspectRatio,
    count: job.count,
    referenceImages: [],
  }
}

export async function convertImageResultToReferenceFile(
  result: ImageCreationResult,
  filename = 'reference.png',
): Promise<File> {
  const response = await fetch(result.url || `data:${result.mimeType || 'image/png'};base64,${result.b64Json || ''}`)
  const blob = await response.blob()
  return new File([blob], filename, { type: blob.type || result.mimeType || 'image/png' })
}
