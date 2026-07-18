<template>
  <section class="rounded-xl border border-white/10 bg-black/30 p-4 space-y-4">
    <div class="flex items-center justify-between gap-3">
      <div>
        <h3 class="text-base font-semibold text-white">生成结果</h3>
        <p class="mt-1 text-sm text-zinc-500">结果可以直接下载，也可以再拿来做参考图。</p>
      </div>
      <div class="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs text-zinc-400">
        <span :class="statusDotClass" class="h-2 w-2 rounded-full"></span>
        <span>{{ statusLabel }}</span>
      </div>
    </div>

    <div v-if="!job && !loading" class="rounded-lg border border-white/5 bg-white/5 px-3 py-4 text-sm text-zinc-500">
      还没有生成结果。
    </div>

    <div v-else-if="loading" class="rounded-lg border border-white/5 bg-white/5 px-3 py-8 text-sm text-zinc-500">
      正在生成，请稍候。
    </div>

    <div v-else class="space-y-4">
      <div v-if="job?.error" class="rounded-xl border border-red-500/20 bg-red-500/10 px-4 py-3 text-sm text-red-200">
        {{ job.error }}
      </div>

      <div v-if="job?.results.length" class="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
        <article
          v-for="(result, index) in job.results"
          :key="`${job.id}-${index}`"
          class="overflow-hidden rounded-xl border border-white/10 bg-black/40"
        >
          <div v-if="resolveUrl(result)" class="relative">
            <img :src="resolveUrl(result)" alt="生成结果" class="h-72 w-full object-cover bg-black" />
          </div>
          <div v-else class="flex h-72 items-center justify-center bg-black text-zinc-500">
            <ImageIcon class="h-8 w-8" />
          </div>
          <div class="space-y-3 p-3">
            <p v-if="result.revisedPrompt" class="line-clamp-2 text-xs text-zinc-500">
              {{ result.revisedPrompt }}
            </p>
            <div class="flex gap-2">
              <button
                type="button"
                class="inline-flex flex-1 items-center justify-center gap-2 rounded-lg border border-white/10 bg-white/5 px-3 py-2 text-sm text-white transition hover:bg-white/10"
                @click="emit('download', result, index)"
              >
                <Download class="h-4 w-4" />
                下载
              </button>
              <button
                type="button"
                class="inline-flex flex-1 items-center justify-center gap-2 rounded-lg border border-white/10 bg-white/5 px-3 py-2 text-sm text-white transition hover:bg-white/10"
                @click="emit('use-as-reference', result, index)"
              >
                <ImageIcon class="h-4 w-4" />
                设为参考图
              </button>
            </div>
          </div>
        </article>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Download, Image as ImageIcon } from '@lucide/vue'
import type { ImageCreationJob, ImageCreationResult } from '@/types/image-creation'
import { resolveImageDownloadUrl } from '@/api/image-creation'

const props = defineProps<{
  job: ImageCreationJob | null
  loading?: boolean
}>()

const emit = defineEmits<{
  (event: 'download', result: ImageCreationResult, index: number): void
  (event: 'use-as-reference', result: ImageCreationResult, index: number): void
}>()

const statusLabel = computed(() => {
  if (props.loading) return '生成中'
  if (!props.job) return '空闲'
  if (props.job.status === 'failed') return '失败'
  if (props.job.status === 'running' || props.job.status === 'queued') return '排队中'
  return '已完成'
})

const statusDotClass = computed(() => {
  if (props.loading || props.job?.status === 'running') return 'bg-amber-400'
  if (props.job?.status === 'failed') return 'bg-red-400'
  if (props.job?.status === 'succeeded') return 'bg-emerald-400'
  return 'bg-zinc-500'
})

const resolveUrl = (result: ImageCreationResult) => resolveImageDownloadUrl(result)
</script>
