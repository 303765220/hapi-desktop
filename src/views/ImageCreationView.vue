<template>
  <div class="space-y-4">
    <section class="rounded-xl border border-white/10 bg-black/30 p-4">
      <div class="flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
        <div class="space-y-1">
          <div class="inline-flex items-center gap-2 rounded-full border border-purple-500/20 bg-purple-500/10 px-3 py-1 text-xs text-purple-300">
            <Sparkles class="h-3.5 w-3.5" />
            图片创作
          </div>
          <h2 class="text-xl font-semibold text-white">提示词 + 参考图生成台</h2>
          <p class="text-sm text-zinc-500">
            选一个站内 Key，上传参考图，填提示词，就可以直接生成、下载，再次生成或回填为参考图。
          </p>
        </div>

        <div class="grid gap-3 sm:grid-cols-3">
          <div class="rounded-lg border border-white/10 bg-white/5 px-3 py-2">
            <p class="text-xs text-zinc-500">可用 Key</p>
            <p class="mt-1 text-sm font-medium text-white">{{ activeKeyLabel }}</p>
          </div>
          <div class="rounded-lg border border-white/10 bg-white/5 px-3 py-2">
            <p class="text-xs text-zinc-500">历史任务</p>
            <p class="mt-1 text-sm font-medium text-white">{{ history.length }}</p>
          </div>
          <div class="rounded-lg border border-white/10 bg-white/5 px-3 py-2">
            <p class="text-xs text-zinc-500">当前状态</p>
            <p class="mt-1 text-sm font-medium text-white">{{ currentStateLabel }}</p>
          </div>
        </div>
      </div>
    </section>

    <div v-if="errorMsg" class="rounded-xl border border-red-500/20 bg-red-500/10 px-4 py-3 text-sm text-red-200">
      {{ errorMsg }}
    </div>

    <div class="grid gap-4 xl:grid-cols-[minmax(0,1.55fr)_minmax(320px,0.9fr)]">
      <div class="space-y-4">
        <ImageCreationComposer
          :keys="keys"
          :selected-key-id="selectedKeyId"
          :prompt="prompt"
          :model="model"
          :aspect-ratio="aspectRatio"
          :count="count"
          :reference-images="referenceImages"
          :loading="loading"
          :submitting="submitting"
          @update:selected-key-id="selectedKeyId = $event"
          @update:prompt="prompt = $event"
          @update:model="model = $event"
          @update:aspect-ratio="aspectRatio = $event"
          @update:count="count = $event"
          @upload-reference="handleUploadReferenceFiles"
          @remove-reference="handleRemoveReference"
          @submit="handleSubmit"
        />

        <ImageCreationResultGrid
          :job="currentJob"
          :loading="submitting || polling"
          @download="handleDownloadResult"
          @use-as-reference="handleUseResultAsReference"
        />
      </div>

      <ImageCreationHistoryList
        :history="history"
        @retry="handleRetryFromHistory"
        @use-as-reference="handleUseHistoryAsReference"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onBeforeUnmount, ref } from 'vue'
import { Sparkles } from '@lucide/vue'
import ImageCreationComposer from '@/components/image-creation/ImageCreationComposer.vue'
import ImageCreationHistoryList from '@/components/image-creation/ImageCreationHistoryList.vue'
import ImageCreationResultGrid from '@/components/image-creation/ImageCreationResultGrid.vue'
import type { ApiKey } from '@/types'
import type { ImageCreationAspectRatio, ImageCreationDraft, ImageCreationHistoryItem, ImageCreationJob, ImageCreationReferenceImagePreview, ImageCreationResult } from '@/types/image-creation'
import { resolveImageDownloadUrl } from '@/api/image-creation'
import {
  convertImageResultToReferenceFile,
  hydrateDraftFromJob,
  loadImageCreationBootstrap,
  pollImageCreationJob,
  selectDefaultImageCreationKey,
  submitImageCreationDraft,
  validateImageCreationDraft,
} from '@/services/image-creation'

const loading = ref(true)
const submitting = ref(false)
const polling = ref(false)
const errorMsg = ref('')
const keys = ref<ApiKey[]>([])
const history = ref<ImageCreationHistoryItem[]>([])
const currentJob = ref<ImageCreationJob | null>(null)
const prompt = ref('')
const model = ref('gpt-image-1')
const aspectRatio = ref<ImageCreationAspectRatio>('1:1')
const count = ref(1)
const selectedKeyId = ref<number | null>(null)
const referenceImages = ref<ImageCreationReferenceImagePreview[]>([])

let referenceSeed = 0

const currentStateLabel = computed(() => {
  if (polling.value) return '轮询中'
  if (submitting.value) return '提交中'
  if (!currentJob.value) return '空闲'
  if (currentJob.value.status === 'failed') return '失败'
  if (currentJob.value.status === 'running' || currentJob.value.status === 'queued') return '排队中'
  return '已完成'
})

const activeKeyLabel = computed(() => {
  const key = keys.value.find((item) => item.id === selectedKeyId.value)
  return key ? (key.name || `API Key #${key.id}`) : '未选择'
})

