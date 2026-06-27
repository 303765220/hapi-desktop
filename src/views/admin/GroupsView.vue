<template>
  <div class="space-y-6">
    <div class="flex items-center justify-end">
      
      
      <button class="flex items-center px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white text-sm font-medium rounded-lg transition shadow-lg shadow-purple-500/20">
        <Plus class="w-4 h-4 mr-2" />
        新建分组
      </button>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6" v-if="groups.length > 0">
      
      <!-- Group Card -->
      <div v-for="group in groups" :key="group.id" class="glass-panel p-6 flex flex-col relative overflow-hidden group">
        <div class="absolute top-0 right-0 w-32 h-32 bg-blue-500/10 rounded-full blur-2xl group-hover:bg-blue-500/20 transition-all duration-500" :class="group.is_exclusive ? 'bg-amber-500/10' : ''"></div>
        <div class="absolute top-0 left-4 px-3 py-1 bg-white/10 text-white text-xs font-bold rounded-b-lg tracking-wider border-x border-b border-white/10" :class="group.is_exclusive ? 'bg-amber-500 border-amber-500 text-white' : ''">ID: {{ group.name }}</div>
        
        <div class="flex items-start justify-between mb-6 mt-4 relative z-10">
          <div>
            <h3 class="text-xl font-bold text-white mb-1">{{ group.name === 'default' ? '默认分组' : group.name === 'vip' ? 'VIP 高级分组' : group.name }}</h3>
            <p class="text-sm text-zinc-400">{{ group.description || (group.is_exclusive ? '专属调度分组。' : '公开注册用户组。') }}</p>
          </div>
          <div class="p-2 rounded-lg border" :class="group.is_exclusive ? 'bg-amber-500/10 border-amber-500/20' : 'bg-blue-500/10 border-blue-500/20'">
            <Crown v-if="group.is_exclusive" class="w-5 h-5 text-amber-400" />
            <Users v-else class="w-5 h-5 text-blue-400" />
          </div>
        </div>
        
        <div class="space-y-4 mb-6 flex-1">
          <div>
            <div class="text-xs font-medium text-zinc-500 mb-2">平台限制</div>
            <div class="flex flex-wrap gap-2">
              <span v-if="group.platform" class="text-xs bg-emerald-500/10 text-emerald-400 px-2 py-1 rounded border border-emerald-500/20">{{ group.platform }}</span>
              <span v-else class="text-xs text-zinc-500">无限制</span>
            </div>
          </div>
          <div>
            <div class="text-xs font-medium text-zinc-500 mb-2">状态与特性</div>
            <div class="flex gap-2">
              <span class="text-xs px-2 py-1 rounded border border-white/10" :class="group.status === 'active' ? 'bg-green-500/10 text-green-400' : 'bg-red-500/10 text-red-400'">{{ group.status === 'active' ? '正常' : '已禁用' }}</span>
              <span class="text-xs px-2 py-1 rounded border border-white/10 bg-white/5 text-zinc-300" v-if="group.is_exclusive">独占组</span>
            </div>
          </div>
        </div>
        
        <div class="flex space-x-3 pt-4 border-t border-white/5 mt-auto">
          <button class="flex-1 py-2 bg-white/5 hover:bg-white/10 text-white text-sm font-medium rounded-lg transition border border-white/10">编辑分组</button>
          <button class="flex-1 py-2 bg-red-500/10 hover:bg-red-500/20 text-red-400 text-sm font-medium rounded-lg transition border border-red-500/20">删除分组</button>
        </div>
      </div>
    </div>
    <div v-else-if="!loading" class="text-center py-20 text-zinc-500">
      暂无分组
    </div>
    <div v-if="loading" class="text-center py-20 text-zinc-500">
      加载中...
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Plus, Users, Star as Crown } from '@lucide/vue'
import { adminAPI } from '@/api/admin'
import type { AdminGroup } from '@/types'

const groups = ref<AdminGroup[]>([])
const loading = ref(false)

onMounted(async () => {
  loading.value = true
  try {
    const res = await adminAPI.groups.list(1, 100)
    groups.value = res.items
  } catch (err) {
    console.error(err)
  } finally {
    loading.value = false
  }
})
</script>
