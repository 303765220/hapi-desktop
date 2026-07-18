<template>
  <section class="rounded-xl border border-white/10 bg-black/30 p-4 space-y-4">
    <div class="flex items-start justify-between gap-3">
      <div>
        <h3 class="text-base font-semibold text-white">生成输入</h3>
        <p class="mt-1 text-sm text-zinc-500">提示词 + 参考图，最多 4 张。</p>
      </div>
      <div class="min-w-[220px]">
        <label class="mb-1 block text-xs font-medium text-zinc-500">站内 API Key</label>
        <select
          class="h-10 w-full rounded-lg border border-white/10 bg-black/60 px-3 text-sm text-white outline-none transition focus:border-purple-500 focus:ring-1 focus:ring-purple-500 disabled:opacity-50"
          :disabled="loading || keys.length === 0"
          :value="selectedKeyId ?? ''"
          @change="handleKeyChange"
        >
          <option value="" disabled>{{ loading ? '正在加载...' : '请选择 API Key' }}</option>
          <option v-for="key in keys" :key="key.id" :value="key.id">
            {{ key.name || `API Key #${key.id}` }}
          </option>
        </select>
      </div>
    </div>

    <div class="grid gap-4 lg:grid-cols-[minmax(0,1.6fr)_minmax(280px,0.8fr)]">
      <div class="space-y-4">
        <div>
          <label class="mb-1 block text-xs font-medium text-zinc-500">提示词</label>
          <textarea
            class="min-h-[170px] w-full rounded-xl border border-white/10 bg-black/60 px-4 py-3 text-sm text-white outline-none transition placeholder:text-zinc-600 focus:border-purple-500 focus:ring-1 focus:ring-purple-500"
            :value="prompt"
            placeholder="描述你想要的图片内容、风格、光影、构图、材质..."
            @input="emit('update:prompt', ($event.target as HTMLTextAreaElement).value)"
          />
        </div>

        <div class="rounded-xl border border-dashed border-white/10 bg-black/30 p-4">
          <div class="flex items-center justify-between gap-3">
            <div>
              <label class="block text-xs font-medium text-zinc-500">参考图</label>
              <p class="mt-1 text-xs text-zinc-600">上传后可作为构图、人物、风格或商品参考。</p>
            </div>
            <button
              type="button"
              class="inline-flex items-center gap-2 rounded-lg border border-white/10 bg-white/5 px-3 py-2 text-sm text-white transition hover:bg-white/10 disabled:cursor-not-allowed disabled:opacity-50"
              :disabled="submitting"
              @click="triggerUpload"
            >
              <Upload class="h-4 w-4" />
              上传
            </button>
          </div>

          <input
            ref="uploadInput"
            type="file"
            accept="image/*"
            multiple
            class="hidden"
            @change="handleUpload"
          />

          <div v-if="referenceImages.length" class="mt-4 grid grid-cols-2 gap-3 md:grid-cols-4">
            <div
              v-for="image in referenceImages"
              :key="image.id"
              class="group overflow-hidden rounded-lg border border-white/10 bg-black/50"
            >
              <img :src="image.previewUrl" :alt="image.name" class="h-28 w-full object-cover" />
              <div class="flex items-center justify-between gap-2 px-2 py-2">
                <p class="min-w-0 flex-1 truncate text-xs text-zinc-400">{{ image.name }}</p>
                <button
                  type="button"
                  class="inline-flex items-center justify-center rounded-md p-1 text-zinc-500 transition hover:bg-white/10 hover:text-white"
                  @click="emit('remove-reference', image.id)"
                >
                  <Trash2 class="h-4 w-4" />
                </button>
              </div>
            </div>
          </div>

          <div v-else class="mt-4 rounded-lg border border-white/5 bg-white/5 px-3 py-4 text-sm text-zinc-500">
            还没有参考图。
          </div>
        </div>
      </div>

      <div class="space-y-4">
        <div>
          <label class="mb-1 block text-xs font-medium text-zinc-500">模型</label>
          <select
            class="h-10 w-full rounded-lg border border-white/10 bg-black/60 px-3 text-sm text-white outline-none transition focus:border-purple-500 focus:ring-1 focus:ring-purple-500"
            :value="model"
            @change="emit('update:model', ($event.target as HTMLSelectElement).value)"
          >
            <option v-for="option in modelOptions" :key="option.value" :value="option.value">
              {{ option.label }}
            </option>
          </select>
        </div>

        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="mb-1 block text-xs font-medium text-zinc-500">比例</label>
            <select
              class="h-10 w-full rounded-lg border border-white/10 bg-black/60 px-3 text-sm text-white outline-none transition focus:border-purple-500 focus:ring-1 focus:ring-purple-500"
              :value="aspectRatio"
              @change="emit('update:aspectRatio', ($event.target as HTMLSelectElement).value as ImageCreationAspectRatio)"
            >
              <option v-for="option in aspectOptions" :key="option.value" :value="option.value">
                {{ option.label }}
              </option>
            </select>
          </div>

          <div>
            <label class="mb-1 block text-xs font-medium text-zinc-500">张数</label>
            <input
              class="h-10 w-full rounded-lg border border-white/10 bg-black/60 px-3 text-sm text-white outline-none transition focus:border-purple-500 focus:ring-1 focus:ring-purple-500"
              type="number"
              min="1"
              max="4"
              :value="count"
              @input="emit('update:count', Number(($event.target as HTMLInputElement).value || 1))"
            />
          </div>
        </div>

        <div class="rounded-xl border border-white/10 bg-white/5 p-4 text-sm text-zinc-400">
          <div class="flex items-center justify-between">
            <span>当前已选 Key</span>
            <span class="text-white">
              {{ activeKeyLabel }}
            </span>
          </div>
        </div>

        <button
          type="button"
          class="inline-flex h-11 w-full items-center justify-center gap-2 rounded-xl bg-purple-600 px-4 text-sm font-semibold text-white transition hover:bg-purple-500 disabled:cursor-not-allowed disabled:bg-purple-600/50"
          :disabled="submitting || loading || !selectedKeyId"
          @click="emit('submit')"
        >
          <Sparkles class="h-4 w-4" />
          {{ submitting ? '生成中...' : '开始生成' }}
        </button>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { Sparkles, Trash2, Upload } from '@lucide/vue'
