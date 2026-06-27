<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-end">
      <div class="flex items-center space-x-2">
        <select class="bg-black/50 border border-white/10 text-white text-sm rounded-lg px-3 py-2 focus:ring-purple-500">
          <option>最近 7 天</option>
          <option>最近 30 天</option>
          <option>本月</option>
        </select>
        <button class="px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white font-medium rounded-lg transition shadow-lg shadow-purple-500/20">
          导出报表
        </button>
      </div>
    </div>

    <!-- Core Metrics -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      <div class="glass-panel p-6 relative overflow-hidden group">
        <div class="absolute -right-4 -top-4 w-24 h-24 bg-blue-500/20 rounded-full blur-2xl group-hover:bg-blue-500/30 transition"></div>
        <div class="flex items-center space-x-3 mb-4">
          <div class="p-2 bg-blue-500/10 rounded-lg"><Users class="w-5 h-5 text-blue-400" /></div>
          <span class="text-sm font-medium text-zinc-400">总用户数</span>
        </div>
        <div class="flex items-end justify-between">
          <div class="text-3xl font-bold text-white">{{ stats?.total_users || 0 }}</div>
          <div class="text-sm font-medium text-green-400 flex items-center"><TrendingUp class="w-3 h-3 mr-1" />今日新增 {{ stats?.today_new_users || 0 }}</div>
        </div>
      </div>

      <div class="glass-panel p-6 relative overflow-hidden group">
        <div class="absolute -right-4 -top-4 w-24 h-24 bg-purple-500/20 rounded-full blur-2xl group-hover:bg-purple-500/30 transition"></div>
        <div class="flex items-center space-x-3 mb-4">
          <div class="p-2 bg-purple-500/10 rounded-lg"><Activity class="w-5 h-5 text-purple-400" /></div>
          <span class="text-sm font-medium text-zinc-400">API 总请求</span>
        </div>
        <div class="flex items-end justify-between">
          <div class="text-3xl font-bold text-white">{{ formatNumber(stats?.total_requests || 0) }}</div>
          <div class="text-sm font-medium text-zinc-400 flex items-center">今日 {{ formatNumber(stats?.today_requests || 0) }}</div>
        </div>
      </div>

      <div class="glass-panel p-6 relative overflow-hidden group">
        <div class="absolute -right-4 -top-4 w-24 h-24 bg-emerald-500/20 rounded-full blur-2xl group-hover:bg-emerald-500/30 transition"></div>
        <div class="flex items-center space-x-3 mb-4">
          <div class="p-2 bg-emerald-500/10 rounded-lg"><DollarSign class="w-5 h-5 text-emerald-400" /></div>
          <span class="text-sm font-medium text-zinc-400">总消费</span>
        </div>
        <div class="flex items-end justify-between">
          <div class="text-3xl font-bold text-white">¥{{ formatNumber(stats?.total_cost || 0) }}</div>
          <div class="text-sm font-medium text-zinc-400 flex items-center">总成本 ¥{{ formatNumber(stats?.total_account_cost || 0) }}</div>
        </div>
      </div>

      <div class="glass-panel p-6 relative overflow-hidden group">
        <div class="absolute -right-4 -top-4 w-24 h-24 bg-amber-500/20 rounded-full blur-2xl group-hover:bg-amber-500/30 transition"></div>
        <div class="flex items-center space-x-3 mb-4">
          <div class="p-2 bg-amber-500/10 rounded-lg"><Server class="w-5 h-5 text-amber-400" /></div>
          <span class="text-sm font-medium text-zinc-400">正常账户</span>
        </div>
        <div class="flex items-end justify-between">
          <div class="text-3xl font-bold text-white">{{ stats?.normal_accounts || 0 }} / {{ stats?.total_accounts || 0 }}</div>
          <div class="text-sm font-medium text-red-400 flex items-center" v-if="(stats?.error_accounts || 0) > 0"><AlertCircle class="w-3 h-3 mr-1" />{{ stats?.error_accounts }} 个异常</div>
          <div class="text-sm font-medium text-green-400 flex items-center" v-else>全部正常</div>
        </div>
      </div>
    </div>

    <!-- Charts Section -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      
      <!-- Main Chart (Requests) -->
      <div class="glass-panel lg:col-span-2 p-6 flex flex-col h-[400px]">
        <div class="flex justify-between items-center mb-6">
          <h3 class="text-lg font-semibold text-white">请求流量与成本概览</h3>
        </div>
        <div class="flex-1 relative">
          <Line :data="chartData" :options="chartOptions" />
        </div>
      </div>

      <!-- Donut Chart (Model Distribution) -->
      <div class="glass-panel p-6 flex flex-col h-[400px]">
        <h3 class="text-lg font-semibold text-white mb-6">模型调用占比</h3>
        <div class="flex-1 relative flex items-center justify-center">
          <Doughnut :data="donutData" :options="donutOptions" v-if="donutData.labels.length > 0" />
          <div v-else class="text-zinc-500 text-sm">暂无数据</div>
          <div class="absolute inset-0 flex flex-col items-center justify-center pointer-events-none" v-if="donutData.labels.length > 0">
            <span class="text-3xl font-bold text-white">{{ donutData.labels.length }}</span>
            <span class="text-xs text-zinc-400">个可用模型</span>
          </div>
        </div>
      </div>
      
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Users, Activity, DollarSign, Server, TrendingUp, AlertCircle } from '@lucide/vue'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  ArcElement,
  Filler
} from 'chart.js'
import { Line, Doughnut } from 'vue-chartjs'
import { adminAPI } from '@/api/admin'
import type { DashboardStats, TrendDataPoint, ModelStat } from '@/types'

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  ArcElement,
  Filler
)

