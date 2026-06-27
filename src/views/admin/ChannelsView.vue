<template>
  <div class="space-y-6">
    <div class="flex items-center justify-end">
      
      
      <div class="flex items-center space-x-3">
        <div class="relative">
          <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-zinc-500" />
          <input 
            v-model="searchQuery"
            type="text" 
            placeholder="搜索渠道名称或模型..." 
            class="bg-black/50 border border-white/10 rounded-lg pl-9 pr-4 py-2 text-sm text-white placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-purple-500/50 w-64"
            @keyup.enter="fetchChannels"
          />
        </div>
        <button class="flex items-center px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white text-sm font-medium rounded-lg transition shadow-lg shadow-purple-500/20">
          <Plus class="w-4 h-4 mr-2" />
          添加渠道
        </button>
      </div>
    </div>

    <!-- Channels Table -->
    <div class="glass-panel overflow-hidden">
      <table class="w-full text-left border-collapse">
        <thead>
          <tr class="bg-black/40">
            <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">渠道名称 / ID</th>
            <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">类型</th>
            <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">支持模型</th>
            <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">权重</th>
            <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">响应延迟</th>
            <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">状态</th>
            <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider text-right">操作</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-white/5">
          <tr v-for="channel in channels" :key="channel.id" class="hover:bg-white/5 transition-colors group">
            <td class="px-6 py-4">
              <div class="flex items-center space-x-3">
                <div class="w-8 h-8 rounded-lg border flex items-center justify-center text-xs font-bold" :class="getProviderStyle(channel.type)">
                  <Server class="w-4 h-4" />
                </div>
                <div>
                  <div class="font-medium text-white text-sm">{{ channel.name }}</div>
                  <div class="text-xs text-zinc-500 font-mono">ID: {{ channel.id }}</div>
                </div>
              </div>
            </td>
            <td class="px-6 py-4">
              <span class="text-xs font-medium text-zinc-300 bg-white/5 px-2 py-1 rounded border border-white/10">
                {{ channel.type }}
              </span>
            </td>
            <td class="px-6 py-4">
              <div class="flex flex-wrap gap-1 max-w-[200px]">
                <span v-for="(model, index) in channel.models.slice(0, 3)" :key="index" class="text-[10px] bg-purple-500/10 text-purple-400 px-1.5 py-0.5 rounded border border-purple-500/20">
                  {{ model }}
                </span>
                <span v-if="channel.models.length > 3" class="text-[10px] text-zinc-500">
                  +{{ channel.models.length - 3 }}
                </span>
              </div>
            </td>
            <td class="px-6 py-4">
              <span class="text-sm text-zinc-300">N/A</span>
            </td>
            <td class="px-6 py-4">
              <div class="flex items-center space-x-2">
                <span class="text-sm font-mono text-zinc-400">
                  -- ms
                </span>
              </div>
            </td>
            <td class="px-6 py-4">
              <div class="flex items-center">
                <div class="relative flex h-2 w-2 mr-2">
                  <span v-if="channel.status === 'active'" class="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span>
                  <span class="relative inline-flex rounded-full h-2 w-2" :class="channel.status === 'active' ? 'bg-green-500' : 'bg-red-500'"></span>
                </div>
                <span class="text-xs font-medium" :class="channel.status === 'active' ? 'text-green-400' : 'text-red-400'">
                  {{ channel.status === 'active' ? '在线' : '离线' }}
                </span>
              </div>
            </td>
            <td class="px-6 py-4 text-right">
              <div class="flex items-center justify-end space-x-2 opacity-0 group-hover:opacity-100 transition-opacity">
                <button class="p-1.5 bg-white/5 hover:bg-white/10 rounded text-zinc-400 hover:text-white transition" title="测试连接">
                  <Activity class="w-4 h-4" />
                </button>
                <button class="p-1.5 bg-white/5 hover:bg-white/10 rounded text-zinc-400 hover:text-white transition" title="编辑渠道">
                  <Edit class="w-4 h-4" />
                </button>
                <button class="p-1.5 bg-red-500/10 hover:bg-red-500/20 rounded text-red-400 transition" title="删除渠道">
                  <Trash2 class="w-4 h-4" />
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Search, Plus, Server, Edit, Trash2, Activity } from '@lucide/vue'
import { adminAPI } from '@/api/admin'

const channels = ref<any[]>([])
const loading = ref(false)
const searchQuery = ref('')

const fetchChannels = async () => {
  loading.value = true
  try {
    const res = await adminAPI.channels.list(1, 100) // Simplification for now
    channels.value = res.items.map((c: any) => {
      const type = c.model_pricing?.[0]?.platform || 'Unknown'
      const models = c.model_pricing?.flatMap((p: any) => p.models) || []
      return {
        id: c.id,
        name: c.name,
        type,
        models,
        status: c.status
      }
    })
  } catch (err) {
    console.error("Failed to fetch channels", err)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchChannels()
})

const getProviderStyle = (type: string) => {
  switch (type.toLowerCase()) {
    case 'openai': return 'bg-emerald-500/10 border-emerald-500/20 text-emerald-400'
    case 'anthropic': return 'bg-orange-500/10 border-orange-500/20 text-orange-400'
    case 'google': return 'bg-blue-500/10 border-blue-500/20 text-blue-400'
    case 'azure': return 'bg-cyan-500/10 border-cyan-500/20 text-cyan-400'
    default: return 'bg-zinc-500/10 border-zinc-500/20 text-zinc-400'
  }
}
</script>
