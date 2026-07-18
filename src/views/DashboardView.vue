<template>
  <div class="space-y-6">
    <!-- Loading Skeletons -->
    <div v-if="loading" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-2">
      <!-- 4 Main Cards -->
      <div v-for="i in 4" :key="`main-${i}`" class="glass-panel p-5 h-[130px] animate-pulse flex flex-col justify-between">
        <div class="flex justify-between items-start mb-3">
          <div class="w-8 h-8 bg-white/5 rounded-lg border border-white/5"></div>
          <div class="w-16 h-5 bg-white/5 rounded-full border border-white/5"></div>
        </div>
        <div>
          <div class="w-24 h-3 bg-white/5 rounded mb-2"></div>
          <div class="w-32 h-7 bg-white/10 rounded"></div>
        </div>
      </div>
      <!-- 4 Mini Cards -->
      <div v-for="i in 4" :key="`mini-${i}`" class="glass-panel p-4 h-[76px] animate-pulse flex items-center justify-between">
        <div>
          <div class="w-20 h-3 bg-white/5 rounded mb-2"></div>
          <div class="w-12 h-6 bg-white/10 rounded"></div>
        </div>
        <div class="w-8 h-8 bg-white/5 rounded-lg border border-white/5"></div>
      </div>
    </div>

    <!-- Header Stats -->
    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-2">
      
      <!-- Balance Card -->
      <div class="glass-panel p-5 flex flex-col justify-between relative overflow-hidden group hover:border-purple-500/50 transition-colors">
        <div class="absolute top-0 right-0 w-24 h-24 bg-purple-500/10 rounded-full blur-2xl group-hover:bg-purple-500/20 transition-all duration-500 -translate-y-1/2 translate-x-1/2"></div>
        <div class="flex justify-between items-start mb-3">
          <div class="p-2 bg-white/5 rounded-lg border border-white/5 group-hover:border-purple-500/30 transition-colors">
            <CreditCard class="w-4 h-4 text-purple-400" />
          </div>
          <span class="text-xs font-medium px-2 py-1 bg-green-500/10 text-green-400 rounded-full border border-green-500/20">充裕</span>
        </div>
        <div>
          <p class="text-xs text-zinc-400 font-medium mb-1">当前余额 (Quota)</p>
          <div class="flex items-baseline space-x-1">
            <h2 class="text-2xl font-bold tracking-tight text-white">¥{{ (authStore.user?.balance || 0).toLocaleString() }}</h2>
          </div>
        </div>
      </div>

      <!-- Actual Cost Card -->
      <div class="glass-panel p-5 flex flex-col justify-between relative overflow-hidden group hover:border-pink-500/50 transition-colors">
        <div class="absolute top-0 right-0 w-24 h-24 bg-pink-500/10 rounded-full blur-2xl group-hover:bg-pink-500/20 transition-all duration-500 -translate-y-1/2 translate-x-1/2"></div>
        <div class="flex justify-between items-start mb-3">
          <div class="p-2 bg-white/5 rounded-lg border border-white/5 group-hover:border-pink-500/30 transition-colors">
            <Coins class="w-4 h-4 text-pink-400" />
          </div>
          <span v-if="stats?.total_actual_cost > 0" class="text-xs font-medium px-2 py-1 bg-pink-500/10 text-pink-400 rounded-full border border-pink-500/20">总计: ${{ (stats?.total_actual_cost || 0).toFixed(4) }}</span>
        </div>
        <div>
          <p class="text-xs text-zinc-400 font-medium mb-1">今日消费 (USD)</p>
          <div class="flex items-baseline space-x-1">
            <h2 class="text-2xl font-bold tracking-tight text-white">${{ (stats?.today_actual_cost || 0).toFixed(4) }}</h2>
          </div>
        </div>
      </div>

      <!-- Requests Card -->
      <div class="glass-panel p-5 flex flex-col justify-between relative overflow-hidden group hover:border-blue-500/50 transition-colors">
        <div class="absolute top-0 right-0 w-24 h-24 bg-blue-500/10 rounded-full blur-2xl group-hover:bg-blue-500/20 transition-all duration-500 -translate-y-1/2 translate-x-1/2"></div>
        <div class="flex justify-between items-start mb-3">
          <div class="p-2 bg-white/5 rounded-lg border border-white/5 group-hover:border-blue-500/30 transition-colors">
            <Activity class="w-4 h-4 text-blue-400" />
          </div>
          <span v-if="stats?.total_requests > 0" class="text-xs font-medium px-2 py-1 bg-blue-500/10 text-blue-400 rounded-full border border-blue-500/20">总计: {{ (stats?.total_requests || 0).toLocaleString() }}</span>
        </div>
        <div>
          <p class="text-xs text-zinc-400 font-medium mb-1">今日请求数</p>
          <div class="flex items-baseline space-x-1">
            <h2 class="text-2xl font-bold tracking-tight text-white">{{ (stats?.today_requests || 0).toLocaleString() }}</h2>
          </div>
        </div>
      </div>

      <!-- Tokens Card -->
      <div class="glass-panel p-5 flex flex-col justify-between relative overflow-hidden group hover:border-amber-500/50 transition-colors">
        <div class="absolute top-0 right-0 w-24 h-24 bg-amber-500/10 rounded-full blur-2xl group-hover:bg-amber-500/20 transition-all duration-500 -translate-y-1/2 translate-x-1/2"></div>
        <div class="flex justify-between items-start mb-3">
          <div class="p-2 bg-white/5 rounded-lg border border-white/5 group-hover:border-amber-500/30 transition-colors">
            <Zap class="w-4 h-4 text-amber-400" />
          </div>
          <span v-if="stats?.total_tokens > 0" class="text-xs font-medium px-2 py-1 bg-amber-500/10 text-amber-400 rounded-full border border-amber-500/20">总计: {{ formatNumberKMB(stats?.total_tokens) }}</span>
        </div>
        <div>
          <div class="flex flex-col sm:flex-row sm:items-center justify-between mb-1.5 sm:mb-0.5 gap-1 sm:gap-0">
            <p class="text-xs text-zinc-400 font-medium">今日消耗 Tokens</p>
            <div class="text-[10px] sm:text-xs text-zinc-500 flex flex-wrap gap-2 sm:gap-0 sm:space-x-2 font-medium">
              <span title="输入Tokens">入: {{ formatNumberKMB(stats?.today_input_tokens) }}</span>
              <span title="输出Tokens">出: {{ formatNumberKMB(stats?.today_output_tokens) }}</span>
              <span title="缓存Tokens">缓: {{ formatNumberKMB((stats?.today_cache_creation_tokens || 0) + (stats?.today_cache_read_tokens || 0)) }}</span>
            </div>
          </div>
          <div class="flex items-baseline space-x-1">
            <h2 class="text-2xl font-bold tracking-tight text-white break-all">{{ formatNumberKMB(stats?.today_tokens) }}</h2>
          </div>
        </div>
      </div>

      <!-- Second Row Stats (Mini Cards) -->
      <div class="glass-panel p-4 flex items-center justify-between group hover:border-indigo-500/50 transition-colors">
        <div>
          <p class="text-xs text-zinc-400 mb-1">活跃 API 密钥</p>
          <div class="flex items-end space-x-2">
            <span class="text-lg font-bold text-white">{{ stats?.active_api_keys || 0 }}</span>
            <span class="text-xs text-zinc-500 mb-1">/ {{ stats?.total_api_keys || 0 }}</span>
          </div>
        </div>
        <div class="p-2 bg-indigo-500/10 border border-indigo-500/20 rounded-lg text-indigo-400">
          <Key class="w-4 h-4" />
        </div>
      </div>

      <div class="glass-panel p-4 flex items-center justify-between group hover:border-cyan-500/50 transition-colors">
        <div>
          <p class="text-xs text-zinc-400 mb-1">平均响应耗时</p>
          <div class="flex items-end space-x-2">
            <span class="text-lg font-bold text-white">{{ ((stats?.average_duration_ms || 0) / 1000).toFixed(2) }}</span>
            <span class="text-xs text-zinc-500 mb-1">s</span>
          </div>
        </div>
        <div class="p-2 bg-cyan-500/10 border border-cyan-500/20 rounded-lg text-cyan-400">
          <Timer class="w-4 h-4" />
        </div>
      </div>

      <div class="glass-panel p-4 flex items-center justify-between group hover:border-emerald-500/50 transition-colors">
        <div>
          <p class="text-xs text-zinc-400 mb-1">当前并发 (RPM)</p>
          <div class="flex items-end space-x-2">
            <span class="text-lg font-bold text-white">{{ stats?.rpm || 0 }}</span>
            <span class="text-xs text-zinc-500 mb-1">次/分</span>
          </div>
        </div>
        <div class="p-2 bg-emerald-500/10 border border-emerald-500/20 rounded-lg text-emerald-400">
          <TrendingUp class="w-4 h-4" />
        </div>
      </div>

      <div class="glass-panel p-4 flex items-center justify-between group hover:border-rose-500/50 transition-colors">
        <div>
          <p class="text-xs text-zinc-400 mb-1">当前流速 (TPM)</p>
          <div class="flex items-end space-x-2">
            <span class="text-lg font-bold text-white">{{ stats?.tpm || 0 }}</span>
            <span class="text-xs text-zinc-500 mb-1">tokens/分</span>
          </div>
        </div>
        <div class="p-2 bg-rose-500/10 border border-rose-500/20 rounded-lg text-rose-400">
          <Flame class="w-4 h-4" />
        </div>
      </div>
    </div>

    <!-- Main Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      
      <!-- Chart Section -->
      <div class="glass-panel lg:col-span-2 p-6 flex flex-col min-h-[400px]">
        <div class="flex items-center justify-between mb-6">
          <div>
            <h3 class="text-lg font-semibold text-white">API 调用走势</h3>
            <p class="text-sm text-zinc-400">过去 {{ currentRange === '30d' ? '30 天' : currentRange === '7d' ? '7 天' : '24 小时' }}的请求量</p>
          </div>
          <div class="flex space-x-2">
            <button @click="updateTrend('30d')" class="px-3 py-1.5 text-xs font-medium rounded-md transition-colors" :class="currentRange === '30d' ? 'bg-white/10 text-white border border-white/5' : 'text-zinc-400 hover:text-white hover:bg-white/5'">30天</button>
            <button @click="updateTrend('7d')" class="px-3 py-1.5 text-xs font-medium rounded-md transition-colors" :class="currentRange === '7d' ? 'bg-white/10 text-white border border-white/5' : 'text-zinc-400 hover:text-white hover:bg-white/5'">7天</button>
            <button @click="updateTrend('24h')" class="px-3 py-1.5 text-xs font-medium rounded-md transition-colors" :class="currentRange === '24h' ? 'bg-white/10 text-white border border-white/5' : 'text-zinc-400 hover:text-white hover:bg-white/5'">24小时</button>
          </div>
        </div>
        
        <!-- Real Chart using vue-chartjs -->
        <div class="flex-1 w-full bg-zinc-900/50 rounded-xl border border-white/5 relative overflow-hidden flex flex-col p-4">
          <div class="absolute inset-0 bg-[linear-gradient(to_right,#ffffff02_1px,transparent_1px),linear-gradient(to_bottom,#ffffff02_1px,transparent_1px)] bg-[size:4rem_4rem]"></div>
          <div class="relative w-full h-full min-h-[250px]" :class="{ 'opacity-50 pointer-events-none': isTrendLoading }">
            <Line v-if="trend?.trend" :data="chartData" :options="chartOptions" />
            <div v-else class="absolute inset-0 flex items-center justify-center text-zinc-500 text-sm">
              <span class="animate-pulse">加载走势数据中...</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Recent Activity / Side Panel -->
      <div class="glass-panel p-6 flex flex-col">
        <h3 class="text-lg font-semibold text-white mb-6">最近 API 密钥</h3>
        
        <div class="space-y-4 flex-1">
          <template v-if="loading">
            <div v-for="i in 3" :key="i" class="flex items-center space-x-4 p-3 rounded-xl border border-transparent animate-pulse">
              <div class="w-10 h-10 rounded-lg bg-white/5 border border-white/5"></div>
              <div class="flex-1">
                <div class="w-24 h-4 bg-white/10 rounded mb-2"></div>
                <div class="w-16 h-3 bg-white/5 rounded"></div>
              </div>
            </div>
          </template>
          <template v-else>
            <div v-for="key in recentKeys" :key="key.id" class="flex items-center justify-between p-3 rounded-xl hover:bg-white/5 border border-transparent hover:border-white/5 transition-colors cursor-pointer group">
              <div class="flex items-center space-x-4">
                <div class="w-10 h-10 rounded-lg bg-blue-500/10 border border-blue-500/20 flex items-center justify-center text-blue-400">
                  <Server class="w-5 h-5" />
                </div>
                <div>
                  <p class="text-sm font-medium text-white">{{ key.name }}</p>
                  <p class="text-xs text-zinc-500 font-mono mt-0.5">{{ key.key.substring(0, 8) }}...</p>
                </div>
              </div>
              <button class="opacity-0 group-hover:opacity-100 p-2 text-zinc-400 hover:text-white transition">
                <Copy class="w-4 h-4" />
              </button>
            </div>
            
            <div v-if="recentKeys.length === 0" class="text-center py-4 text-zinc-500 text-sm">
              暂无 API 密钥
            </div>
          </template>
        </div>

        <router-link to="/keys" class="w-full py-2.5 mt-4 text-sm font-medium text-white bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg transition-colors flex items-center justify-center">
          <Plus class="w-4 h-4 mr-2" />
          创建新密钥
        </router-link>
      </div>

