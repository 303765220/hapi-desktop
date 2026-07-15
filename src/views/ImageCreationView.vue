<template>
  <div class="space-y-4">
    <div class="flex flex-col gap-3 rounded-xl border border-white/10 bg-black/30 p-4 sm:flex-row sm:items-center sm:justify-between">
      <div>
        <h2 class="text-lg font-semibold text-white">图片创作</h2>
        <p class="mt-1 text-sm text-zinc-500">本地预览版会把所选站内 API Key 注入 Infinite Canvas。</p>
      </div>

      <div class="flex w-full flex-col gap-2 sm:w-auto sm:min-w-[360px]">
        <label class="text-xs font-medium text-zinc-500">选择 API Key</label>
        <select
          v-model="selectedKey"
          class="h-10 rounded-lg border border-white/10 bg-black/60 px-3 text-sm text-white outline-none transition focus:border-purple-500 focus:ring-1 focus:ring-purple-500 disabled:opacity-50"
          :disabled="loading || apiKeys.length === 0"
        >
          <option value="" disabled>{{ loading ? '正在加载...' : '请选择 API Key' }}</option>
          <option v-for="key in apiKeys" :key="key.id" :value="key.key">
            {{ key.name || `API Key #${key.id}` }} · {{ formatKey(key.key) }}
          </option>
        </select>
      </div>
    </div>

    <div v-if="errorMsg" class="rounded-xl border border-red-500/20 bg-red-500/10 px-4 py-3 text-sm text-red-300">
      {{ errorMsg }}
    </div>

    <div v-if="!selectedKey" class="glass-panel flex min-h-[520px] items-center justify-center p-8 text-center">
      <div>
        <ImageIcon class="mx-auto mb-4 h-12 w-12 text-zinc-500" />
        <h3 class="text-lg font-semibold text-white">请选择一个站内 API Key</h3>
        <p class="mt-2 text-sm text-zinc-500">选择后会打开本地 Infinite Canvas，并使用 Hapi OpenAI 兼容入口。</p>
      </div>
    </div>

    <div v-else class="overflow-hidden rounded-xl border border-white/10 bg-black/40 shadow-2xl shadow-black/30">
      <iframe
        :src="canvasUrl"
        class="h-[calc(100vh-220px)] min-h-[620px] w-full border-0 bg-white"
        title="图片创作"
      ></iframe>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { Image as ImageIcon } from '@lucide/vue'
import { list as getKeysList } from '@/api/keys'
import type { ApiKey } from '@/types'

// 本地预览阶段固定指向 Infinite Canvas dev server；改成线上同域部署时应替换为 `/canvas/`，否则线上用户访问不到本机 3000 端口。用 image-creation-view.spec 锁定当前本地预览行为。
const LOCAL_CANVAS_URL = 'http://127.0.0.1:3000/'
// Infinite Canvas 使用 OpenAI 兼容 Base URL；这里走 Hapi `/v1`，确保请求按所选站内 API Key 进入 Hapi 计费、用量和风控。改成站点根地址可能导致客户端拼接路径不符合 OpenAI SDK 预期。用 image-creation-view.spec 验证。
const HAPI_OPENAI_BASE_URL = 'https://www.hapi666.com/v1'

const apiKeys = ref<ApiKey[]>([])
const selectedKey = ref('')
const loading = ref(false)
const errorMsg = ref('')

const canvasUrl = computed(() => {
  if (!selectedKey.value) return LOCAL_CANVAS_URL
  const params = new URLSearchParams({
    apiKey: selectedKey.value,
    baseUrl: HAPI_OPENAI_BASE_URL
  })
  return `${LOCAL_CANVAS_URL}?${params.toString()}`
})

const formatKey = (key: string) => {
  if (!key) return ''
  if (key.length <= 12) return key
  return `${key.slice(0, 8)}...${key.slice(-4)}`
}

const loadKeys = async () => {
  loading.value = true
  errorMsg.value = ''
  try {
    const res = await getKeysList(1, 100)
    apiKeys.value = (res.items || []).filter(key => key.status === 'active')
    selectedKey.value = apiKeys.value[0]?.key || ''
  } catch (err) {
    console.error('Failed to load API keys for image creation', err)
    errorMsg.value = '加载 API Key 失败，请稍后重试。'
  } finally {
    loading.value = false
  }
}

onMounted(loadKeys)
</script>
