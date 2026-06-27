<template>
  <div class="space-y-6">
    <div class="flex items-center justify-end">
      
      
      <div class="flex items-center space-x-3">
        <div class="relative">
          <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-zinc-500" />
          <input 
            v-model="searchQuery"
            type="text" 
            placeholder="搜索模型或关键字..." 
            class="bg-black/50 border border-white/10 rounded-lg pl-9 pr-4 py-2 text-sm text-white placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-purple-500/50 w-64"
            @keyup.enter="fetchUsage"
          />
        </div>
      </div>
    </div>

    <!-- Usage Logs Table -->
    <div class="glass-panel overflow-hidden">
      <div class="overflow-x-auto">
        <table class="w-full text-left border-collapse whitespace-nowrap">
          <thead>
            <tr class="bg-black/40">
              <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">时间 (北京时间)</th>
              <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">用户ID</th>
              <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">模型</th>
              <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">Tokens (In/Out)</th>
              <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">消费 ($)</th>
              <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">IP 地址</th>
              <th class="px-6 py-4 text-xs font-semibold text-zinc-400 uppercase tracking-wider">耗时</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-white/5">
            <tr v-for="log in logs" :key="log.id" class="hover:bg-white/5 transition-colors group">
              <td class="px-6 py-4 text-sm text-zinc-400">
                {{ formatBeijingTime(log.created_at) }}
              </td>
              <td class="px-6 py-4">
                <span class="text-sm font-medium text-white">{{ log.user_id }}</span>
              </td>
              <td class="px-6 py-4">
                <span class="text-xs bg-purple-500/10 text-purple-400 border border-purple-500/20 px-2 py-1 rounded">
                  {{ log.model }}
                </span>
              </td>
              <td class="px-6 py-4 text-sm font-mono">
                <div class="flex items-center space-x-1">
                  <ArrowDownToLine class="w-3 h-3 text-green-400/70" />
                  <span class="text-zinc-300">{{ log.input_tokens || 0 }}</span>
                  <span class="text-zinc-600 mx-1">|</span>
                  <ArrowUpFromLine class="w-3 h-3 text-purple-400/70" />
                  <span class="text-zinc-300">{{ log.output_tokens || 0 }}</span>
                </div>
              </td>
              <td class="px-6 py-4 text-sm font-mono" :class="log.actual_cost > 0 ? 'text-green-400' : 'text-zinc-400'">
                <div class="flex items-center justify-end w-full min-w-[80px]">
                  <span>{{ log.actual_cost ? Number(log.actual_cost).toFixed(6) : '0' }}</span>
                  <Info 
                    class="w-3.5 h-3.5 ml-1.5 text-zinc-500 cursor-help hover:text-white transition-colors"
                    @mouseenter="(e) => showTooltip(e, log)"
                    @mouseleave="hideTooltip"
                  />
                </div>
              </td>
              <td class="px-6 py-4 text-sm font-mono text-zinc-500">
                {{ log.ip_address || '-' }}
              </td>
              <td class="px-6 py-4">
                <div class="flex flex-col">
                  <span class="text-sm text-purple-400 font-medium">{{ log.rate_multiplier || 1 }}x</span>
                  <span class="text-xs text-zinc-500">{{ log.stream ? '流式' : '同步' }}</span>
                </div>
              </td>
              <td class="px-6 py-4">
                <div class="flex flex-col justify-center">
                  <span v-if="log.first_token_ms != null && log.first_token_ms > 0" class="text-xs text-zinc-500 font-mono mb-1">首字: {{ formatDuration(log.first_token_ms) }}</span>
                  <span class="text-sm text-zinc-400 font-mono">总计: {{ formatDuration(log.duration_ms) }}</span>
                </div>
              </td>
            </tr>
            <tr v-if="logs.length === 0 && !loading">
              <td colspan="8" class="px-6 py-12 text-center text-zinc-500">
                暂无使用记录
              </td>
            </tr>
            <tr v-if="loading">
              <td colspan="7" class="px-6 py-12 text-center text-zinc-500">
                加载中...
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      
      <!-- Pagination -->
      <div class="px-6 py-4 border-t border-white/5 flex items-center justify-between" v-if="totalLogs > 0">
        <span class="text-xs text-zinc-500">显示 {{ (currentPage - 1) * pageSize + 1 }} 到 {{ Math.min(currentPage * pageSize, totalLogs) }} 条，共 {{ totalLogs }} 条记录</span>
        <div class="flex space-x-1">
          <button @click="changePage(currentPage - 1)" :disabled="currentPage <= 1" class="px-3 py-1 bg-white/5 text-zinc-400 text-xs rounded hover:bg-white/10 transition disabled:opacity-50">上一页</button>
          <button class="px-3 py-1 bg-purple-600 text-white text-xs rounded shadow-lg shadow-purple-500/20">{{ currentPage }}</button>
          <button @click="changePage(currentPage + 1)" :disabled="currentPage * pageSize >= totalLogs" class="px-3 py-1 bg-white/5 text-zinc-400 text-xs rounded hover:bg-white/10 transition disabled:opacity-50">下一页</button>
        </div>
      </div>
    </div>

    <!-- Global Tooltip for Cost Details -->
    <Teleport to="body">
      <div v-if="hoveredLog" class="fixed z-[9999] -translate-y-1/2 pointer-events-none transition-opacity duration-150" :style="tooltipStyle">
        <div class="bg-[#1e232e] border border-white/10 p-3 rounded-xl shadow-2xl w-[250px] text-xs space-y-1.5 text-zinc-300">
          <div class="text-sm font-medium text-white mb-2">成本明细</div>
          
          <div class="flex justify-between">
            <span>输入成本</span>
            <span class="font-mono text-white">${{ (hoveredLog.input_cost || 0).toFixed(6) }}</span>
          </div>
          <div class="flex justify-between">
            <span>输出成本</span>
            <span class="font-mono text-white">${{ (hoveredLog.output_cost || 0).toFixed(6) }}</span>
          </div>
          <div class="flex justify-between">
            <span>输入单价</span>
            <span class="font-mono text-sky-400">{{ formatTokenPricePerMillion(hoveredLog.input_cost, hoveredLog.input_tokens) }} / 1M Token</span>
          </div>
          <div class="flex justify-between">
            <span>输出单价</span>
            <span class="font-mono text-purple-400">{{ formatTokenPricePerMillion(hoveredLog.output_cost, hoveredLog.output_tokens) }} / 1M Token</span>
          </div>
          
          <div class="h-px bg-white/10 my-2"></div>
          
          <div class="flex justify-between">
            <span>服务档位</span>
            <span class="text-sky-400 font-medium">{{ hoveredLog.service_tier || 'Default' }}</span>
          </div>
          <div class="flex justify-between">
            <span>倍率</span>
            <span class="text-blue-400 font-medium">{{ hoveredLog.rate_multiplier || 1 }}x</span>
          </div>
          <div class="flex justify-between">
            <span>原始</span>
            <span class="font-mono text-white">${{ (hoveredLog.total_cost || 0).toFixed(6) }}</span>
          </div>
          
          <div class="h-px bg-white/10 my-2"></div>
          
          <div class="flex justify-between items-center mt-2">
            <span class="text-zinc-400">计费</span>
            <span class="text-green-400 font-bold font-mono text-sm">${{ (hoveredLog.actual_cost || 0).toFixed(6) }}</span>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Search, Info, ArrowDownToLine, ArrowUpFromLine } from '@lucide/vue'