<div class="lg:col-span-3 grid grid-cols-1 xl:grid-cols-2 gap-6 mt-6">
      <!-- Model Distribution -->
      <div class="glass-panel p-6 flex flex-col min-h-[300px]">
        <div class="flex flex-wrap items-center justify-between gap-4 mb-6">
          <h3 class="text-lg font-semibold text-white shrink-0">模型分布</h3>
          <div class="flex bg-black/40 rounded-lg p-1 border border-white/5">
            <button
              @click="modelSortMode = 'token'"
              class="px-3 py-1.5 text-xs font-medium rounded-md transition-all"
              :class="modelSortMode === 'token' ? 'bg-white/10 text-white shadow-sm' : 'text-zinc-500 hover:text-zinc-300'"
            >
              按 Token
            </button>
            <button
              @click="modelSortMode = 'cost'"
              class="px-3 py-1.5 text-xs font-medium rounded-md transition-all"
              :class="modelSortMode === 'cost' ? 'bg-white/10 text-white shadow-sm' : 'text-zinc-500 hover:text-zinc-300'"
            >
              按实际消费
            </button>
          </div>
        </div>
        <div v-if="loadingModels" class="flex-1 flex items-center justify-center">
          <span class="animate-pulse text-zinc-500 text-sm">加载模型分布中...</span>
        </div>
        <div v-else-if="modelStats.length === 0" class="flex-1 flex items-center justify-center text-zinc-500 text-sm">
          暂无数据
        </div>
        <div v-else class="flex flex-col lg:flex-row xl:flex-col 2xl:flex-row items-center gap-6 flex-1 min-h-0">
          <div class="relative h-48 w-48 shrink-0">
            <Doughnut :data="modelChartData" :options="doughnutOptions" />
          </div>
          <div class="distribution-table-shell custom-scrollbar">
            <table class="distribution-table">
              <colgroup>
                <col class="w-[36%]" />
                <col class="w-[15%]" />
                <col class="w-[16%]" />
                <col class="w-[17%]" />
                <col class="w-[16%]" />
              </colgroup>
              <thead>
                <tr>
                  <th>模型</th>
                  <th class="text-right">请求</th>
                  <th class="text-right">Token</th>
                  <th class="text-right">实际</th>
                  <th class="text-right">标准</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="model in sortedModelStats" :key="model.model">
                  <td class="distribution-name" :title="model.model">{{ model.model }}</td>
                  <td class="text-right">{{ formatRequestCount(model.requests) }}</td>
                  <td class="text-right">{{ formatNumberKMB(model.total_tokens) }}</td>
                  <td class="distribution-cost text-right">${{ model.actual_cost.toFixed(4) }}</td>
                  <td class="distribution-muted text-right">${{ model.cost.toFixed(4) }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <!-- Group Distribution (Admin Only) -->
      <div v-if="authStore.isAdmin" class="glass-panel p-6 flex flex-col min-h-[300px]">
        <div class="flex flex-wrap items-center justify-between gap-4 mb-6">
          <h3 class="text-lg font-semibold text-white shrink-0">分组使用分布</h3>
          <div class="flex bg-black/40 rounded-lg p-1 border border-white/5">
            <button
              @click="groupSortMode = 'token'"
              class="px-3 py-1.5 text-xs font-medium rounded-md transition-all"
              :class="groupSortMode === 'token' ? 'bg-white/10 text-white shadow-sm' : 'text-zinc-500 hover:text-zinc-300'"
            >
              按 Token
            </button>
            <button
              @click="groupSortMode = 'cost'"
              class="px-3 py-1.5 text-xs font-medium rounded-md transition-all"
              :class="groupSortMode === 'cost' ? 'bg-white/10 text-white shadow-sm' : 'text-zinc-500 hover:text-zinc-300'"
            >
              按实际消费
            </button>
          </div>
        </div>
        <div v-if="loadingGroups" class="flex-1 flex items-center justify-center">
          <span class="animate-pulse text-zinc-500 text-sm">加载分组分布中...</span>
        </div>
        <div v-else-if="groupStats.length === 0" class="flex-1 flex items-center justify-center text-zinc-500 text-sm">
          暂无数据
        </div>
        <div v-else class="flex flex-col lg:flex-row xl:flex-col 2xl:flex-row items-center gap-6 flex-1 min-h-0">
          <div class="relative h-48 w-48 shrink-0">
            <Doughnut :data="groupChartData" :options="doughnutOptions" />
          </div>
          <div class="distribution-table-shell custom-scrollbar">
            <table class="distribution-table">
              <colgroup>
                <col class="w-[36%]" />
                <col class="w-[15%]" />
                <col class="w-[16%]" />
                <col class="w-[17%]" />
                <col class="w-[16%]" />
              </colgroup>
              <thead>
                <tr>
                  <th>分组</th>
                  <th class="text-right">请求</th>
                  <th class="text-right">Token</th>
                  <th class="text-right">实际</th>
                  <th class="text-right">标准</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="group in sortedGroupStats" :key="group.group_id">
                  <td class="distribution-name" :title="group.group_name">{{ group.group_name }}</td>
                  <td class="text-right">{{ formatRequestCount(group.requests) }}</td>
                  <td class="text-right">{{ formatNumberKMB(groupTokenTotal(group)) }}</td>
                  <td class="distribution-cost text-right">${{ group.actual_cost.toFixed(4) }}</td>
                  <td class="distribution-muted text-right">${{ group.cost.toFixed(4) }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { Line, Doughnut } from 'vue-chartjs'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler,
  ArcElement
} from 'chart.js'

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler,
  ArcElement
)
import { 
  CreditCard, 
  Zap, 
  Activity, 
  Server, 
  Copy,
  Plus,
  Coins,
  Key,
  Timer,
  TrendingUp,
  Flame
} from '@lucide/vue'
import { usageAPI } from '@/api/usage'
import adminDashboardAPI from '@/api/admin/dashboard'
import { list as listKeys } from '@/api/keys'
import { useAuthStore } from '@/stores/auth'