const mapJobToHistoryItem = (job: ImageCreationJob): ImageCreationHistoryItem => ({
  id: job.id,
  keyId: job.keyId,
  prompt: job.prompt,
  model: job.model,
  aspectRatio: job.aspectRatio,
  count: job.count,
  previewUrl: resolveImageDownloadUrl(job.results[0]),
  createdAt: job.createdAt,
})

const syncHistory = (job: ImageCreationJob) => {
  const nextItem = mapJobToHistoryItem(job)
  history.value = [nextItem, ...history.value.filter((item) => item.id !== nextItem.id)].slice(0, 20)
}

const addReferenceFiles = (files: File[]) => {
  const nextItems = files.slice(0, Math.max(0, 4 - referenceImages.value.length)).map((file) => ({
    id: `ref-${Date.now()}-${referenceSeed += 1}`,
    name: file.name || `reference-${referenceSeed}.png`,
    previewUrl: URL.createObjectURL(file),
    file,
  }))

  referenceImages.value = [...referenceImages.value, ...nextItems].slice(0, 4)
}

const revokeReferencePreviews = (items: ImageCreationReferenceImagePreview[]) => {
  items.forEach((item) => URL.revokeObjectURL(item.previewUrl))
}

const clearReferenceImages = () => {
  revokeReferencePreviews(referenceImages.value)
  referenceImages.value = []
}

const handleUploadReferenceFiles = (files: File[]) => {
  if (referenceImages.value.length + files.length > 4) {
    errorMsg.value = '参考图最多允许 4 张。'
    return
  }
  errorMsg.value = ''
  addReferenceFiles(files)
}

const handleRemoveReference = (id: string) => {
  const removed = referenceImages.value.find((item) => item.id === id)
  if (removed) {
    URL.revokeObjectURL(removed.previewUrl)
  }
  referenceImages.value = referenceImages.value.filter((item) => item.id !== id)
}

const handleDownloadResult = (result: ImageCreationResult, index: number) => {
  const url = resolveImageDownloadUrl(result)
  if (!url) return

  const link = document.createElement('a')
  link.href = url
  link.download = `hapi-image-${Date.now()}-${index + 1}.png`
  link.rel = 'noreferrer'
  document.body.appendChild(link)
  link.click()
  link.remove()
}

const handleUseResultAsReference = async (result: ImageCreationResult, index: number) => {
  const file = await convertImageResultToReferenceFile(result, `result-${index + 1}.png`)
  handleUploadReferenceFiles([file])
}

const handleUseHistoryAsReference = async (item: ImageCreationHistoryItem) => {
  const file = await convertImageResultToReferenceFile({ url: item.previewUrl }, `history-${item.id}.png`)
  handleUploadReferenceFiles([file])
}

const applyJobAsDraft = (job: ImageCreationJob) => {
  const draft = hydrateDraftFromJob(job)
  selectedKeyId.value = draft.keyId
  prompt.value = draft.prompt
  model.value = draft.model || model.value
  aspectRatio.value = draft.aspectRatio
  count.value = draft.count
  clearReferenceImages()
}

const handleRetryFromHistory = (item: ImageCreationHistoryItem) => {
  const job: ImageCreationJob = {
    id: item.id,
    keyId: item.keyId,
    prompt: item.prompt,
    model: item.model,
    aspectRatio: item.aspectRatio,
    count: item.count,
    results: [],
    status: 'queued',
    createdAt: item.createdAt,
    updatedAt: item.createdAt,
  }
  applyJobAsDraft(job)
}

const handleSubmit = async () => {
  errorMsg.value = ''
  if (!selectedKeyId.value) {
    errorMsg.value = '请选择一个站内 API Key。'
    return
  }

  try {
    const draft: ImageCreationDraft = validateImageCreationDraft({
      keyId: selectedKeyId.value,
      prompt: prompt.value,
      model: model.value,
      aspectRatio: aspectRatio.value,
      count: count.value,
      referenceImages: referenceImages.value.map((item) => item.file),
    })

    submitting.value = true
    const job = await submitImageCreationDraft(draft)
    currentJob.value = job
    syncHistory(job)

    if (job.status === 'queued' || job.status === 'running') {
      polling.value = true
      const finished = await pollImageCreationJob(job.id, { intervalMs: 1200, maxAttempts: 20 })
      currentJob.value = finished
      syncHistory(finished)
    }
  } catch (error) {
    errorMsg.value = error instanceof Error ? error.message : '生成失败，请稍后重试。'
  } finally {
    submitting.value = false
    polling.value = false
  }
}

const handleBootstrap = async () => {
  loading.value = true
  errorMsg.value = ''
  try {
    const bootstrap = await loadImageCreationBootstrap()
    keys.value = bootstrap.keys
    history.value = bootstrap.history
    selectedKeyId.value = bootstrap.selectedKey?.id || null

    const firstKey = selectDefaultImageCreationKey(bootstrap.keys)
    if (firstKey && !selectedKeyId.value) {
      selectedKeyId.value = firstKey.id
    }
  } catch (error) {
    console.error('Failed to load image creation bootstrap', error)
    errorMsg.value = '加载图片创作页面失败，请稍后重试。'
  } finally {
    loading.value = false
  }
}

onMounted(handleBootstrap)

onBeforeUnmount(() => {
  revokeReferencePreviews(referenceImages.value)
})
</script>
