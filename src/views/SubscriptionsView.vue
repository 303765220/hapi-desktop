<template>
  <div class="flex flex-col h-full bg-[#09090b] text-[#f4f4f5] overflow-y-auto custom-scrollbar">
    <!-- Main Content Area -->
    <div class="p-8 flex-1">
      <div v-if="loading" class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
        <div v-for="i in 3" :key="i" class="glass-panel p-6 rounded-2xl animate-pulse h-[320px]">
          <div class="h-6 w-1/3 bg-white/10 rounded mb-4"></div>
          <div class="h-4 w-1/4 bg-white/5 rounded mb-8"></div>
          <div class="space-y-6">
            <div class="h-8 w-full bg-white/5 rounded"></div>
            <div class="h-8 w-full bg-white/5 rounded"></div>
            <div class="h-8 w-full bg-white/5 rounded"></div>
          </div>
        </div>
      </div>

      <div v-else-if="subscriptions.length === 0" class="flex flex-col items-center justify-center py-32">
        <div class="w-20 h-20 bg-zinc-900 rounded-full flex items-center justify-center mb-6 border border-white/5 shadow-2xl">
          <Layers class="w-10 h-10 text-zinc-700" />
        </div>
        <h3 class="text-xl font-semibold text-zinc-300 mb-2">暂无可用订阅</h3>
        <p class="text-zinc-500 text-sm max-w-md text-center">
          您目前没有任何活跃或历史订阅。请联系管理员开通相应的 API 调用权限。
        </p>
      </div>

      <div v-else class="grid grid-cols-1 lg:grid-cols-2 2xl:grid-cols-3 gap-6">
        <div 
          v-for="sub in subscriptions" 
          :key="sub.id"
          class="glass-panel p-6 rounded-2xl border transition-all duration-300 hover:shadow-2xl hover:shadow-purple-500/5 group relative overflow-hidden flex flex-col"
          :class="sub.status === 'active' ? 'border-purple-500/20 hover:border-purple-500/40' : 'border-white/5 opacity-80 hover:opacity-100'"
        >
          <!-- Background Glow for active subscriptions -->
          <div v-if="sub.status === 'active'" class="absolute -top-12 -right-12 w-32 h-32 bg-purple-500/10 rounded-full blur-3xl group-hover:bg-purple-500/20 transition-all duration-500 pointer-events-none"></div>

          <!-- Card Header -->
          <div class="flex justify-between items-start mb-6 relative z-10">
            <div>
              <h3 class="text-xl font-bold text-white flex items-center">
                {{ sub.group?.name || `分组 ${sub.group_id}` }}
              </h3>
              <p class="text-xs text-zinc-500 mt-1 font-mono">
                起始: {{ formatDate(sub.starts_at) }}
                <span v-if="sub.expires_at"> · 截至: {{ formatDate(sub.expires_at) }}</span>
                <span v-else> · 长期有效</span>
              </p>
            </div>
            
            <div 
              class="px-2.5 py-1 rounded-full text-xs font-semibold flex items-center"
              :class="{
                'bg-emerald-500/10 text-emerald-400 border border-emerald-500/20': sub.status === 'active',
                'bg-zinc-800 text-zinc-400 border border-zinc-700': sub.status === 'expired',
                'bg-red-500/10 text-red-400 border border-red-500/20': sub.status === 'revoked'
              }"
            >
              <div v-if="sub.status === 'active'" class="w-1.5 h-1.5 rounded-full bg-emerald-400 mr-1.5 animate-pulse"></div>
              {{ formatStatus(sub.status) }}
            </div>
          </div>

          <!-- Progress Bars -->
          <div class="space-y-6 flex-1 relative z-10">
            <template v-if="hasAnyLimit(sub)">
              <div v-if="sub.group?.daily_limit_usd" class="space-y-2">
                <div class="flex justify-between text-sm">
                  <span class="text-zinc-400 flex items-center"><CalendarDays class="w-3.5 h-3.5 mr-1.5 text-blue-400"/>今日限额</span>
                  <span class="font-medium text-zinc-200">
                    <span class="text-white">${{ (sub.daily_usage_usd || 0).toFixed(4) }}</span> / ${{ sub.group.daily_limit_usd.toFixed(4) }}
                  </span>
                </div>
                <div class="h-1.5 w-full bg-zinc-800 rounded-full overflow-hidden">
                  <div 
                    class="h-full rounded-full transition-all duration-1000"
                    :class="getProgressBarColor(getPercentage(sub.daily_usage_usd, sub.group.daily_limit_usd))"
                    :style="`width: ${getPercentage(sub.daily_usage_usd, sub.group.daily_limit_usd)}%`"
                  ></div>
                </div>
                <div class="flex justify-between text-[10px] text-zinc-500">
                  <span>{{ getPercentage(sub.daily_usage_usd, sub.group.daily_limit_usd).toFixed(1) }}% 已用</span>
                </div>
              </div>

              <div v-if="sub.group?.weekly_limit_usd" class="space-y-2">
                <div class="flex justify-between text-sm">
                  <span class="text-zinc-400 flex items-center"><CalendarRange class="w-3.5 h-3.5 mr-1.5 text-purple-400"/>本周限额</span>
                  <span class="font-medium text-zinc-200">
                    <span class="text-white">${{ (sub.weekly_usage_usd || 0).toFixed(4) }}</span> / ${{ sub.group.weekly_limit_usd.toFixed(4) }}
                  </span>
                </div>
                <div class="h-1.5 w-full bg-zinc-800 rounded-full overflow-hidden">
                  <div 
                    class="h-full rounded-full transition-all duration-1000"
                    :class="getProgressBarColor(getPercentage(sub.weekly_usage_usd, sub.group.weekly_limit_usd))"
                    :style="`width: ${getPercentage(sub.weekly_usage_usd, sub.group.weekly_limit_usd)}%`"
                  ></div>
                </div>
                <div class="flex justify-between text-[10px] text-zinc-500">
                  <span>{{ getPercentage(sub.weekly_usage_usd, sub.group.weekly_limit_usd).toFixed(1) }}% 已用</span>
                </div>
              </div>

              <div v-if="sub.group?.monthly_limit_usd" class="space-y-2">
                <div class="flex justify-between text-sm">
                  <span class="text-zinc-400 flex items-center"><Calendar class="w-3.5 h-3.5 mr-1.5 text-fuchsia-400"/>本月限额</span>
                  <span class="font-medium text-zinc-200">
                    <span class="text-white">${{ (sub.monthly_usage_usd || 0).toFixed(4) }}</span> / ${{ sub.group.monthly_limit_usd.toFixed(4) }}
                  </span>
                </div>
                <div class="h-1.5 w-full bg-zinc-800 rounded-full overflow-hidden">
                  <div 
                    class="h-full rounded-full transition-all duration-1000"
                    :class="getProgressBarColor(getPercentage(sub.monthly_usage_usd, sub.group.monthly_limit_usd))"
                    :style="`width: ${getPercentage(sub.monthly_usage_usd, sub.group.monthly_limit_usd)}%`"
                  ></div>
                </div>
                <div class="flex justify-between text-[10px] text-zinc-500">
                  <span>{{ getPercentage(sub.monthly_usage_usd, sub.group.monthly_limit_usd).toFixed(1) }}% 已用</span>
                </div>
              </div>
            </template>
            <div v-else class="flex items-center justify-center h-24 text-zinc-500 text-sm italic">
              此订阅无用量限制
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Layers, Calendar, CalendarDays, CalendarRange } from '@lucide/vue'
import { getMySubscriptions } from '@/api/subscriptions'
import type { UserSubscription } from '@/types'