const authStore = useAuthStore()
const stats = ref<any>(null)
const trend = ref<any>(null)
const recentKeys = ref<any[]>([])
const loading = ref(true)

// Stats
const modelStats = ref<any[]>([])
const loadingModels = ref(false)
const modelSortMode = ref<'token' | 'cost'>('token')

const groupStats = ref<any[]>([])
const loadingGroups = ref(false)
const groupSortMode = ref<'token' | 'cost'>('token')

const formatNumberKMB = (num: number | undefined) => {
  if (num === null || num === undefined) return '0'
  if (num >= 1_000_000_000) return (num / 1_000_000_000).toFixed(1).replace(/\.0$/, '') + 'B'
  if (num >= 1_000_000) return (num / 1_000_000).toFixed(1).replace(/\.0$/, '') + 'M'
  if (num >= 1_000) return (num / 1_000).toFixed(1).replace(/\.0$/, '') + 'K'
  return num.toString()
}

const formatRequestCount = (num: number | undefined) => {
  return (num || 0).toLocaleString()
}

const currentRange = ref('30d')
const isTrendLoading = ref(false)

const chartData = computed(() => {
  if (!trend.value || !trend.value.trend) return { labels: [], datasets: [] }
  
  return {
    labels: trend.value.trend.map((t: any) => {
      const date = new Date(t.date)
      if (currentRange.value === '24h') {
        return `${String(date.getHours()).padStart(2, '0')}:00`
      }
      return `${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`
    }),
    datasets: [
      {
        label: 'API 请求量',
        data: trend.value.trend.map((t: any) => t.requests),
        borderColor: '#a855f7',
        backgroundColor: (context: any) => {
          const ctx = context.chart.ctx
          const gradient = ctx.createLinearGradient(0, 0, 0, 300)
          gradient.addColorStop(0, 'rgba(168, 85, 247, 0.4)')
          gradient.addColorStop(1, 'rgba(168, 85, 247, 0.0)')
          return gradient
        },
        fill: true,
        tension: 0.4,
        borderWidth: 2,
        pointBackgroundColor: '#a855f7',
        pointBorderColor: '#fff',
        pointBorderWidth: 1,
        pointRadius: 0,
        pointHoverRadius: 6,
        pointHitRadius: 10
      }
    ]
  }
})

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { display: false },
    tooltip: { 
      mode: 'index' as const, 
      intersect: false,
      backgroundColor: 'rgba(24, 24, 27, 0.9)',
      titleColor: '#a1a1aa',
      bodyColor: '#fff',
      borderColor: 'rgba(255,255,255,0.1)',
      borderWidth: 1,
      padding: 10,
      displayColors: false
    }
  },
  scales: {
    x: { 
      grid: { display: false },
      ticks: { color: '#71717a', maxTicksLimit: 7 }
    },
    y: { 
      grid: { color: 'rgba(255, 255, 255, 0.05)' },
      ticks: { color: '#71717a' },
      beginAtZero: true
    }
  },
  interaction: {
    mode: 'index' as const,
    intersect: false,
  }
}

