<template>
  <div class="space-y-6">
    <!-- Alert Banner -->
    <div class="glass-panel p-4 flex items-start space-x-4 border-red-500/30 bg-red-500/5 relative overflow-hidden group">
      <div class="absolute top-0 right-0 w-32 h-32 bg-red-500/10 rounded-full blur-3xl group-hover:bg-red-500/20 transition-all duration-500"></div>
      <div class="p-2 bg-red-500/20 rounded-full shrink-0">
        <ShieldAlert class="w-6 h-6 text-red-500" />
      </div>
      <div class="flex-1">
        <h3 class="text-white font-semibold mb-1">拦截到 12 次高频并发请求</h3>
        <p class="text-sm text-red-200/70">系统在过去一小时内自动封禁了 3 个来源 IP，建议检查风控规则。</p>
      </div>
      <button class="px-3 py-1.5 bg-red-500 hover:bg-red-600 text-white text-xs font-medium rounded transition">
        查看详情
      </button>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      
      <!-- IP Blacklist -->
      <div class="glass-panel p-6 flex flex-col h-[400px]">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-lg font-semibold text-white">IP 黑名单</h3>
          <button class="p-1.5 bg-white/5 hover:bg-white/10 rounded text-zinc-400 transition" title="添加 IP">
            <Plus class="w-4 h-4" />
          </button>
        </div>
        
        <div class="flex-1 overflow-y-auto custom-scrollbar pr-2 space-y-2">
          <div v-for="ip in blacklistedIPs" :key="ip.address" class="flex items-center justify-between p-3 bg-black/40 rounded-lg border border-white/5 hover:border-white/10 transition">
            <div>
              <div class="text-sm font-mono text-white mb-0.5">{{ ip.address }}</div>
              <div class="text-xs text-zinc-500">{{ ip.reason }}</div>
            </div>
            <button class="p-1.5 text-zinc-500 hover:text-red-400 transition">
              <Trash2 class="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>

      <!-- Rate Limits -->
      <div class="glass-panel p-6 flex flex-col h-[400px]">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-lg font-semibold text-white">速率限制规则</h3>
          <button class="p-1.5 bg-white/5 hover:bg-white/10 rounded text-zinc-400 transition" title="新建规则">
            <Plus class="w-4 h-4" />
          </button>
        </div>
        
        <div class="flex-1 overflow-y-auto custom-scrollbar pr-2 space-y-4">
          <div v-for="rule in rateLimits" :key="rule.id" class="p-4 bg-black/40 rounded-lg border border-white/5">
            <div class="flex items-center justify-between mb-3">
              <span class="text-sm font-medium text-white">{{ rule.name }}</span>
              <div class="relative inline-block w-8 mr-2 align-middle select-none">
                <input type="checkbox" :checked="rule.enabled" class="toggle-checkbox absolute block w-4 h-4 rounded-full bg-white border-4 border-zinc-700 appearance-none cursor-pointer transition-transform duration-200 ease-in-out checked:translate-x-4 checked:border-purple-500"/>
                <label class="toggle-label block overflow-hidden h-4 rounded-full bg-zinc-700 cursor-pointer transition-colors duration-200 ease-in-out peer-checked:bg-purple-500"></label>
              </div>
            </div>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <span class="text-xs text-zinc-500 block mb-1">目标对象</span>
                <span class="text-xs font-mono text-zinc-300 bg-white/5 px-2 py-0.5 rounded">{{ rule.target }}</span>
              </div>
              <div>
                <span class="text-xs text-zinc-500 block mb-1">限制条件</span>
                <span class="text-xs text-zinc-300">{{ rule.limit }} req / {{ rule.window }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
      
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { ShieldAlert, Plus, Trash2 } from '@lucide/vue'

const blacklistedIPs = ref([
  { address: '192.168.1.100', reason: '并发攻击 (1243 req/s)' },
  { address: '45.33.12.99', reason: '无效 Token 探测' },
  { address: '201.55.82.11', reason: '恶意爬虫' },
  { address: '8.8.8.8', reason: '管理员手动添加' },
])

const rateLimits = ref([
  { id: 1, name: '全局并发上限', target: 'Global', limit: 1000, window: '1s', enabled: true },
  { id: 2, name: '单 IP 默认限制', target: 'Per IP', limit: 60, window: '1m', enabled: true },
  { id: 3, name: '普通用户限速', target: 'Role: User', limit: 10, window: '1s', enabled: true },
  { id: 4, name: 'API Key 并发限速', target: 'Per Token', limit: 5, window: '1s', enabled: false },
])
</script>

<style scoped>
/* Simple CSS Toggle Switch */
.toggle-checkbox:checked {
  right: 0;
  border-color: #a855f7;
}
.toggle-checkbox:checked + .toggle-label {
  background-color: #a855f7;
}
</style>
