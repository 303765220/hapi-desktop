<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex flex-wrap items-center justify-between gap-4">
      <div class="flex flex-wrap items-center gap-3">
        <!-- Custom Date Range Picker -->
        <div class="relative">
          <div v-if="showDatePicker" class="fixed inset-0 z-40" @click="showDatePicker = false"></div>
          <div 
            @click="showDatePicker = !showDatePicker"
            class="relative z-40 flex items-center justify-between space-x-3 bg-[#18181b] border border-white/10 rounded-lg px-3 py-1.5 shadow-sm hover:border-white/20 transition-colors cursor-pointer min-w-[200px] h-[34px]"
            :class="showDatePicker ? 'border-purple-500/50 ring-1 ring-purple-500/50' : ''"
          >
            <div class="flex items-center space-x-2">
              <Calendar class="w-4 h-4 shrink-0 transition-colors" :class="showDatePicker ? 'text-purple-400' : 'text-zinc-400'" />
              <span class="text-sm text-zinc-200">{{ getDateRangeDisplay() }}</span>
            </div>
            <ChevronDown class="w-4 h-4 text-zinc-400 shrink-0 transition-transform" :class="showDatePicker ? 'rotate-180 text-zinc-200' : ''" />
          </div>
          
          <div v-if="showDatePicker" class="absolute top-full left-0 mt-2 p-3 bg-[#18181b] border border-white/10 rounded-xl shadow-2xl z-50 w-[300px] shadow-purple-500/10 flex flex-col gap-3">
            <div class="grid grid-cols-3 gap-2">
              <button @click="setPreset('today')" class="px-2 py-1.5 text-xs rounded bg-white/5 hover:bg-white/10 text-zinc-300 transition-colors">今天</button>
              <button @click="setPreset('yesterday')" class="px-2 py-1.5 text-xs rounded bg-white/5 hover:bg-white/10 text-zinc-300 transition-colors">昨天</button>
              <button @click="setPreset('7days')" class="px-2 py-1.5 text-xs rounded bg-white/5 hover:bg-white/10 text-zinc-300 transition-colors">最近 7 天</button>
              <button @click="setPreset('30days')" class="px-2 py-1.5 text-xs rounded bg-white/5 hover:bg-white/10 text-zinc-300 transition-colors">最近 30 天</button>
              <button @click="setPreset('thisMonth')" class="px-2 py-1.5 text-xs rounded bg-white/5 hover:bg-white/10 text-zinc-300 transition-colors">本月</button>
              <button @click="setPreset('all')" class="px-2 py-1.5 text-xs rounded bg-white/5 hover:bg-white/10 text-zinc-300 transition-colors">全部时间</button>
            </div>
            
            <div class="h-px bg-white/10 w-full"></div>
            
            <div class="text-xs text-zinc-500 font-medium px-1">自定义时间 (YYYY-MM-DD)</div>
            <div class="flex items-center space-x-2">
              <input type="text" v-model="manualStart" placeholder="开始日期" class="w-full bg-black/50 border border-white/10 rounded px-2 py-1.5 text-xs text-center text-white focus:outline-none focus:border-purple-500/50 transition-colors" />
              <span class="text-zinc-500">-</span>
              <input type="text" v-model="manualEnd" placeholder="结束日期" class="w-full bg-black/50 border border-white/10 rounded px-2 py-1.5 text-xs text-center text-white focus:outline-none focus:border-purple-500/50 transition-colors" />
            </div>
            
            <button @click="applyDates" class="w-full py-2 bg-purple-600 hover:bg-purple-700 text-white text-xs font-medium rounded-lg transition-colors shadow-lg shadow-purple-500/20">
              确定
            </button>
          </div>
        </div>
        
        <!-- Custom API Key Dropdown -->
        <div class="relative">
          <div v-if="showKeyDropdown" class="fixed inset-0 z-40" @click="showKeyDropdown = false"></div>
          <div 
            @click="showKeyDropdown = !showKeyDropdown"
            class="relative z-40 flex items-center justify-between space-x-2 bg-[#18181b] border border-white/10 rounded-lg pl-3 pr-2 py-1.5 shadow-sm hover:border-white/20 transition-colors cursor-pointer min-w-[180px] h-[34px]"
            :class="showKeyDropdown ? 'border-purple-500/50 ring-1 ring-purple-500/50' : ''"
          >
            <div class="flex items-center space-x-2 overflow-hidden">
              <Key class="w-4 h-4 shrink-0 transition-colors" :class="showKeyDropdown ? 'text-purple-400' : 'text-zinc-400'" />
              <span class="text-sm text-zinc-200 truncate">{{ getSelectedKeyName() }}</span>
            </div>
            <ChevronDown class="w-4 h-4 text-zinc-400 shrink-0 transition-transform" :class="showKeyDropdown ? 'rotate-180 text-zinc-200' : ''" />
          </div>
          
          <div v-if="showKeyDropdown" class="absolute top-full left-0 mt-2 w-full bg-[#18181b] border border-white/10 rounded-xl shadow-2xl z-50 py-1 max-h-60 overflow-y-auto custom-scrollbar shadow-purple-500/10">
            <div 
              class="px-3 py-2 hover:bg-white/10 cursor-pointer text-sm transition-colors flex items-center"
              :class="filters.api_key_id === null ? 'text-purple-400 font-medium' : 'text-zinc-400'"
              @click="filters.api_key_id = null; showKeyDropdown = false"
            >
              所有 API 密钥
            </div>
            <div 
              v-for="k in availableKeys" :key="k.id"
              class="px-3 py-2 hover:bg-white/10 cursor-pointer text-sm transition-colors flex items-center truncate"
              :class="filters.api_key_id === k.id ? 'text-purple-400 font-medium' : 'text-zinc-200'"
              @click="filters.api_key_id = k.id; showKeyDropdown = false"
            >
              {{ k.name || k.key.substring(0, 8) + '...' }}
            </div>
          </div>
        </div>

        <button @click="handleFilter" class="px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white text-sm font-medium rounded-lg transition shadow-lg shadow-purple-500/20 disabled:opacity-50" :disabled="loading">
          查询
        </button>
      </div>
      
      <div class="flex space-x-2">
        <button class="px-3 py-2 bg-white/5 border border-white/10 text-white text-sm font-medium rounded-lg hover:bg-white/10 transition">
          Export CSV
        </button>
      </div>
    </div>

    <!-- Stats Cards -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      <template v-if="statsLoading">
        <div v-for="i in 4" :key="i" class="glass-panel p-5 h-[104px] animate-pulse flex items-center space-x-4">
          <div class="w-10 h-10 bg-white/5 rounded-xl"></div>
          <div class="flex-1 space-y-2">
            <div class="h-3 bg-white/5 rounded w-1/3"></div>
            <div class="h-6 bg-white/10 rounded w-1/2"></div>
            <div class="h-2 bg-white/5 rounded w-3/4"></div>
          </div>
        </div>
      </template>
      <template v-else-if="stats">
        <!-- Requests -->
        <div class="glass-panel p-5 flex items-start space-x-4 group hover:border-blue-500/30 transition-colors">
          <div class="w-10 h-10 rounded-xl bg-blue-500/10 text-blue-400 flex items-center justify-center shrink-0 group-hover:bg-blue-500/20 transition-colors">
            <FileText class="w-5 h-5" />
          </div>
          <div class="flex-1 min-w-0">
            <p class="text-xs text-zinc-400 font-medium mb-1">总请求数</p>
            <h3 class="text-xl font-bold text-white mb-1 truncate">{{ stats.total_requests.toLocaleString() }}</h3>
            <p class="text-xs text-zinc-500 mt-1">所选周期内</p>
          </div>
        </div>
        
        <!-- Tokens -->
        <div class="glass-panel p-5 flex items-start space-x-4 group hover:border-amber-500/30 transition-colors">
          <div class="w-10 h-10 rounded-xl bg-amber-500/10 text-amber-400 flex items-center justify-center shrink-0 group-hover:bg-amber-500/20 transition-colors">
            <Box class="w-5 h-5" />
          </div>
          <div class="flex-1 min-w-0">
            <p class="text-xs text-zinc-400 font-medium mb-1">总 Token</p>
            <h3 class="text-xl font-bold text-white mb-1 truncate">{{ formatNumberKMB(stats.total_tokens) }}</h3>
            <div class="flex flex-wrap gap-x-2 gap-y-0.5 text-[11px] text-zinc-500 mt-1.5" :title="`输入 ${formatNumberKMB(stats.total_input_tokens)} · 输出 ${formatNumberKMB(stats.total_output_tokens)} · 命中缓存 ${formatNumberKMB(stats.total_cache_read_tokens)} · 缓存写入 ${formatNumberKMB(stats.total_cache_creation_tokens)}`">
              <span>入 <span class="text-white">{{ formatNumberKMB(stats.total_input_tokens) }}</span></span>
              <span class="text-zinc-600">·</span>
              <span>出 <span class="text-white">{{ formatNumberKMB(stats.total_output_tokens) }}</span></span>
              <span class="text-zinc-600">·</span>
              <span>存 <span class="text-sky-400">{{ formatNumberKMB(stats.total_cache_read_tokens) }}</span></span>
              <span class="text-zinc-600">·</span>
              <span>写 <span class="text-amber-400">{{ formatNumberKMB(stats.total_cache_creation_tokens) }}</span></span>
            </div>
            <div v-if="((stats.total_cache_read_tokens || 0) + (stats.total_input_tokens || 0) + (stats.total_cache_creation_tokens || 0)) > 0" class="flex flex-wrap items-center gap-x-1.5 mt-1 text-[11px] text-zinc-500">
              <span>命中率: <span class="text-white">{{ formatNumberKMB(stats.total_cache_read_tokens) }} / {{ formatNumberKMB((stats.total_cache_read_tokens || 0) + (stats.total_input_tokens || 0) + (stats.total_cache_creation_tokens || 0)) }}</span></span>
              <span class="text-sky-400 font-mono bg-sky-500/10 px-1 py-0.5 rounded">{{ (((stats.total_cache_read_tokens || 0) / ((stats.total_cache_read_tokens || 0) + (stats.total_input_tokens || 0) + (stats.total_cache_creation_tokens || 0))) * 100).toFixed(1) }}%</span>
            </div>
          </div>
        </div>

        <!-- Cost -->
        <div class="glass-panel p-5 flex items-start space-x-4 group hover:border-green-500/30 transition-colors">
          <div class="w-10 h-10 rounded-xl bg-green-500/10 text-green-400 flex items-center justify-center shrink-0 group-hover:bg-green-500/20 transition-colors">
            <DollarSign class="w-5 h-5" />
          </div>
          <div class="flex-1 min-w-0">
            <p class="text-xs text-zinc-400 font-medium mb-1">估算费用</p>
            <h3 class="text-xl font-bold text-green-400 mb-1 truncate">${{ (stats.total_actual_cost || 0).toFixed(4) }}</h3>
            <p class="text-xs text-zinc-500 mt-1">美元 / 平台实际扣费</p>
          </div>
        </div>

        <!-- Duration -->
        <div class="glass-panel p-5 flex items-start space-x-4 group hover:border-purple-500/30 transition-colors">
          <div class="w-10 h-10 rounded-xl bg-purple-500/10 text-purple-400 flex items-center justify-center shrink-0 group-hover:bg-purple-500/20 transition-colors">
            <Timer class="w-5 h-5" />
          </div>
          <div class="flex-1 min-w-0">
            <p class="text-xs text-zinc-400 font-medium mb-1">平均耗时</p>
            <h3 class="text-xl font-bold text-white mb-1 truncate">{{ stats.average_duration_ms > 0 ? (stats.average_duration_ms / 1000).toFixed(2) : '0.00' }}s</h3>
            <p class="text-xs text-zinc-500 mt-1">单次请求</p>
          </div>
        </div>
      </template>
    </div>

    <div class="glass-panel overflow-hidden">
      <div v-if="loading" class="w-full">
        <table class="w-full text-left border-collapse">
          <thead>
            <tr class="border-b border-white/10 bg-black/20">
              <th v-for="i in 7" :key="i" class="px-6 py-4"><div class="h-4 bg-white/10 rounded w-16"></div></th>
            </tr>
          </thead>
          <tbody class="divide-y divide-white/5">
            <tr v-for="i in 5" :key="i" class="animate-pulse">
              <td v-for="j in 7" :key="j" class="px-6 py-4">
                <div class="h-4 bg-white/5 rounded" :class="j === 3 ? 'w-32' : 'w-24'"></div>
                <div v-if="j === 3 || j === 4 || j === 5" class="h-3 bg-white/5 rounded w-16 mt-2"></div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      
      <div v-else-if="logs.length === 0" class="p-12 text-center">
        <div class="w-16 h-16 mx-auto bg-white/5 rounded-full flex items-center justify-center mb-4">
          <Activity class="w-8 h-8 text-zinc-500" />
        </div>
        <h3 class="text-lg font-medium text-white mb-2">No usage logs</h3>
        <p class="text-zinc-400 mb-6">No requests found in this time period.</p>
      </div>

      <div v-else class="w-full overflow-x-auto">
        <table class="w-full min-w-[980px] text-left border-collapse">
          <thead>
            <tr class="border-b border-white/10 bg-black/20">
              <th class="px-6 py-4 text-sm font-medium text-zinc-400">时间 (北京时间)</th>
              <th class="px-6 py-4 text-sm font-medium text-zinc-400">模型</th>
              <th class="px-6 py-4 text-sm font-medium text-zinc-400">分组</th>
              <th class="px-6 py-4 text-sm font-medium text-zinc-400">Tokens</th>
              <th class="px-6 py-4 text-sm font-medium text-zinc-400">倍率/类型</th>
              <th class="px-6 py-4 text-sm font-medium text-zinc-400">耗时</th>
              <th class="px-6 py-4 text-sm font-medium text-zinc-400">消费 ($)</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-white/5">
            <tr v-for="log in logs" :key="log.id" class="hover:bg-white/5 transition-colors">
              <td class="px-6 py-4 text-sm text-zinc-300">
                {{ formatBeijingTime(log.created_at) }}
              </td>
              <td class="px-6 py-4 text-sm text-white font-medium">
                {{ log.model || 'N/A' }}
              </td>
              <td class="px-6 py-4 text-sm">
                <span class="inline-flex max-w-[160px] items-center rounded border border-white/10 bg-white/5 px-2 py-1 text-xs font-medium text-zinc-300">
                  <span class="truncate">{{ formatUsageGroup(log) }}</span>
                </span>
              </td>
	              <td class="px-6 py-4">
	                <div class="flex flex-col">
	                  <span class="text-sm text-zinc-500 font-mono flex flex-wrap items-center gap-x-2 gap-y-1">
	                    <span class="inline-flex items-center" title="输入 Token">
	                      <ArrowDownToLine class="w-3.5 h-3.5 mr-1 text-green-400/80" /> {{ formatTokenCount(log.input_tokens) }}
	                    </span>
		                    <span class="inline-flex items-center" title="输出 Token">
		                      <ArrowUpFromLine class="w-3.5 h-3.5 mr-1 text-purple-400/80" /> {{ formatTokenCount(log.output_tokens) }}
		                    </span>
		                    <span v-if="hasCacheReadTokens(log)" class="inline-flex items-center text-sky-400" title="cache read tokens">
		                      <Box class="w-3.5 h-3.5 mr-1 text-sky-400/80" /> {{ formatCacheTokenCount(log.cache_read_tokens) }}
		                    </span>
		                    <span v-if="(log.cache_creation_5m_tokens || 0) > 0" class="inline-flex items-center text-amber-400" title="5m cache write tokens">
		                      <Timer class="w-3.5 h-3.5 mr-1 text-amber-400/80" /> 5m {{ formatCacheTokenCount(log.cache_creation_5m_tokens) }}
		                    </span>
		                    <span v-if="(log.cache_creation_1h_tokens || 0) > 0" class="inline-flex items-center text-orange-400" title="1h cache write tokens">
		                      <Timer class="w-3.5 h-3.5 mr-1 text-orange-400/80" /> 1h {{ formatCacheTokenCount(log.cache_creation_1h_tokens) }}
		                    </span>
		                    <span v-if="hasAggregateCacheCreationTokens(log)" class="inline-flex items-center text-amber-400" title="cache write tokens">
		                      <Timer class="w-3.5 h-3.5 mr-1 text-amber-400/80" /> {{ formatCacheTokenCount(log.cache_creation_tokens) }}
		                    </span>
		                  </span>
	                  <span class="mt-1 text-xs text-zinc-500 font-mono">{{ getUsageLogTotalTokens(log).toLocaleString() }} 总计</span>
		                </div>
	              </td>
              <td class="px-6 py-4">
                <div class="flex flex-col">
                  <span class="text-sm text-purple-400 font-medium">{{ log.rate_multiplier || 1 }}x</span>
                  <span class="text-xs text-zinc-500 uppercase">{{ log.stream ? '流式' : '同步' }}</span>
                </div>
              </td>
              <td class="px-6 py-4">
                <div class="flex flex-col justify-center">
                  <span v-if="log.first_token_ms != null && log.first_token_ms > 0" class="text-xs text-zinc-500 font-mono mb-1">首字: {{ formatDuration(log.first_token_ms) }}</span>
                  <span class="text-sm text-zinc-300 font-mono">总计: {{ formatDuration(log.duration_ms) }}</span>
                </div>
              </td>
              <td class="px-6 py-4 text-sm font-mono" :class="log.actual_cost > 0 ? 'text-green-400' : 'text-zinc-300'">
                <div class="flex items-center justify-end w-full">
                  <span>{{ log.actual_cost ? Number(log.actual_cost).toFixed(6) : '0' }}</span>
                  <Info 
                    class="w-3.5 h-3.5 ml-1.5 text-zinc-500 cursor-help hover:text-white transition-colors" 
                    @mouseenter="(e) => showTooltip(e, log)"
                    @mouseleave="hideTooltip"
                  />
                </div>
              </td>
            </tr>
          </tbody>
        </table>

        <!-- Pagination -->
        <div v-if="totalPages > 1" class="px-6 py-4 border-t border-white/5 flex items-center justify-between bg-black/20">
          <div class="text-sm text-zinc-400">
            共 <span class="text-white font-medium">{{ totalLogs }}</span> 条记录
          </div>
          <div class="flex items-center space-x-2">
            <button 
              @click="page > 1 ? (page--, fetchLogs()) : null"
              :disabled="page === 1"
              class="px-3 py-1.5 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg text-sm text-white disabled:opacity-50 transition-colors"
            >
              上一页
            </button>
            <div class="px-3 py-1.5 text-sm text-zinc-400 font-medium">
              {{ page }} / {{ totalPages }}
            </div>
            <button 
              @click="page < totalPages ? (page++, fetchLogs()) : null"
              :disabled="page === totalPages"
              class="px-3 py-1.5 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg text-sm text-white disabled:opacity-50 transition-colors"
            >
              下一页
            </button>
          </div>
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
import { ref, onMounted, computed } from 'vue'
import { Activity, Info, ArrowDownToLine, ArrowUpFromLine, Calendar, Key, ChevronDown, FileText, Box, DollarSign, Timer } from '@lucide/vue'
import { list as getUsageLogs, getStatsByDateRange } from '@/api/usage'
import { list as getKeysList } from '@/api/keys'
import { formatUsageGroup } from '@/utils/usageGroup'