const stats = ref<DashboardStats | null>(null)
const chartData = ref<any>({
  labels: [],
  datasets: []
})
const donutData = ref<any>({
  labels: [],
  datasets: []
})

const formatNumber = (num: number) => {
  if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M'
  if (num >= 1000) return (num / 1000).toFixed(1) + 'K'
  return num.toString()
}

onMounted(async () => {
  try {
    const [statsRes, trendRes, modelsRes] = await Promise.all([
      adminAPI.dashboard.getStats(),
      adminAPI.dashboard.getUsageTrend(),
      adminAPI.dashboard.getModelStats()
    ])
    
    stats.value = statsRes
    
    const trends = (trendRes.trend || trendRes) as TrendDataPoint[]
    
    if (trends && trends.length > 0) {
      chartData.value = {
        labels: trends.map(t => new Date(t.date).toLocaleDateString()),
        datasets: [
          {
            label: '请求数',
            data: trends.map(t => t.requests),
            borderColor: '#a855f7',
            backgroundColor: 'rgba(168, 85, 247, 0.1)',
            tension: 0.4,
            fill: true,
            yAxisID: 'y'
          },
          {
            label: '成本 (¥)',
            data: trends.map(t => t.cost),
            borderColor: '#3b82f6',
            backgroundColor: 'transparent',
            tension: 0.4,
            yAxisID: 'y1'
          }
        ]
      }
    }

    const models = (modelsRes.models) as ModelStat[]
    if (models && models.length > 0) {
      // Top 5 models
      const topModels = models.sort((a, b) => b.requests - a.requests).slice(0, 5)
      const colors = ['#a855f7', '#3b82f6', '#10b981', '#f59e0b', '#ec4899']
      
      donutData.value = {
        labels: topModels.map(m => m.model),
        datasets: [{
          data: topModels.map(m => m.requests),
          backgroundColor: colors.slice(0, topModels.length),
          borderWidth: 0,
          hoverOffset: 4
        }]
      }
    }
  } catch (err) {
    console.error('Failed to load dashboard data', err)
  }
})

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { display: false },
    tooltip: { mode: 'index' as const, intersect: false, backgroundColor: '#18181b', titleColor: '#a1a1aa', bodyColor: '#fff', borderColor: 'rgba(255,255,255,0.1)', borderWidth: 1 }
  },
  scales: {
    y: {
      type: 'linear' as const,
      display: true,
      position: 'left' as const,
      grid: { color: 'rgba(255, 255, 255, 0.05)' },
      ticks: { color: 'rgba(255, 255, 255, 0.5)' }
    },
    y1: {
      type: 'linear' as const,
      display: true,
      position: 'right' as const,
      grid: { drawOnChartArea: false },
      ticks: { color: 'rgba(255, 255, 255, 0.5)' }
    },
    x: {
      grid: { display: false },
      ticks: { color: 'rgba(255, 255, 255, 0.5)' }
    }
  }
}

const donutOptions = {
  responsive: true,
  maintainAspectRatio: false,
  cutout: '75%',
  plugins: {
    legend: { position: 'right' as const, labels: { color: '#a1a1aa', usePointStyle: true, padding: 20 } }
  }
}
</script>