const sortedModelStats = computed(() => {
  const models = [...modelStats.value]
  if (modelSortMode.value === 'token') {
    return models.sort((a: any, b: any) => b.total_tokens - a.total_tokens).slice(0, 5)
  } else {
    return models.sort((a: any, b: any) => b.actual_cost - a.actual_cost).slice(0, 5)
  }
})

const modelChartData = computed(() => {
  const stats = sortedModelStats.value
  if (!stats.length) return { labels: [], datasets: [] }
  const colors = ['#a855f7', '#3b82f6', '#10b981', '#f59e0b', '#ec4899']
  return {
    labels: stats.map((m: any) => m.model),
    datasets: [{
      data: stats.map((m: any) => modelSortMode.value === 'token' ? m.total_tokens : m.actual_cost),
      backgroundColor: colors.slice(0, stats.length),
      borderWidth: 0,
      hoverOffset: 4
    }]
  }
})

const sortedGroupStats = computed(() => {
  const groups = [...groupStats.value]
  if (groupSortMode.value === 'token') {
    return groups.sort((a: any, b: any) => groupTokenTotal(b) - groupTokenTotal(a)).slice(0, 5)
  } else {
    return groups.sort((a: any, b: any) => b.actual_cost - a.actual_cost).slice(0, 5)
  }
})

