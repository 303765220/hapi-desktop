<template>
  <div class="space-y-6">
    <div class="flex items-center justify-end">
      
      
      <div class="flex items-center space-x-3">
        <div class="relative">
          <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-zinc-500" />
          <input 
            v-model="searchQuery"
            type="text" 
            placeholder="搜索用户名或邮箱..." 
            class="bg-black/50 border border-white/10 rounded-lg pl-9 pr-4 py-2 text-sm text-white placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-purple-500/50 w-64"
            @keyup.enter="fetchUsers"
          />
        </div>
        <button class="flex items-center px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white text-sm font-medium rounded-lg transition shadow-lg shadow-purple-500/20">
          <UserPlus class="w-4 h-4 mr-2" />
          添加用户
        </button>
      </div>
    </div>

    <!-- Users Table -->
    <div class="glass-panel overflow-hidden">
      <table class="w-full text-left border-collapse">
        <thead>
          <tr class="bg-black/40">
            <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">用户</th>
            <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">角色</th>
            <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">当前余额</th>
            <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">已消费</th>
            <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">状态</th>
            <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider text-right">操作</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-white/5">
          <tr v-for="user in users" :key="user.id" class="hover:bg-white/5 transition-colors group">
            <td class="px-6 py-4">
              <div class="flex items-center space-x-3">
                <div class="w-8 h-8 rounded-full bg-gradient-to-br flex items-center justify-center text-xs font-bold text-white shadow-lg" :class="user.role === 'admin' ? 'from-purple-500 to-indigo-600' : 'from-zinc-700 to-zinc-800'">
                  {{ (user.username || user.email).charAt(0).toUpperCase() }}
                </div>
                <div>
                  <div class="font-medium text-white text-sm">{{ user.username || 'N/A' }}</div>
                  <div class="text-xs text-zinc-500">{{ user.email }}</div>
                </div>
              </div>
            </td>
            <td class="px-6 py-4">
              <span class="text-xs font-medium px-2 py-1 rounded" :class="user.role === 'admin' ? 'bg-purple-500/10 text-purple-400 border border-purple-500/20' : 'bg-white/5 text-zinc-400 border border-white/10'">
                {{ user.role === 'admin' ? '管理员' : '普通用户' }}
              </span>
            </td>
            <td class="px-6 py-4 text-sm font-mono" :class="user.balance < 5 ? 'text-red-400' : 'text-green-400'">
              ¥{{ user.balance.toFixed(2) }}
            </td>
            <td class="px-6 py-4 text-sm font-mono text-zinc-400">
              --
            </td>
            <td class="px-6 py-4">
              <span class="flex items-center text-xs font-medium" :class="user.status === 'active' ? 'text-green-400' : 'text-red-400'">
                <span class="w-1.5 h-1.5 rounded-full mr-2" :class="user.status === 'active' ? 'bg-green-500' : 'bg-red-500'"></span>
                {{ user.status === 'active' ? '正常' : '已封禁' }}
              </span>
            </td>
            <td class="px-6 py-4 text-right">
              <div class="flex items-center justify-end space-x-2 opacity-0 group-hover:opacity-100 transition-opacity">
                <button class="p-1.5 bg-white/5 hover:bg-white/10 rounded text-zinc-400 hover:text-white transition" title="编辑额度">
                  <DollarSign class="w-4 h-4" />
                </button>
                <button class="p-1.5 bg-white/5 hover:bg-white/10 rounded text-zinc-400 hover:text-white transition" title="编辑信息">
                  <Edit class="w-4 h-4" />
                </button>
                <button v-if="user.status === 'active'" class="p-1.5 bg-red-500/10 hover:bg-red-500/20 rounded text-red-400 transition" title="封禁用户">
                  <Ban class="w-4 h-4" />
                </button>
                <button v-else class="p-1.5 bg-green-500/10 hover:bg-green-500/20 rounded text-green-400 transition" title="解封用户">
                  <CheckCircle class="w-4 h-4" />
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
      
      <!-- Pagination -->
      <div class="px-6 py-4 border-t border-white/5 flex items-center justify-between">
        <span class="text-xs text-zinc-500">显示 {{ (currentPage - 1) * pageSize + 1 }} 到 {{ Math.min(currentPage * pageSize, totalUsers) }} 条，共 {{ totalUsers }} 名用户</span>
        <div class="flex space-x-1">
          <button @click="changePage(currentPage - 1)" :disabled="currentPage <= 1" class="px-3 py-1 bg-white/5 text-zinc-400 text-xs rounded hover:bg-white/10 transition disabled:opacity-50">上一页</button>
          <button class="px-3 py-1 bg-purple-600 text-white text-xs rounded shadow-lg shadow-purple-500/20">{{ currentPage }}</button>
          <button @click="changePage(currentPage + 1)" :disabled="currentPage * pageSize >= totalUsers" class="px-3 py-1 bg-white/5 text-zinc-400 text-xs rounded hover:bg-white/10 transition disabled:opacity-50">下一页</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Search, UserPlus, Edit, Ban, DollarSign, CheckCircle } from '@lucide/vue'
import { adminAPI } from '@/api/admin'
import type { AdminUser } from '@/types'

const users = ref<AdminUser[]>([])
const loading = ref(false)
const searchQuery = ref('')
const currentPage = ref(1)
const pageSize = ref(10)
const totalUsers = ref(0)

const fetchUsers = async () => {
  loading.value = true
  try {
    const res = await adminAPI.users.list(currentPage.value, pageSize.value, { search: searchQuery.value })
    users.value = res.items
    totalUsers.value = res.total
  } catch (err) {
    console.error("Failed to fetch users", err)
  } finally {
    loading.value = false
  }
}

const changePage = (page: number) => {
  if (page < 1 || (page - 1) * pageSize.value >= totalUsers.value) return
  currentPage.value = page
  fetchUsers()
}

onMounted(() => {
  fetchUsers()
})
</script>
