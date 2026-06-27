<template>
  <div class="space-y-6">
    <div class="flex items-center justify-end">
      
      
      <div class="flex items-center space-x-3">
        <div class="relative">
          <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-zinc-500" />
          <input 
            v-model="searchQuery"
            type="text" 
            placeholder="搜索兑换码..." 
            class="bg-black/50 border border-white/10 rounded-lg pl-9 pr-4 py-2 text-sm text-white placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-purple-500/50 w-64"
            @keyup.enter="fetchCodes"
          />
        </div>
        <button class="flex items-center px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white text-sm font-medium rounded-lg transition shadow-lg shadow-purple-500/20">
          <Plus class="w-4 h-4 mr-2" />
          生成兑换码
        </button>
      </div>
    </div>

    <!-- Codes Table -->
    <div class="glass-panel overflow-hidden">
      <table class="w-full text-left border-collapse">
        <thead>
          <tr class="bg-black/40">
            <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">兑换码</th>
            <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">类型</th>
            <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">额度/内容</th>
            <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">状态</th>
            <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">使用人</th>
            <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider text-right">操作</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-white/5">
          <tr v-for="code in codes" :key="code.id" class="hover:bg-white/5 transition-colors group">
            <td class="px-6 py-4">
              <div class="font-mono text-sm text-white">{{ code.code }}</div>
            </td>
            <td class="px-6 py-4">
              <span class="text-xs font-medium px-2 py-1 rounded" :class="code.type === 'balance' ? 'bg-blue-500/10 text-blue-400 border border-blue-500/20' : 'bg-purple-500/10 text-purple-400 border border-purple-500/20'">
                {{ code.type === 'balance' ? '余额' : '订阅' }}
              </span>
            </td>
            <td class="px-6 py-4">
              <span class="text-sm font-mono text-zinc-300">
                {{ code.type === 'balance' ? '¥' + code.value : '订阅配置' }}
              </span>
            </td>
            <td class="px-6 py-4">
              <span class="flex items-center text-xs font-medium" :class="code.status === 'unused' ? 'text-green-400' : code.status === 'used' ? 'text-zinc-500' : 'text-red-400'">
                <span class="w-1.5 h-1.5 rounded-full mr-2" :class="code.status === 'unused' ? 'bg-green-500' : code.status === 'used' ? 'bg-zinc-500' : 'bg-red-500'"></span>
                {{ code.status === 'unused' ? '未使用' : code.status === 'used' ? '已使用' : code.status }}
              </span>
            </td>
            <td class="px-6 py-4 text-sm text-zinc-400">
              {{ code.used_by || '--' }}
            </td>
            <td class="px-6 py-4 text-right">
              <div class="flex items-center justify-end space-x-2 opacity-0 group-hover:opacity-100 transition-opacity">
                <button class="p-1.5 bg-red-500/10 hover:bg-red-500/20 rounded text-red-400 transition" title="删除">
                  <Trash2 class="w-4 h-4" />
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
      
      <!-- Pagination -->
      <div class="px-6 py-4 border-t border-white/5 flex items-center justify-between">
        <span class="text-xs text-zinc-500">显示 {{ (currentPage - 1) * pageSize + 1 }} 到 {{ Math.min(currentPage * pageSize, totalCodes) }} 条，共 {{ totalCodes }} 个兑换码</span>
        <div class="flex space-x-1">
          <button @click="changePage(currentPage - 1)" :disabled="currentPage <= 1" class="px-3 py-1 bg-white/5 text-zinc-400 text-xs rounded hover:bg-white/10 transition disabled:opacity-50">上一页</button>
          <button class="px-3 py-1 bg-purple-600 text-white text-xs rounded shadow-lg shadow-purple-500/20">{{ currentPage }}</button>
          <button @click="changePage(currentPage + 1)" :disabled="currentPage * pageSize >= totalCodes" class="px-3 py-1 bg-white/5 text-zinc-400 text-xs rounded hover:bg-white/10 transition disabled:opacity-50">下一页</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Search, Plus, Trash2 } from '@lucide/vue'
import { adminAPI } from '@/api/admin'
import type { RedeemCode } from '@/types'

const codes = ref<RedeemCode[]>([])
const loading = ref(false)
const searchQuery = ref('')
const currentPage = ref(1)
const pageSize = ref(20)
const totalCodes = ref(0)

const fetchCodes = async () => {
  loading.value = true
  try {
    const res = await adminAPI.redeem.list(currentPage.value, pageSize.value, { search: searchQuery.value })
    codes.value = res.items
    totalCodes.value = res.total
  } catch (err) {
    console.error("Failed to fetch redeem codes", err)
  } finally {
    loading.value = false
  }
}

const changePage = (page: number) => {
  if (page < 1 || (page - 1) * pageSize.value >= totalCodes.value) return
  currentPage.value = page
  fetchCodes()
}

onMounted(() => {
  fetchCodes()
})
</script>
