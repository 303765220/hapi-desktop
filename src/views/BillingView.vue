<template>
  <div class="space-y-6 flex flex-col min-h-0">
    <!-- Header: Recharge and Balance -->
    <div class="grid grid-cols-1 lg:grid-cols-[1fr_1.12fr_1fr] gap-6 shrink-0">
      
      <!-- Current Balance -->
      <div class="glass-panel p-6 flex flex-col justify-center">
        <h3 class="text-lg font-semibold text-white mb-4">当前余额</h3>
        <div class="text-4xl font-bold text-white mb-2">¥{{ (authStore.user?.balance || 0).toLocaleString() }}</div>
        <p class="text-sm text-zinc-400 mb-6">可用于所有计费服务</p>
        
        <div class="w-full bg-white/5 rounded-full h-2 mb-2 mt-auto">
          <div class="bg-gradient-to-r from-purple-500 to-indigo-500 h-2 rounded-full" style="width: 15%"></div>
        </div>
      </div>

      <!-- Redeem Code -->
      <div class="glass-panel p-6 flex flex-col justify-center">
        <h3 class="text-lg font-semibold text-white mb-4">兑换码</h3>
        <p class="text-sm text-zinc-400 mb-4">拥有兑换码？在此输入充值。</p>
        
        <div class="mt-auto flex flex-col gap-3 sm:flex-row sm:items-stretch rounded-xl border border-white/10 bg-black/35 p-1.5 shadow-inner shadow-black/20 focus-within:border-purple-500/40 focus-within:ring-2 focus-within:ring-purple-500/15">
          <input 
            v-model="redeemCode" 
            type="text" 
            class="h-12 min-w-0 flex-1 bg-transparent px-4 text-base font-medium tracking-wide text-white placeholder-zinc-600 outline-none"
            placeholder="XXXX-XXXX-XXXX"
          />
          <button
            @click="handleRedeem"
            class="inline-flex h-12 w-full shrink-0 items-center justify-center whitespace-nowrap rounded-lg bg-purple-600 px-5 text-sm font-bold text-white transition-colors hover:bg-purple-500 disabled:cursor-not-allowed disabled:opacity-50 sm:w-28"
            :disabled="redeeming || !redeemCode.trim()"
          >
            <Loader2 v-if="redeeming" class="mr-2 h-4 w-4 animate-spin" />
            {{ redeeming ? '处理中' : '兑换' }}
          </button>
        </div>
      </div>

      <!-- Buy Code Section -->
      <div class="glass-panel p-6 flex flex-col items-center justify-center text-center relative overflow-hidden group">
        <div class="absolute inset-0 bg-gradient-to-br from-purple-500/10 to-indigo-500/5 pointer-events-none"></div>
        <ShoppingCart class="w-8 h-8 text-purple-400 mb-3 relative z-10" />
        <h3 class="text-lg font-semibold text-white mb-2 relative z-10">获取充值额度</h3>
        <p class="text-sm text-zinc-400 mb-6 relative z-10">
          前往发卡网购买授权兑换码
        </p>
        <button
          @click="openExternalUrl('https://catfk.com/shop/hapi')"
          class="w-full px-4 py-2.5 bg-white/10 hover:bg-white/20 text-white text-sm font-medium rounded-lg transition-all border border-white/5 flex items-center justify-center space-x-2 mt-auto relative z-10 cursor-pointer"
        >
          <ExternalLink class="w-4 h-4" />
          <span>前往购买</span>
        </button>
      </div>

    </div>

    <!-- Discount Packages (Plans) -->
    <div class="glass-panel p-8">
      <div class="flex items-center justify-between mb-8">
        <div>
          <div class="inline-flex items-center space-x-2 bg-purple-500/10 text-purple-400 px-3 py-1 rounded-full text-xs font-semibold mb-3 border border-purple-500/20">
            <Sparkles class="w-3 h-3" />
            <span>超值特惠</span>
          </div>
          <h3 class="text-2xl font-bold text-white">优惠套餐</h3>
        </div>
      </div>
      
      <!-- Loading Skeleton for Plans -->
      <div v-if="loadingPlans" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div v-for="i in 3" :key="i" class="bg-white/5 p-6 rounded-2xl animate-pulse h-[400px]"></div>
      </div>

      <!-- Empty State for Plans -->
      <div v-else-if="plans.length === 0" class="flex flex-col items-center justify-center py-10 bg-white/5 rounded-2xl">
        <PackageOpen class="w-12 h-12 text-zinc-600 mb-4" />
        <h3 class="text-lg font-semibold text-zinc-300 mb-2">暂无优惠套餐</h3>
        <p class="text-zinc-500 text-sm">目前没有在售的订阅套餐，请稍后再来看看。</p>
      </div>

      <!-- Pricing Cards -->
      <div v-else class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6 items-stretch">
        <div 
          v-for="(plan, index) in plans" 
          :key="plan.id"
          class="rounded-3xl border transition-all duration-300 hover:-translate-y-1 group relative overflow-hidden flex flex-col bg-zinc-900/50"
          :class="isPopular(index) ? 'border-purple-500/40 shadow-xl shadow-purple-500/10' : 'border-white/10 hover:border-white/30'"
        >
          <template v-if="isPopular(index)">
            <div class="absolute top-0 inset-x-0 h-1 bg-gradient-to-r from-purple-500 via-indigo-500 to-cyan-500"></div>
            <div class="absolute -top-24 -right-24 w-48 h-48 bg-purple-500/10 rounded-full blur-3xl pointer-events-none"></div>
            <div class="absolute top-4 right-4 bg-purple-500/10 border border-purple-500/30 text-purple-400 text-[10px] font-bold px-3 py-1 rounded-full uppercase tracking-wider">
              最受欢迎
            </div>
          </template>
          
          <div class="p-6 md:p-8 flex-1 flex flex-col">
            <div class="mb-6 relative z-10">
              <h3 class="text-xl md:text-2xl font-bold text-white mb-2">{{ plan.name }}</h3>
              <p class="text-sm text-zinc-400 min-h-[40px]">{{ plan.description || '解锁更多特权，享受无忧体验。' }}</p>
            </div>

            <div class="mb-6 relative z-10">
              <div class="flex items-baseline space-x-2">
                <span class="text-4xl md:text-5xl font-extrabold text-white">¥{{ plan.price }}</span>
                <span class="text-zinc-500 font-medium">/ {{ formatValidity(plan.validity_days, plan.validity_unit) }}</span>
              </div>
              <div v-if="plan.original_price && plan.original_price > plan.price" class="mt-2 flex items-center space-x-2">
                <span class="text-sm text-zinc-500 line-through decoration-zinc-500/50">原价 ¥{{ plan.original_price }}</span>
                <span class="text-xs font-semibold text-emerald-400 bg-emerald-400/10 px-2 py-0.5 rounded">
                  立省 ¥{{ (plan.original_price - plan.price).toFixed(2) }}
                </span>
              </div>
              <div v-else class="mt-2 h-5"></div>
            </div>

            <div class="flex-1 relative z-10 mb-8">
              <ul class="space-y-3">
                <li v-if="plan.daily_limit_usd" class="flex items-start text-sm">
                  <div class="mt-0.5 bg-blue-500/20 p-1 rounded-full mr-3 border border-blue-500/30">
                    <Check class="w-3 h-3 text-blue-400" />
                  </div>
                  <span class="text-zinc-200">每日调用额度 <span class="font-bold text-white">${{ plan.daily_limit_usd }}</span></span>
                </li>
                <li v-else-if="plan.monthly_limit_usd" class="flex items-start text-sm">
                  <div class="mt-0.5 bg-fuchsia-500/20 p-1 rounded-full mr-3 border border-fuchsia-500/30">
                    <Check class="w-3 h-3 text-fuchsia-400" />
                  </div>
                  <span class="text-zinc-200">每月调用额度 <span class="font-bold text-white">${{ plan.monthly_limit_usd }}</span></span>
                </li>
                
                <li v-for="(feature, fIndex) in getParsedFeatures(plan.features)" :key="fIndex" class="flex items-start text-sm">
                  <div class="mt-0.5 bg-emerald-500/20 p-1 rounded-full mr-3 border border-emerald-500/30">
                    <Check class="w-3 h-3 text-emerald-400" />
                  </div>
                  <span class="text-zinc-300">{{ feature }}</span>
                </li>
              </ul>
            </div>

            <div class="mt-auto relative z-10">
              <button 
                @click="handleSubscribe(plan)"
                class="w-full py-3.5 px-6 rounded-xl font-bold transition-all duration-300 transform active:scale-95 flex items-center justify-center space-x-2"
                :class="isPopular(index) ? 'bg-white text-black hover:bg-zinc-200 shadow-lg shadow-white/10' : 'bg-zinc-800 text-white hover:bg-zinc-700 border border-white/10'"
              >
                <span>立即订阅</span>
                <ArrowRight class="w-4 h-4" />
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Orders History -->
    <div class="glass-panel p-6">
      <h3 class="text-lg font-semibold text-white mb-6">兑换记录</h3>
      <div class="overflow-x-auto">
        <table class="w-full text-left border-collapse">
          <thead>
            <tr class="border-b border-white/10 bg-black/20">
              <th class="px-6 py-4 text-sm font-medium text-zinc-400">类型</th>
              <th class="px-6 py-4 text-sm font-medium text-zinc-400">兑换码</th>
              <th class="px-6 py-4 text-sm font-medium text-zinc-400">面额</th>
              <th class="px-6 py-4 text-sm font-medium text-zinc-400 text-right">时间</th>
            </tr>
          </thead>
          <tbody v-if="loadingHistory" class="divide-y divide-white/5">
            <tr v-for="i in 3" :key="'sk-'+i" class="animate-pulse">
              <td class="px-6 py-4"><div class="h-4 bg-white/10 rounded w-12"></div></td>
              <td class="px-6 py-4"><div class="h-4 bg-white/5 rounded w-32"></div></td>
              <td class="px-6 py-4"><div class="h-4 bg-white/5 rounded w-16"></div></td>
              <td class="px-6 py-4 flex justify-end"><div class="h-4 bg-white/5 rounded w-32"></div></td>
            </tr>
          </tbody>
          <tbody v-else class="divide-y divide-white/5">
            <tr v-for="item in redeemHistory" :key="item.id" class="hover:bg-white/5 transition-colors">
              <td class="px-6 py-4 text-sm font-medium text-white">{{ item.type === 'balance' ? '余额' : '并发' }}</td>
              <td class="px-6 py-4 text-sm text-zinc-300 font-mono">{{ item.code }}</td>
              <td class="px-6 py-4 text-sm text-green-400">+¥{{ item.value }}</td>
              <td class="px-6 py-4 text-sm text-zinc-400 text-right">{{ formatBeijingTime(item.used_at) }}</td>
            </tr>
            <tr v-if="redeemHistory.length === 0">
              <td colspan="4" class="px-6 py-8 text-center text-zinc-500">暂无兑换记录</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ShoppingCart, ExternalLink, Sparkles, PackageOpen, Check, ArrowRight, Loader2 } from '@lucide/vue'