import type { ApiKey } from '@/types'
import type { ImageCreationAspectRatio, ImageCreationReferenceImagePreview } from '@/types/image-creation'

const props = defineProps<{
  keys: ApiKey[]
  selectedKeyId: number | null
  prompt: string
  model: string
  aspectRatio: ImageCreationAspectRatio
  count: number
  referenceImages: ImageCreationReferenceImagePreview[]
  loading?: boolean
  submitting?: boolean
}>()

const emit = defineEmits<{
  (event: 'update:selectedKeyId', value: number | null): void
  (event: 'update:prompt', value: string): void
  (event: 'update:model', value: string): void
  (event: 'update:aspectRatio', value: ImageCreationAspectRatio): void
  (event: 'update:count', value: number): void
  (event: 'upload-reference', files: File[]): void
  (event: 'remove-reference', id: string): void
  (event: 'submit'): void
}>()

const uploadInput = ref<HTMLInputElement | null>(null)

const modelOptions = [
  { value: 'gpt-image-2', label: 'gpt-image-2' },
  { value: 'gpt-image-1.5', label: 'gpt-image-1.5' },
  { value: 'gpt-image-1', label: 'gpt-image-1' }
]

const aspectOptions = [
  { value: '1:1', label: '1:1' },
  { value: '4:3', label: '4:3' },
  { value: '3:4', label: '3:4' },
  { value: '16:9', label: '16:9' },
  { value: '9:16', label: '9:16' },
  { value: '3:2', label: '3:2' },
  { value: '2:3', label: '2:3' },
]

const activeKeyLabel = computed(() => {
  const key = props.keys.find((item) => item.id === props.selectedKeyId)
  return key ? (key.name || `API Key #${key.id}`) : '未选择'
})

const handleKeyChange = (event: Event) => {
  const value = (event.target as HTMLSelectElement).value
  emit('update:selectedKeyId', value ? Number(value) : null)
}

const triggerUpload = () => {
  uploadInput.value?.click()
}

const handleUpload = (event: Event) => {
  const files = Array.from((event.target as HTMLInputElement).files || [])
  if (files.length > 0) {
    emit('upload-reference', files)
  }
  if (uploadInput.value) {
    uploadInput.value.value = ''
  }
}
</script>
