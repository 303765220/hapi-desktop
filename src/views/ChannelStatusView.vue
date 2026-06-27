<template>
  <div class="space-y-6">
    <div class="flex flex-col md:flex-row justify-between md:items-end gap-4 mb-6">
      <div>
        <h2 class="text-2xl font-bold text-white mb-2">渠道状态</h2>
        <p class="text-zinc-400">实时监控 API 渠道的健康状况和可用性</p>
      </div>
      <div class="flex items-center space-x-2 bg-white/5 px-4 py-2 rounded-lg border border-white/10">
        <div class="w-3 h-3 rounded-full" :class="overallStatus === 'operational' ? 'bg-green-500 shadow-[0_0_10px_rgba(34,197,94,0.5)]' : 'bg-yellow-500 shadow-[0_0_10px_rgba(234,179,8,0.5)]'"></div>
        <span class="text-white font-medium">{{ overallStatusText }}</span>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
      <div v-for="i in 6" :key="i" class="glass-panel p-6 animate-pulse">
        <div class="h-6 bg-white/10 rounded w-1/3 mb-4"></div>
        <div class="flex gap-0.5 h-12 mb-4">
          <div v-for="j in 48" :key="j" class="flex-1 bg-white/5 rounded-t-sm"></div>
        </div>
        <div class="flex justify-between">
          <div class="h-4 bg-white/10 rounded w-1/4"></div>
          <div class="h-4 bg-white/10 rounded w-1/4"></div>
        </div>
      </div>
    </div>

    <!-- Monitor Cards -->
    <div v-else class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
      <div 
        v-for="item in items" 
        :key="item.id"
        class="glass-panel p-6 hover:border-purple-500/30 transition-all duration-300 group hover:-translate-y-1 hover:shadow-lg hover:shadow-purple-500/10"
      >
        <div class="flex justify-between items-start mb-6">
          <div>
            <h3 class="text-lg font-bold text-white mb-1 group-hover:text-purple-400 transition-colors">{{ item.name }}</h3>
            <span class="text-xs text-zinc-400 font-medium px-2 py-0.5 bg-white/5 border border-white/10 rounded capitalize">{{ item.provider || 'default' }}</span>
          </div>
          <div 
            class="px-2.5 py-1 text-xs font-semibold rounded border"
            :class="getStatusBadgeClass(item.primary_status)"
          >
            {{ getStatusText(item.primary_status) }}
          </div>
        </div>

        <!-- Timeline -->
        <div class="mb-4">
          <div class="flex items-end gap-0.5 h-12 w-full group/timeline">
            <div 
              v-for="(point, idx) in getPaddedTimeline(item.timeline)" 
              :key="idx"
              class="flex-1 rounded-t-sm transition-all duration-300 hover:opacity-80 cursor-crosshair relative"
              :class="getTimelineBarClass(point.status)"
              :style="{ height: getTimelineBarHeight(point) }"
              :title="point.status && (point.status as any) !== 'unknown' ? `${getStatusText(point.status)} (延迟: ${point.latency_ms || '-'}ms)` : '无数据'"
            ></div>
          </div>
          <div class="flex justify-between text-xs text-zinc-500 mt-2 font-medium">
            <span>24小时前</span>
            <span>现在</span>
          </div>
        </div>

        <div class="flex justify-between items-center pt-4 border-t border-white/5">
          <div class="text-sm">
            <span class="text-zinc-500 mr-2">7天可用率</span>
            <span class="font-medium text-white">{{ Number(item.availability_7d).toFixed(2) }}%</span>
          </div>
          <div class="text-sm">
            <span class="text-zinc-500 mr-2">当前延迟</span>
            <span class="font-medium text-white">{{ item.primary_latency_ms ? item.primary_latency_ms + ' ms' : '-' }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { list, type UserMonitorView, type MonitorTimelinePoint } from '@/api/channelMonitor'

const items = ref<UserMonitorView[]>([])
const loading = ref(true)

const overallStatus = computed(() => {
  if (items.value.length === 0) return 'operational'
  const hasErrors = items.value.some(it => it.primary_status !== 'operational')
  return hasErrors ? 'degraded' : 'operational'
})

const overallStatusText = computed(() => {
  if (items.value.length === 0) return '暂无监控数据'
  return overallStatus.value === 'operational' ? '所有渠道运行正常' : '部分渠道存在异常'
})

const getStatusBadgeClass = (status: string) => {
  switch (status) {
    case 'operational': return 'bg-green-500/10 text-green-400 border-green-500/20 shadow-sm shadow-green-500/5'
    case 'degraded': return 'bg-yellow-500/10 text-yellow-400 border-yellow-500/20 shadow-sm shadow-yellow-500/5'
    case 'failed':
    case 'error': return 'bg-red-500/10 text-red-400 border-red-500/20 shadow-sm shadow-red-500/5'
    default: return 'bg-zinc-500/10 text-zinc-400 border-zinc-500/20'
  }
}

const getStatusText = (status: string) => {
  switch (status) {
    case 'operational': return '运行正常'
    case 'degraded': return '性能下降'
    case 'failed':
    case 'error': return '服务不可用'
    default: return '未知状态'
  }
}

// Pads timeline to at least 24 points so the UI looks consistent
const getPaddedTimeline = (timeline: MonitorTimelinePoint[]) => {
  const maxPoints = 48
  const points = [...(timeline || [])].slice(-maxPoints)
  while (points.length < maxPoints) {
    points.unshift({ status: 'unknown' as any, latency_ms: null, ping_latency_ms: null, checked_at: '' })
  }
  return points
}

const getTimelineBarClass = (status: string) => {
  switch (status) {
    case 'operational': return 'bg-green-400/80 hover:bg-green-300'
    case 'degraded': return 'bg-yellow-400/80 hover:bg-yellow-300'
    case 'failed':
    case 'error': return 'bg-red-400/80 hover:bg-red-300'
    default: return 'bg-white/5'
  }
}

const getTimelineBarHeight = (point: MonitorTimelinePoint) => {
  if (!point.status || (point.status as any) === 'unknown') return '10%'
  if (point.status !== 'operational') return '100%'
  // For operational, calculate height based on latency (lower latency = shorter bar, but min 20%)
  const latency = point.latency_ms || 100
  const maxLatency = 2000
  const percentage = Math.max(20, Math.min(100, (latency / maxLatency) * 100))
  return `${percentage}%`
}

onMounted(async () => {
  try {
    const res = await list()
    items.value = res.items || []
  } catch (error) {
    console.error('Failed to load channel monitors', error)
  } finally {
    loading.value = false
  }
})
</script>