const groupTokenTotal = (group: any) => group.total_tokens ?? group.tokens ?? 0

const groupChartData = computed(() => {
  const stats = sortedGroupStats.value
  if (!stats.length) return { labels: [], datasets: [] }
  const colors = ['#3b82f6', '#10b981', '#f59e0b', '#ec4899', '#a855f7']
  return {
    labels: stats.map((g: any) => g.group_name || '默认分组'),
    datasets: [{
      data: stats.map((g: any) => groupSortMode.value === 'token' ? groupTokenTotal(g) : g.actual_cost),
      backgroundColor: colors.slice(0, stats.length),
      borderWidth: 0,
      hoverOffset: 4
    }]
  }
})

const doughnutOptions = {
  responsive: true,
  maintainAspectRatio: false,
  cutout: '70%',
  plugins: {
    legend: { display: false },
    tooltip: {
      backgroundColor: 'rgba(24, 24, 27, 0.9)',
      titleColor: '#a1a1aa',
      bodyColor: '#fff',
      borderColor: 'rgba(255,255,255,0.1)',
      borderWidth: 1,
      padding: 10
    }
  }
}

const getTrendDateParams = (range: string) => {
  const now = new Date()
  const startDate = new Date()
  let granularity: 'day' | 'hour' = 'day'
  
  if (range === '30d') {
    startDate.setTime(now.getTime() - 29 * 86400000)
  } else if (range === '7d') {
    startDate.setTime(now.getTime() - 6 * 86400000)
  } else if (range === '24h') {
    startDate.setHours(now.getHours() - 24)
    granularity = 'hour'
  }
  
  const pad = (n: number) => String(n).padStart(2, '0')
  const formatDate = (d: Date) => {
    const ymd = `${d.getFullYear()}-${pad(d.getMonth()+1)}-${pad(d.getDate())}`
    return ymd
  }

  // 后端 Dashboard 日期参数契约是 YYYY-MM-DD；24 小时视图只通过 granularity=hour 切换聚合粒度。
  return {
    start_date: formatDate(startDate),
    end_date: formatDate(now),
    granularity
  }
}

