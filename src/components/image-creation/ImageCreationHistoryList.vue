<template>
  <section class="rounded-xl border border-white/10 bg-black/30 p-4 space-y-4">
    <div class="flex items-center justify-between gap-3">
      <div>
        <h3 class="text-base font-semibold text-white">最近任务</h3>
        <p class="mt-1 text-sm text-zinc-500">再次生成会沿用上次的提示词和参数。</p>
      </div>
      <History class="h-5 w-5 text-zinc-500" />
    </div>

    <div v-if="history.length === 0" class="rounded-lg border border-white/5 bg-white/5 px-3 py-4 text-sm text-zinc-500">
      暂无历史记录。
    </div>

    <div v-else class="space-y-3">
      <article
        v-for="item in history"
        :key="item.id"
        class="rounded-xl border border-white/10 bg-white/5 p-3 transition hover:border-white/20"
      >
        <div class="flex items-start gap-3">
          <img
            v-if="item.previewUrl"
            :src="item.previewUrl"
            :alt="item.prompt"
            class="h-16 w-16 rounded-lg object-cover border border-white/10"
          />
          <div v-else class="flex h-16 w-16 items-center justify-center rounded-lg border border-white/10 bg-white/5">
            <ImageIcon class="h-5 w-5 text-zinc-500" />
          </div>
          <div class="min-w-0 flex-1">
            <p class="truncate text-sm font-medium text-white">{{ item.prompt }}</p>
            <p class="mt-1 text-xs text-zinc-500">{{ item.model }} · {{ item.aspectRatio }} · {{ item.count }} 张</p>
            <p class="mt-1 text-xs text-zinc-600">{{ item.createdAt }}</p>
          </div>
        </div>

        <div class="mt-3 flex gap-2">
          <button
            type="button"
            class="inline-flex flex-1 items-center justify-center gap-2 rounded-lg border border-white/10 bg-white/5 px-3 py-2 text-sm text-white transition hover:bg-white/10"
            @click="emit('retry', item)"
          >
            <RotateCcw class="h-4 w-4" />
            再次生成
          </button>
          <button
            type="button"
            class="inline-flex flex-1 items-center justify-center gap-2 rounded-lg border border-white/10 bg-white/5 px-3 py-2 text-sm text-white transition hover:bg-white/10"
            @click="emit('use-as-reference', item)"
          >
            <ImageIcon class="h-4 w-4" />
            设为参考图
          </button>
        </div>
      </article>
    </div>
  </section>
</template>

<script setup lang="ts">
import { History, Image as ImageIcon, RotateCcw } from '@lucide/vue'
import type { ImageCreationHistoryItem } from '@/types/image-creation'

defineProps<{
  history: ImageCreationHistoryItem[]
}>()

const emit = defineEmits<{
  (event: 'retry', item: ImageCreationHistoryItem): void
  (event: 'use-as-reference', item: ImageCreationHistoryItem): void
}>()
</script>