const logs = ref<any[]>([])
const loading = ref(true)
const page = ref(1)
const pageSize = ref(20)
const totalLogs = ref(0)
const totalPages = computed(() => Math.max(1, Math.ceil(totalLogs.value / pageSize.value)))

const formatNumberKMB = (num: number | undefined) => {
  if (num === null || num === undefined) return '0'
  if (num >= 1_000_000_000) return (num / 1_000_000_000).toFixed(2).replace(/\.00$/, '') + 'B'
  if (num >= 1_000_000) return (num / 1_000_000).toFixed(2).replace(/\.00$/, '') + 'M'
  if (num >= 1_000) return (num / 1_000).toFixed(2).replace(/\.00$/, '') + 'K'
  return num.toString()
}

const stats = ref<any>(null)
const statsLoading = ref(false)

const fetchStats = async () => {
  statsLoading.value = true
  try {
    const res = await getStatsByDateRange(filters.value.start_date, filters.value.end_date, filters.value.api_key_id === null ? undefined : filters.value.api_key_id)
    stats.value = res
  } catch (err) {
    console.error("Failed to load stats", err)
  } finally {
    statsLoading.value = false
  }
}

const availableKeys = ref<any[]>([])
const filters = ref({
  start_date: '',
  end_date: '',
  api_key_id: null as number | null
})

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