import { adminAPI } from '@/api/admin'
import type { AdminUsageLog } from '@/types'

const logs = ref<AdminUsageLog[]>([])
const loading = ref(false)
const searchQuery = ref('')
const currentPage = ref(1)
const pageSize = ref(20)
const totalLogs = ref(0)

// Tooltip State
const hoveredLog = ref<any>(null)
const tooltipStyle = ref({ top: '0px', left: '0px' })

const showTooltip = (event: MouseEvent, log: any) => {
  hoveredLog.value = log
  const rect = (event.target as HTMLElement).getBoundingClientRect()
  tooltipStyle.value = {
    top: `${rect.top + rect.height / 2}px`,
    left: `${rect.left - 265}px` // 250px width + 15px gap
  }
}

const hideTooltip = () => {
  hoveredLog.value = null
}

const fetchUsage = async () => {
  loading.value = true
  try {
    const res = await adminAPI.usage.list({
      page: currentPage.value,
      page_size: pageSize.value,
      model: searchQuery.value || undefined
    })
    logs.value = res.items || []
    totalLogs.value = res.total || 0
  } catch (err) {
    console.error("Failed to fetch usage logs", err)
  } finally {
    loading.value = false
  }
}

const changePage = (page: number) => {
  if (page < 1 || (page - 1) * pageSize.value >= totalLogs.value) return
  currentPage.value = page
  fetchUsage()
}

onMounted(() => {
  fetchUsage()
})

const formatBeijingTime = (dateStr: string) => {
  if (!dateStr) return '-'
  const d = new Date(dateStr)
  return new Intl.DateTimeFormat('zh-CN', {
    timeZone: 'Asia/Shanghai',
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  }).format(d).replace(/\//g, '-')
}

const formatTokenPricePerMillion = (cost: number | undefined, tokens: number | undefined) => {
  if (!cost || !tokens) return '$0.0000'
  return '$' + ((cost / tokens) * 1000000).toFixed(4)
}

const formatDuration = (ms: number | null | undefined) => {
  if (!ms) return '-'
  if (ms >= 1000) return (ms / 1000).toFixed(2) + 's'
  return ms + 'ms'
}
</script>