import { useAuthStore } from '@/stores/auth'
import { useMessage } from '@/utils/message'
import { openExternalUrl } from '@/utils/desktop-env'
import { redeem, getHistory, type RedeemHistoryItem } from '@/api/redeem'
import { paymentAPI } from '@/api/payment'
import type { SubscriptionPlan } from '@/types/payment'

const authStore = useAuthStore()
const message = useMessage()

// --- Redeem Logic ---
const redeemCode = ref('')
const redeeming = ref(false)
const redeemHistory = ref<RedeemHistoryItem[]>([])
const loadingHistory = ref(true)

const formatBeijingTime = (dateStr: string | null | undefined) => {
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

const fetchHistory = async () => {
  loadingHistory.value = true
  try {
    redeemHistory.value = await getHistory()
  } catch (err) {
    console.error(err)
  } finally {
    loadingHistory.value = false
  }
}

const handleRedeem = async () => {
  if (!redeemCode.value) return
  redeeming.value = true
  try {
    const res = await redeem(redeemCode.value)
    if (res.type === 'balance' && res.value) {
      message.success(`兑换成功！已增加额度: ¥${res.value}`)
    } else {
      message.success('兑换成功！')
    }
    redeemCode.value = ''
    await authStore.checkAuth() // Refresh user balance
    await fetchHistory()
  } catch (err) {
    console.error(err)
    message.error('兑换失败或兑换码无效')
  } finally {
    redeeming.value = false
  }
}

// --- Plans Logic ---
const plans = ref<SubscriptionPlan[]>([])
const loadingPlans = ref(true)

const fetchPlans = async () => {
  loadingPlans.value = true
  try {
    const response = await paymentAPI.getPlans()
    let data = response as any
    if (!Array.isArray(data) && data.items) data = data.items
    else if (!Array.isArray(data) && data.data) data = data.data
    
    const parsed = Array.isArray(data) ? data : []
    plans.value = parsed
      .filter(p => p.for_sale !== false)
      .sort((a, b) => (a.sort_order || 0) - (b.sort_order || 0))
  } catch (error) {
    console.error('Failed to load plans:', error)
  } finally {
    loadingPlans.value = false
  }
}

const isPopular = (index: number) => {
  if (plans.value.length >= 3 && index === 1) return true;
  if (plans.value.length === 1 && index === 0) return true;
  return false;
}

const formatValidity = (days: number, unit: string) => {
  if (days === 1) return '天'
  if (days === 7) return '周'
  if (days === 30 || days === 31) return '月'
  if (days === 365) return '年'
  return `${days} ${unit === 'days' ? '天' : unit}`
}

const getParsedFeatures = (featuresStr: any): string[] => {
  if (!featuresStr) return []
  if (Array.isArray(featuresStr)) return featuresStr
  try {
    const parsed = JSON.parse(featuresStr)
    return Array.isArray(parsed) ? parsed : []
  } catch (e) {
    return []
  }
}

const handleSubscribe = (_plan: SubscriptionPlan) => {
  openExternalUrl('https://catfk.com/shop/hapi')
}

// --- Lifecycle ---
onMounted(() => {
  fetchHistory()
  fetchPlans()
})
</script>
