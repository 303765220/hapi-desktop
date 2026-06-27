<template>
  <div class="space-y-6">
    <!-- Referral Code Section -->
    <div v-if="loading" class="glass-panel p-8 relative overflow-hidden animate-pulse flex flex-col md:flex-row md:items-center justify-between gap-6">
      <div class="flex-1">
        <div class="w-48 h-6 bg-white/10 rounded mb-4"></div>
        <div class="w-full max-w-lg h-12 bg-white/5 rounded mb-6"></div>
        <div class="w-32 h-3 bg-white/5 rounded mb-2"></div>
        <div class="w-full h-10 bg-white/10 rounded"></div>
      </div>
      <div class="flex-shrink-0 grid grid-cols-2 gap-4">
        <div class="p-4 bg-white/5 border border-white/10 rounded-xl text-center w-32 h-24"></div>
        <div class="p-4 bg-white/5 border border-white/10 rounded-xl text-center w-32 h-24"></div>
      </div>
    </div>
    
    <div v-else class="glass-panel p-8 relative overflow-hidden group">
      <div class="absolute top-0 right-0 w-64 h-64 bg-amber-500/10 rounded-full blur-3xl group-hover:bg-amber-500/20 transition-all duration-500 -translate-y-1/2 translate-x-1/4 pointer-events-none"></div>
      
      <div class="flex flex-col md:flex-row md:items-center justify-between gap-6 relative z-10">
        <div>
          <h3 class="text-xl font-bold text-white mb-2">赚取邀请佣金</h3>
          <p class="text-zinc-400 max-w-lg mb-6">分享您的专属邀请链接给朋友。当他们注册并充值时，您将获得其充值金额的返利奖励。</p>
          
          <div class="space-y-4">
            <div>
              <label class="block text-xs font-medium text-zinc-500 mb-1 uppercase tracking-wider">您的专属邀请链接</label>
              <div class="flex items-center space-x-2">
                <div class="bg-black/50 border border-white/10 rounded-lg px-4 py-2 text-white font-mono text-sm flex-1 truncate">
                  {{ referralLink }}
                </div>
                <button @click="copyToClipboard(referralLink)" class="p-2 bg-white/5 border border-white/10 rounded-lg hover:bg-white/10 transition text-zinc-300">
                  <Copy class="w-5 h-5" />
                </button>
              </div>
            </div>
          </div>
        </div>
        
        <div class="flex-shrink-0 grid grid-cols-2 gap-4">
          <div class="p-4 bg-white/5 border border-white/10 rounded-xl text-center">
            <div class="text-3xl font-bold text-white mb-1">{{ affiliateDetail?.aff_count || 0 }}</div>
            <div class="text-xs text-zinc-400">累计邀请</div>
          </div>
          <div class="p-4 bg-white/5 border border-amber-500/20 rounded-xl text-center relative overflow-hidden">
            <div class="absolute inset-0 bg-gradient-to-br from-amber-500/5 to-orange-500/10"></div>
            <div class="text-3xl font-bold text-amber-400 mb-1 relative z-10">¥{{ affiliateDetail?.aff_history_quota || 0 }}</div>
            <div class="text-xs text-amber-500/70 relative z-10">总返利收入</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Referral History -->
    <div class="glass-panel overflow-hidden">
      <div class="p-6 border-b border-white/5 flex justify-between items-center">
        <h3 class="text-lg font-semibold text-white">邀请记录</h3>
        <button @click="handleTransfer" class="px-4 py-2 bg-purple-500 text-white font-medium rounded-lg hover:bg-purple-600 transition disabled:opacity-50" :disabled="transferring">
          一键划转至余额
        </button>
      </div>
      
      <table class="w-full text-left border-collapse">
        <thead>
          <tr class="bg-black/20">
            <th class="px-6 py-4 text-sm font-medium text-zinc-400">用户</th>
            <th class="px-6 py-4 text-sm font-medium text-zinc-400">注册日期</th>
            <th class="px-6 py-4 text-sm font-medium text-zinc-400">状态</th>
            <th class="px-6 py-4 text-sm font-medium text-zinc-400 text-right">产生佣金</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-white/5">
          <tr v-for="invitee in affiliateDetail?.invitees || []" :key="invitee.user_id" class="hover:bg-white/5 transition-colors">
            <td class="px-6 py-4">
              <div class="flex items-center space-x-3">
                <div class="w-8 h-8 rounded-full bg-gradient-to-br from-zinc-700 to-zinc-800 flex items-center justify-center text-xs text-white">
                  {{ invitee.username?.substring(0, 1).toUpperCase() || 'U' }}
                </div>
                <div class="font-medium text-white text-sm">{{ invitee.username || invitee.email || `User_${invitee.user_id}` }}</div>
              </div>
            </td>
            <td class="px-6 py-4 text-sm text-zinc-400">
              {{ formatBeijingTime(invitee.created_at) }}
            </td>
            <td class="px-6 py-4">
              <span class="text-xs font-medium px-2 py-1 bg-green-500/10 text-green-400 rounded-full border border-green-500/20">已加入</span>
            </td>
            <td class="px-6 py-4 text-right text-sm font-mono text-amber-400">
              +¥{{ invitee.total_rebate || 0 }}
            </td>
          </tr>
          <tr v-if="!affiliateDetail?.invitees?.length">
            <td colspan="4" class="px-6 py-8 text-center text-zinc-500">暂无邀请记录</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Copy } from '@lucide/vue'
import { getAffiliateDetail, transferAffiliateQuota } from '@/api/user'
import { useMessage, showConfirm } from '@/utils/message'
import type { UserAffiliateDetail } from '@/types'

const message = useMessage()
const affiliateDetail = ref<UserAffiliateDetail | null>(null)
const referralLink = ref('')
const transferring = ref(false)
const loading = ref(true)

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

onMounted(async () => {
  try {
    const detail = await getAffiliateDetail()
    affiliateDetail.value = detail
    referralLink.value = `${window.location.origin}/register?aff=${detail.aff_code}`
  } catch (err) {
    console.error('Failed to load affiliate details', err)
  } finally {
    loading.value = false
  }
})

const handleTransfer = async () => {
  showConfirm({
    title: '确认划转',
    content: '确定要将所有可提取的返利划转到账户余额吗？',
    onConfirm: async () => {
      transferring.value = true
      try {
        await transferAffiliateQuota()
        message.success('划转成功！')
        const detail = await getAffiliateDetail()
        affiliateDetail.value = detail
      } catch (err) {
        console.error('Transfer failed', err)
        message.error('划转失败')
      } finally {
        transferring.value = false
      }
    }
  })
}

const copyToClipboard = (text: string) => {
  navigator.clipboard.writeText(text)
}
</script>