const updateTrend = async (range: string) => {
  if (isTrendLoading.value || currentRange.value === range) return
  currentRange.value = range
  isTrendLoading.value = true
  try {
    const params = getTrendDateParams(range)
    const [trendRes, modelsRes] = await Promise.all([
      usageAPI.getDashboardTrend(params),
      usageAPI.getDashboardModels(params)
    ])
    trend.value = trendRes
    modelStats.value = modelsRes.models || []
    if (authStore.isAdmin) {
      const adminRes = await adminDashboardAPI.getSnapshotV2({
        start_date: params.start_date,
        end_date: params.end_date,
        include_group_stats: true
      })
      groupStats.value = adminRes.groups || []
    }
  } catch (err) {
    console.error("Failed to update dashboard data", err)
  } finally {
    isTrendLoading.value = false
  }
}

onMounted(async () => {
  try {
    await authStore.checkAuth()
    const params = getTrendDateParams('30d')
    const [statsData, trendData, keysData, modelsData] = await Promise.all([
      usageAPI.getDashboardStats(),
      usageAPI.getDashboardTrend(params),
      listKeys(1, 4, { sort_by: 'created_at', sort_order: 'desc' }),
      usageAPI.getDashboardModels(params)
    ])
    stats.value = statsData
    trend.value = trendData
    modelStats.value = modelsData.models || []
    recentKeys.value = keysData.items || []
    if (authStore.isAdmin) {
      const adminRes = await adminDashboardAPI.getSnapshotV2({
        start_date: params.start_date,
        end_date: params.end_date,
        include_group_stats: true
      })
      groupStats.value = adminRes.groups || []
    }
  } catch (err) {
    console.error("Failed to load dashboard data", err)
  } finally {
    loading.value = false
  }
})
</script>

