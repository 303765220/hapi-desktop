<template>
  <div class="space-y-6">
    <!-- Network Status Panel -->
    <div class="glass-panel p-4 flex flex-wrap items-center justify-between gap-4">
      <div class="flex items-center space-x-3">
        <div class="relative flex h-3 w-3">
          <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span>
          <span class="relative inline-flex rounded-full h-3 w-3 bg-green-500"></span>
        </div>
        <span class="text-sm font-medium text-white">所有系统运行正常</span>
      </div>
      <div class="flex space-x-6 text-sm">
        <div class="flex items-center text-zinc-400">
          <span class="w-2 h-2 rounded-full bg-green-500 mr-2"></span>
          OpenAI (99.9% 在线)
        </div>
        <div class="flex items-center text-zinc-400">
          <span class="w-2 h-2 rounded-full bg-green-500 mr-2"></span>
          Anthropic (100% 在线)
        </div>
      </div>
    </div>

    <div v-if="loading" class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
      <div v-for="i in 10" :key="i" class="glass-panel p-6 flex flex-col relative overflow-hidden animate-pulse">
        <div class="flex items-start justify-between mb-4">
          <div class="flex items-center space-x-3">
            <div class="w-12 h-12 rounded-xl bg-white/5 border border-white/10"></div>
            <div>
              <div class="w-24 h-5 bg-white/10 rounded mb-2"></div>
              <div class="w-16 h-3 bg-white/5 rounded"></div>
            </div>
          </div>
          <div class="w-12 h-6 bg-white/5 rounded"></div>
        </div>
        <div class="w-full h-10 bg-white/5 rounded mb-6 mt-2"></div>
        <div class="grid grid-cols-2 gap-y-4 gap-x-4 pt-4 border-t border-white/10 mt-auto">
          <div>
            <div class="w-16 h-3 bg-white/5 rounded mb-2"></div>
            <div class="w-20 h-4 bg-white/10 rounded"></div>
          </div>
          <div>
            <div class="w-16 h-3 bg-white/5 rounded mb-2"></div>
            <div class="w-20 h-4 bg-white/10 rounded"></div>
          </div>
          <div>
            <div class="w-16 h-3 bg-white/5 rounded mb-2"></div>
            <div class="w-20 h-4 bg-white/10 rounded"></div>
          </div>
        </div>
      </div>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
      
      <!-- Model Card -->
      <div 
        v-for="model in filteredModels" 
        :key="model.id"
        class="glass-panel p-6 flex flex-col relative overflow-hidden group hover:border-purple-500/50 transition-colors"
      >
        <div class="absolute top-0 right-0 w-32 h-32 rounded-full blur-3xl opacity-20 transition-all duration-500 -translate-y-1/2 translate-x-1/2 group-hover:opacity-40" :class="getProviderColor(model.platform)"></div>
        
        <div class="flex items-start justify-between mb-4 relative z-10">
          <div class="flex items-center space-x-3">
            <div class="w-12 h-12 rounded-xl bg-white/5 border border-white/10 flex items-center justify-center shadow-lg" v-html="getModelIcon(model.platform)">
            </div>
            <div>
              <h3 class="font-semibold text-white text-lg leading-tight">{{ model.name }}</h3>
              <span class="text-xs font-medium text-zinc-400">{{ model.platform || 'Unknown' }}</span>
            </div>
          </div>
          <span class="text-xs font-medium px-2 py-1 bg-green-500/10 text-green-400 rounded border border-green-500/20">正常</span>
        </div>
        
        <p class="text-sm text-zinc-400 mb-6 flex-1 line-clamp-2">支持 {{ model.pricing?.billing_mode === 'per_request' ? '按次' : '按 Token' }} 计费的高性能模型。</p>
        
        <div class="grid grid-cols-2 gap-y-4 gap-x-4 pt-4 border-t border-white/10 mt-auto">
          <div>
            <div class="text-xs text-zinc-500 mb-1">输入 / Input</div>
            <div class="text-sm font-mono text-white">¥{{ model.pricing?.input_price || 0 }} <span class="text-zinc-500 text-xs">/ 1M</span></div>
          </div>
          <div>
            <div class="text-xs text-zinc-500 mb-1">输出 / Output</div>
            <div class="text-sm font-mono text-white">¥{{ model.pricing?.output_price || 0 }} <span class="text-zinc-500 text-xs">/ 1M</span></div>
          </div>
          <div v-if="model.pricing?.cache_read_price !== undefined">
            <div class="text-xs text-zinc-500 mb-1">命中缓存</div>
            <div class="text-sm font-mono text-sky-400">¥{{ model.pricing?.cache_read_price }} <span class="text-zinc-500 text-xs">/ 1M</span></div>
          </div>
          <div v-if="model.pricing?.cache_creation_price !== undefined">
            <div class="text-xs text-zinc-500 mb-1">缓存写入</div>
            <div class="text-sm font-mono text-amber-400">¥{{ model.pricing?.cache_creation_price }} <span class="text-zinc-500 text-xs">/ 1M</span></div>
          </div>
        </div>
      </div>
      
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
const models = ref<any[]>([
  { id: 1, name: 'opus-4.8', platform: 'Anthropic', pricing: { input_price: 15.0, output_price: 75.0, cache_read_price: 1.5, cache_creation_price: 18.75 } },
  { id: 2, name: 'opus-4.7', platform: 'Anthropic', pricing: { input_price: 3.0, output_price: 15.0, cache_read_price: 0.3, cache_creation_price: 3.75 } },
  { id: 3, name: 'opus-4.6', platform: 'Anthropic', pricing: { input_price: 0.25, output_price: 1.25, cache_read_price: 0.025, cache_creation_price: 0.30 } },
  { id: 4, name: 'gpt-5.5', platform: 'OpenAI', pricing: { input_price: 10.0, output_price: 30.0, cache_read_price: 5.0 } },
  { id: 5, name: 'gpt-5.4', platform: 'OpenAI', pricing: { input_price: 5.0, output_price: 15.0, cache_read_price: 2.5 } },
  { id: 6, name: 'gpt-5.4-mini', platform: 'OpenAI', pricing: { input_price: 0.15, output_price: 0.6, cache_read_price: 0.075 } },
  { id: 7, name: 'DeepSeek-v4-pro', platform: 'DeepSeek', pricing: { input_price: 3.0, output_price: 6.0, cache_read_price: 0.025 } },
  { id: 8, name: 'DeepSeek-v4-flash', platform: 'DeepSeek', pricing: { input_price: 1.0, output_price: 2.0, cache_read_price: 0.02 } },
  { id: 9, name: 'glm-5.2', platform: 'Zhipu', pricing: { input_price: 1.4, output_price: 4.4, cache_read_price: 0.26 } },
  { id: 10, name: 'kimi-2.7', platform: 'Moonshot', pricing: { input_price: 0.95, output_price: 4.0, cache_read_price: 0.19 } }
])

