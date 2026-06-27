<template>
  <div class="space-y-6 max-w-4xl">


    <!-- Tabs -->
    <div class="flex space-x-1 border-b border-white/10 pb-4">
      <button 
        v-for="tab in tabs" 
        :key="tab.id"
        @click="activeTab = tab.id"
        class="px-4 py-2 text-sm font-medium rounded-lg transition-colors"
        :class="activeTab === tab.id ? 'bg-white/10 text-white' : 'text-zinc-400 hover:text-zinc-200 hover:bg-white/5'"
      >
        {{ tab.name }}
      </button>
    </div>

    <!-- Settings Content -->
    <div class="glass-panel p-8">
      
      <!-- General Settings -->
      <div v-if="activeTab === 'general'" class="space-y-6">
        <div>
          <label class="block text-sm font-medium text-zinc-300 mb-2">站点名称</label>
          <input 
            type="text" 
            value="Sub2API"
            class="w-full max-w-md bg-black/50 border border-white/10 rounded-lg px-4 py-2 text-white focus:outline-none focus:ring-2 focus:ring-purple-500/50"
          />
        </div>
        <div>
          <label class="block text-sm font-medium text-zinc-300 mb-2">网站 Logo URL</label>
          <input 
            type="text" 
            placeholder="https://..."
            class="w-full max-w-md bg-black/50 border border-white/10 rounded-lg px-4 py-2 text-white placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-purple-500/50"
          />
        </div>
        <div>
          <label class="block text-sm font-medium text-zinc-300 mb-2">站点通知公告</label>
          <textarea 
            rows="3"
            class="w-full bg-black/50 border border-white/10 rounded-lg px-4 py-2 text-white placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-purple-500/50"
            placeholder="支持 Markdown 格式..."
          >欢迎使用全新设计的 API 聚合服务！</textarea>
        </div>
        <div class="pt-4">
          <button class="px-6 py-2 bg-purple-600 hover:bg-purple-700 text-white font-medium rounded-lg transition shadow-lg shadow-purple-500/20">保存更改</button>
        </div>
      </div>

      <!-- Email Settings -->
      <div v-if="activeTab === 'email'" class="space-y-6">
        <div class="flex items-center p-4 bg-amber-500/10 border border-amber-500/20 rounded-lg">
          <AlertTriangle class="w-5 h-5 text-amber-500 mr-3 shrink-0" />
          <p class="text-sm text-amber-200">请确保 SMTP 服务器配置正确，否则用户将无法接收注册验证码与密码重置邮件。</p>
        </div>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <label class="block text-sm font-medium text-zinc-300 mb-2">SMTP 服务器</label>
            <input type="text" placeholder="smtp.gmail.com" class="w-full bg-black/50 border border-white/10 rounded-lg px-4 py-2 text-white focus:outline-none focus:ring-2 focus:ring-purple-500/50" />
          </div>
          <div>
            <label class="block text-sm font-medium text-zinc-300 mb-2">端口 (Port)</label>
            <input type="text" placeholder="465" class="w-full bg-black/50 border border-white/10 rounded-lg px-4 py-2 text-white focus:outline-none focus:ring-2 focus:ring-purple-500/50" />
          </div>
          <div>
            <label class="block text-sm font-medium text-zinc-300 mb-2">SMTP 用户名</label>
            <input type="text" placeholder="noreply@domain.com" class="w-full bg-black/50 border border-white/10 rounded-lg px-4 py-2 text-white focus:outline-none focus:ring-2 focus:ring-purple-500/50" />
          </div>
          <div>
            <label class="block text-sm font-medium text-zinc-300 mb-2">SMTP 密码</label>
            <input type="password" placeholder="••••••••" class="w-full bg-black/50 border border-white/10 rounded-lg px-4 py-2 text-white focus:outline-none focus:ring-2 focus:ring-purple-500/50" />
          </div>
        </div>
        
        <div class="pt-4 flex space-x-3">
          <button class="px-6 py-2 bg-purple-600 hover:bg-purple-700 text-white font-medium rounded-lg transition shadow-lg shadow-purple-500/20">保存配置</button>
          <button class="px-6 py-2 bg-white/5 hover:bg-white/10 text-white font-medium border border-white/10 rounded-lg transition">发送测试邮件</button>
        </div>
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { AlertTriangle } from '@lucide/vue'

const tabs = [
  { id: 'general', name: '基本设置' },
  { id: 'email', name: '邮件 (SMTP)' },
  { id: 'payment', name: '支付网关' },
  { id: 'advanced', name: '高级选项' },
]

const activeTab = ref('general')
</script>