<style scoped>
.chart-path {
  stroke-dasharray: 1000;
  stroke-dashoffset: 1000;
  animation: drawLine 3s ease-out forwards;
}

@keyframes drawLine {
  to {
    stroke-dashoffset: 0;
  }
}

.distribution-table-shell {
  flex: 1 1 0%;
  min-width: 0;
  width: 100%;
  max-height: 12.75rem;
  overflow: auto;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 12px;
  background:
    linear-gradient(180deg, rgba(255, 255, 255, 0.045), rgba(255, 255, 255, 0.015)),
    rgba(8, 8, 10, 0.72);
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.04);
}

.distribution-table {
  width: 100%;
  min-width: 460px;
  table-layout: fixed;
  border-collapse: separate;
  border-spacing: 0;
  text-align: left;
  font-size: 12px;
  line-height: 1.2;
}

.distribution-table thead tr {
  position: sticky;
  top: 0;
  z-index: 10;
  background: rgba(14, 14, 17, 0.96);
  backdrop-filter: blur(12px);
  box-shadow: 0 1px 0 rgba(255, 255, 255, 0.08);
}

.distribution-table th {
  height: 36px;
  padding: 0 10px;
  color: #a1a1aa;
  font-size: 11px;
  font-weight: 600;
  white-space: nowrap;
}

.distribution-table td {
  height: 34px;
  padding: 0 10px;
  color: #a1a1aa;
  white-space: nowrap;
  border-top: 1px solid rgba(255, 255, 255, 0.055);
  vertical-align: middle;
}

.distribution-table tbody tr {
  transition: background-color 0.16s ease, color 0.16s ease;
}

.distribution-table tbody tr:hover {
  background: rgba(255, 255, 255, 0.035);
}

.distribution-name {
  overflow: hidden;
  color: #e4e4e7;
  font-weight: 600;
  text-overflow: ellipsis;
}

.distribution-cost {
  color: #00d492;
  font-weight: 700;
}

.distribution-muted {
  color: #71717a;
}
</style>