const loading = ref(true)

onMounted(() => {
  // API call disabled, using hardcoded models
  setTimeout(() => {
    loading.value = false
  }, 500)
})

const filteredModels = computed(() => {
  return models.value
})

const getProviderColor = (provider: string) => {
  const map: Record<string, string> = {
    'openai': 'bg-green-500',
    'anthropic': 'bg-amber-500',
    'google': 'bg-blue-500',
    'meta': 'bg-blue-600',
    'midjourney': 'bg-purple-500',
    'azure': 'bg-cyan-500',
    'deepseek': 'bg-cyan-500',
    'zhipu': 'bg-indigo-500',
    'moonshot': 'bg-violet-500'
  }
  const color = map[provider.toLowerCase()]
  return color || 'bg-zinc-500'
}

const getModelIcon = (platform: string) => {
  const p = platform.toLowerCase()
  if (p === 'openai') {
    return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-7 h-7 text-green-400 drop-shadow-[0_0_8px_rgba(74,222,128,0.5)]"><path d="M22.2819 9.8211a5.9847 5.9847 0 0 0-.5157-4.9108 6.0462 6.0462 0 0 0-6.5098-2.9A6.0651 6.0651 0 0 0 4.9807 4.1818a5.9847 5.9847 0 0 0-3.9977 2.9 6.0462 6.0462 0 0 0 .7427 7.0966 5.98 5.98 0 0 0 .511 4.9107 6.051 6.051 0 0 0 6.5146 2.9001A5.9847 5.9847 0 0 0 13.2599 24a6.0557 6.0557 0 0 0 5.7718-4.2057 5.9847 5.9847 0 0 0 3.998-2.9001 6.0557 6.0557 0 0 0-.7478-7.073Zm-9.022 12.6081a4.4755 4.4755 0 0 1-2.8764-1.0408l.1419-.0804 4.7783-2.7582a.7948.7948 0 0 0 .3927-.6813v-6.7369l2.02 1.1686a.071.071 0 0 1 .038.052v5.5826a4.504 4.504 0 0 1-4.4945 4.4944Zm-9.6607-4.1254a4.4708 4.4708 0 0 1-.5346-3.0137l.142.0852 4.783 2.7582a.7712.7712 0 0 0 .7806 0l5.8428-3.3685v2.3324a.0804.0804 0 0 1-.0332.0615L9.74 19.9502a4.4992 4.4992 0 0 1-6.1408-1.6464ZM2.3408 7.8956a4.485 4.485 0 0 1 2.3655-1.9728V11.6a.7664.7664 0 0 0 .3879.6765l5.8144 3.3543-2.0201 1.1685a.0757.0757 0 0 1-.071 0l-4.8303-2.7865A4.504 4.504 0 0 1 2.3408 7.8956Zm16.0993 3.8558L12.5973 8.3829V6.0505a.0757.0757 0 0 1 .0332-.0615l4.8303-2.7866a4.4992 4.4992 0 0 1 6.6802 4.66 4.485 4.485 0 0 1-2.3655 1.9728V15.51a.7664.7664 0 0 1-.3879.6765l-2.9474 1.7021Zm-.3927-4.2255-2.02-1.1686a.071.071 0 0 0-.038-.052V6.3688a4.504 4.504 0 0 0-4.4945-4.4945 4.4755 4.4755 0 0 0-2.8764 1.0408l-.1419.0804 4.7783 2.7582a.7948.7948 0 0 1 .3927.6813ZM10.7453 5.083l2.0201-1.1685a.0757.0757 0 0 0 .071 0l4.8303 2.7866a4.504 4.504 0 0 1 1.735 6.0792l-.142-.0852-4.783-2.7582a.7712.7712 0 0 0-.7806 0L7.8533 13.3054V10.973a.0804.0804 0 0 0 .0332-.0615ZM12.0042 14.4827l-3.2372-1.8679 3.2372-1.8678 3.2372 1.8678Z"/></svg>`
  }
  if (p === 'anthropic') {
    return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-7 h-7 text-amber-500 drop-shadow-[0_0_8px_rgba(245,158,11,0.5)]"><path d="M17.382 3.12h4.526l-7.79 17.76h-4.887L17.382 3.12zM6.618 3.12H2.092L9.882 20.88h4.887L6.618 3.12z"/></svg>`
  }
  return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6 text-zinc-400"><rect x="4" y="4" width="16" height="16" rx="2" ry="2"></rect><rect x="9" y="9" width="6" height="6"></rect><line x1="9" y1="1" x2="9" y2="4"></line><line x1="15" y1="1" x2="15" y2="4"></line><line x1="9" y1="20" x2="9" y2="23"></line><line x1="15" y1="20" x2="15" y2="23"></line><line x1="20" y1="9" x2="23" y2="9"></line><line x1="20" y1="14" x2="23" y2="14"></line><line x1="1" y1="9" x2="4" y2="9"></line><line x1="1" y1="14" x2="4" y2="14"></line></svg>`
}
</script>