// Dropdown & Picker State
const showKeyDropdown = ref(false)
const showDatePicker = ref(false)
const manualStart = ref('')
const manualEnd = ref('')

const getSelectedKeyName = () => {
  if (filters.value.api_key_id === null) return '所有 API 密钥'
  const key = availableKeys.value.find(k => k.id === filters.value.api_key_id)
  if (!key) return '所有 API 密钥'
  return key.name || key.key.substring(0, 8) + '...'
}

const getDateRangeDisplay = () => {
  if (!filters.value.start_date && !filters.value.end_date) return '全部时间'
  if (filters.value.start_date && filters.value.end_date) return `${filters.value.start_date} 至 ${filters.value.end_date}`
  if (filters.value.start_date) return `${filters.value.start_date} 之后`
  if (filters.value.end_date) return `${filters.value.end_date} 之前`
  return '全部时间'
}

const formatDateStr = (d: Date) => {
  const year = d.getFullYear()
  const month = String(d.getMonth() + 1).padStart(2, '0')
  const day = String(d.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

const setPreset = (preset: string) => {
  const today = new Date()
  let start = ''
  let end = ''
  
  if (preset === 'today') {
    start = formatDateStr(today)
    end = formatDateStr(today)
  } else if (preset === 'yesterday') {
    const yesterday = new Date(today)
    yesterday.setDate(today.getDate() - 1)
    start = formatDateStr(yesterday)
    end = formatDateStr(yesterday)
  } else if (preset === '7days') {
    const past = new Date(today)
    past.setDate(today.getDate() - 6)
    start = formatDateStr(past)
    end = formatDateStr(today)
  } else if (preset === '30days') {
    const past = new Date(today)
    past.setDate(today.getDate() - 29)
    start = formatDateStr(past)
    end = formatDateStr(today)
  } else if (preset === 'thisMonth') {
    const firstDay = new Date(today.getFullYear(), today.getMonth(), 1)
    start = formatDateStr(firstDay)
    end = formatDateStr(today)
  } else if (preset === 'all') {
    start = ''
    end = ''
  }
  
  filters.value.start_date = start
  filters.value.end_date = end
  manualStart.value = start
  manualEnd.value = end
  showDatePicker.value = false
}

const applyDates = () => {
  filters.value.start_date = manualStart.value
  filters.value.end_date = manualEnd.value
  showDatePicker.value = false
}

const handleFilter = () => {
  page.value = 1
  fetchLogs()
  fetchStats()
}

const fetchKeys = async () => {
  try {
    const res = await getKeysList(1, 100)
    availableKeys.value = res.items || []
  } catch (err) {
    console.error("Failed to load keys", err)
  }
}

const fetchLogs = async () => {
  loading.value = true
  try {
    const apiFilters: any = {}
    if (filters.value.api_key_id) apiFilters.api_key_id = filters.value.api_key_id
    if (filters.value.start_date) apiFilters.start_date = filters.value.start_date
    if (filters.value.end_date) apiFilters.end_date = filters.value.end_date
    
    const res = await getUsageLogs(page.value, pageSize.value, apiFilters)
    logs.value = res.items || []
    totalLogs.value = res.total || 0
  } catch (err) {
    console.error("Failed to load usage logs", err)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchKeys()
  fetchLogs()
  fetchStats()
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

const formatTokenCount = (tokens: number | null | undefined) => {
  return (tokens || 0).toLocaleString()
}

const formatCacheTokenCount = (tokens: number | null | undefined) => {
  return formatNumberKMB(tokens || 0)
}

const hasSplitCacheCreationTokens = (log: any) => {
  return (log.cache_creation_5m_tokens || 0) > 0 || (log.cache_creation_1h_tokens || 0) > 0
}

const hasCacheReadTokens = (log: any) => {
  return (log.cache_read_tokens || 0) > 0
}

const hasAggregateCacheCreationTokens = (log: any) => {
  return !hasSplitCacheCreationTokens(log) && (log.cache_creation_tokens || 0) > 0
}

const getUsageLogTotalTokens = (log: any) => {
  return (log.input_tokens || 0) + (log.output_tokens || 0) + (log.cache_read_tokens || 0) + (log.cache_creation_tokens || 0)
}

const formatDuration = (ms: number | null | undefined) => {
  if (!ms) return '-'
  if (ms >= 1000) return (ms / 1000).toFixed(2) + 's'
  return ms + 'ms'
}
</script>
