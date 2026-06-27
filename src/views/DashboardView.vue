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
          <span v-if="stats?.today_actual_cost > 0" class="text-xs font-medium px-2 py-1 bg-pink-500/10 text-pink-400 rounded-full border border-pink-500/20">今日: ${{ (stats?.today_actual_cost || 0).toFixed(4) }}</span>
        </div>
        <div>
          <p class="text-xs text-zinc-400 font-medium mb-1">总计消费 (USD)</p>
          <div class="flex items-baseline space-x-1">
            <h2 class="text-2xl font-bold tracking-tight text-white">${{ (stats?.total_actual_cost || 0).toFixed(4) }}</h2>
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
          <span v-if="stats?.today_requests > 0" class="text-xs font-medium px-2 py-1 bg-blue-500/10 text-blue-400 rounded-full border border-blue-500/20">今日: {{ stats?.today_requests.toLocaleString() }}</span>
        </div>
        <div>
          <p class="text-xs text-zinc-400 font-medium mb-1">总请求数</p>
          <div class="flex items-baseline space-x-1">
            <h2 class="text-2xl font-bold tracking-tight text-white">{{ (stats?.total_requests || 0).toLocaleString() }}</h2>
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
          <span v-if="stats?.today_tokens > 0" class="text-xs font-medium px-2 py-1 bg-amber-500/10 text-amber-400 rounded-full border border-amber-500/20">今日: {{ formatNumberKMB(stats?.today_tokens) }}</span>
        </div>
        <div>
          <div class="flex items-center justify-between mb-0.5">
            <p class="text-xs text-zinc-400 font-medium">消耗 Tokens</p>
            <div class="text-xs text-zinc-500 flex space-x-2 font-medium">
              <span title="输入Tokens">入: {{ formatNumberKMB(stats?.total_input_tokens) }}</span>
              <span title="输出Tokens">出: {{ formatNumberKMB(stats?.total_output_tokens) }}</span>
              <span title="缓存Tokens">缓: {{ formatNumberKMB((stats?.total_cache_creation_tokens || 0) + (stats?.total_cache_read_tokens || 0)) }}</span>
            </div>
          </div>
          <div class="flex items-baseline space-x-1">
            <h2 class="text-2xl font-bold tracking-tight text-white">{{ formatNumberKMB(stats?.total_tokens) }}</h2>
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

    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { Line } from 'vue-chartjs'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler
} from 'chart.js'

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler
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
import { getDashboardStats, getDashboardTrend } from '@/api/usage'
import { list as listKeys } from '@/api/keys'
import { useAuthStore } from '@/stores/auth'

const authStore = useAuthStore()
const stats = ref<any>(null)
const trend = ref<any>(null)
const recentKeys = ref<any[]>([])
const loading = ref(true)

const formatNumberKMB = (num: number | undefined) => {
  if (num === null || num === undefined) return '0'
  if (num >= 1_000_000_000) return (num / 1_000_000_000).toFixed(1).replace(/\.0$/, '') + 'B'
  if (num >= 1_000_000) return (num / 1_000_000).toFixed(1).replace(/\.0$/, '') + 'M'
  if (num >= 1_000) return (num / 1_000).toFixed(1).replace(/\.0$/, '') + 'K'
  return num.toString()
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
        pointRadius: 0, // hide by default
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

const getTrendDateParams = (range: string) => {
  const now = new Date()
  const startDate = new Date()
  let granularity: 'day' | 'hour' = 'day'
  
  if (range === '30d') {
    startDate.setDate(now.getDate() - 30)
  } else if (range === '7d') {
    startDate.setDate(now.getDate() - 7)
  } else if (range === '24h') {
    startDate.setHours(now.getHours() - 24)
    granularity = 'hour'
  }
  
  const pad = (n: number) => String(n).padStart(2, '0')
  const formatDate = (d: Date, withTime = false) => {
    const ymd = `${d.getFullYear()}-${pad(d.getMonth()+1)}-${pad(d.getDate())}`
    if (withTime) {
      return `${ymd} ${pad(d.getHours())}:${pad(d.getMinutes())}:00`
    }
    return ymd
  }

  return {
    start_date: formatDate(startDate, range === '24h'),
    end_date: formatDate(now, range === '24h'),
    granularity
  }
}

const updateTrend = async (range: string) => {
  if (isTrendLoading.value || currentRange.value === range) return
  currentRange.value = range
  isTrendLoading.value = true
  try {
    const params = getTrendDateParams(range)
    const trendData = await getDashboardTrend(params)
    trend.value = trendData
  } catch (err) {
    console.error("Failed to update trend", err)
  } finally {
    isTrendLoading.value = false
  }
}

onMounted(async () => {
  try {
    await authStore.checkAuth()
    
    // Use Promise.all to fetch in parallel
    const [statsData, trendData, keysData] = await Promise.all([
      getDashboardStats(),
      getDashboardTrend(getTrendDateParams('30d')),
      listKeys(1, 4, { sort_by: 'created_at', sort_order: 'desc' })
    ])
    
    stats.value = statsData
    trend.value = trendData
    if (keysData && keysData.items) {
      recentKeys.value = keysData.items
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
</style>