const loading = ref(true)
const subscriptions = ref<UserSubscription[]>([])

onMounted(async () => {
  try {
    loading.value = true
    
    // Fetch subscriptions
    let subs = await getMySubscriptions()
    
    // Handle array or wrapped paginated response
    if (!Array.isArray(subs) && (subs as any).items) {
      subs = (subs as any).items
    } else if (!Array.isArray(subs) && (subs as any).data) {
      subs = (subs as any).data
    }
    subscriptions.value = Array.isArray(subs) ? subs : []
    
  } catch (error) {
    console.error('Failed to load subscriptions:', error)
  } finally {
    loading.value = false
  }
})

const hasAnyLimit = (sub: UserSubscription) => {
  return (
    !!sub.group?.daily_limit_usd ||
    !!sub.group?.weekly_limit_usd ||
    !!sub.group?.monthly_limit_usd
  )
}

const getPercentage = (used: number | undefined, limit: number | null | undefined) => {
  if (!limit || limit <= 0) return 0
  const u = used || 0
  return Math.min((u / limit) * 100, 100)
}

const formatDate = (dateString?: string | null) => {
  if (!dateString) return ''
  const date = new Date(dateString)
  return new Intl.DateTimeFormat('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit'
  }).format(date)
}

const formatStatus = (status: string) => {
  switch(status) {
    case 'active': return '活跃中'
    case 'expired': return '已过期'
    case 'revoked': return '已撤销'
    default: return status
  }
}

const getProgressBarColor = (percentage: number) => {
  if (percentage >= 100) return 'bg-red-500 shadow-[0_0_10px_rgba(239,68,68,0.5)]'
  if (percentage >= 80) return 'bg-orange-500 shadow-[0_0_10px_rgba(249,115,22,0.5)]'
  if (percentage >= 50) return 'bg-yellow-500'
  return 'bg-purple-500'
}
</script>
