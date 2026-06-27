<template>
  <div class="space-y-6">
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      
      <!-- Personal Info & Security -->
      <div class="glass-panel lg:col-span-2 p-8 flex flex-col space-y-10">
        
        <!-- Basic Info -->
        <section>
          <h3 class="text-lg font-semibold text-white mb-6">基本信息</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <label class="block text-sm font-medium text-zinc-300 mb-2">用户名</label>
              <input 
                type="text" 
                :value="user?.username"
                disabled
                class="w-full bg-black/50 border border-white/10 rounded-lg px-4 py-2.5 text-zinc-500 cursor-not-allowed"
              />
            </div>
            
            <div>
              <label class="block text-sm font-medium text-zinc-300 mb-2">绑定邮箱</label>
              <input 
                type="email" 
                :value="user?.email"
                disabled
                class="w-full bg-black/50 border border-white/10 rounded-lg px-4 py-2.5 text-zinc-500 cursor-not-allowed"
              />
            </div>
          </div>
        </section>

        <hr class="border-white/10" />

        <!-- Security / Password -->
        <section>
          <h3 class="text-lg font-semibold text-white mb-2">修改密码</h3>
          <p class="text-sm text-zinc-400 mb-6">为了保护您的账号安全，请定期修改密码。</p>
          
          <form @submit.prevent="handleUpdatePassword" class="space-y-4 max-w-md">
            <div>
              <input 
                v-model="oldPassword"
                type="password" 
                class="w-full bg-black/50 border border-white/10 rounded-lg px-4 py-2 text-white placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-purple-500/50"
                placeholder="当前密码"
                required
              />
            </div>
            <div>
              <input 
                v-model="newPassword"
                type="password" 
                class="w-full bg-black/50 border border-white/10 rounded-lg px-4 py-2 text-white placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-purple-500/50"
                placeholder="新密码"
                required
              />
            </div>
            <div class="pt-2">
              <button 
                type="submit" 
                class="px-6 py-2.5 bg-purple-600 hover:bg-purple-700 text-white font-medium rounded-lg transition-colors disabled:opacity-50"
                :disabled="updatingPassword"
              >
                {{ updatingPassword ? '提交中...' : '确认修改' }}
              </button>
            </div>
          </form>
        </section>
        
        <hr class="border-white/10" />

        <!-- Balance Alert -->
        <section>
          <h3 class="text-lg font-semibold text-white mb-2">额度预警</h3>
          <p class="text-sm text-zinc-400 mb-6">当账户余额低于设定阈值时，将会向您的邮箱发送提醒。</p>
          
          <div class="flex items-center space-x-4 mb-4">
            <label class="relative inline-flex items-center cursor-pointer">
              <input type="checkbox" v-model="alertEnabled" class="sr-only peer">
              <div class="w-11 h-6 bg-white/10 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-purple-500"></div>
              <span class="ml-3 text-sm font-medium text-zinc-300">开启邮件预警</span>
            </label>
          </div>
          
          <div v-if="alertEnabled" class="max-w-md">
            <label class="block text-sm font-medium text-zinc-300 mb-2">预警阈值 (¥)</label>
            <div class="flex space-x-2">
              <input 
                v-model.number="alertThreshold"
                type="number" 
                class="flex-1 bg-black/50 border border-white/10 rounded-lg px-4 py-2 text-white placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-purple-500/50"
                placeholder="例如: 10"
              />
              <button @click="saveAlertSettings" class="px-4 py-2 bg-white/10 hover:bg-white/20 text-white font-medium rounded-lg transition-colors">
                保存设置
              </button>
            </div>
          </div>
        </section>
        
      </div>

      <!-- Account Summary & Bindings -->
      <div class="space-y-6">
        
        <!-- Profile Card -->
        <div class="glass-panel p-6 relative overflow-hidden group">
          <div class="absolute top-0 right-0 w-32 h-32 bg-purple-500/10 rounded-full blur-2xl group-hover:bg-purple-500/20 transition-all duration-500"></div>
          
          <div class="flex items-center space-x-4 mb-6 relative">
            <div class="w-16 h-16 rounded-full bg-gradient-to-br from-purple-500 to-indigo-600 flex items-center justify-center shadow-lg">
              <span class="font-bold text-white text-2xl">{{ user?.username?.charAt(0).toUpperCase() || 'U' }}</span>
            </div>
            <div>
              <h3 class="text-xl font-bold text-white">{{ user?.username || '用户' }}</h3>
              <p class="text-sm text-zinc-400">UID: {{ user?.id }}</p>
            </div>
          </div>
          
          <div class="space-y-4 relative">
            <div class="flex justify-between items-center text-sm">
              <span class="text-zinc-400">用户角色</span>
              <span class="text-white font-medium capitalize bg-white/5 px-2 py-1 rounded border border-white/5">{{ user?.role === 'admin' ? '管理员' : '普通用户' }}</span>
            </div>
            <div class="flex justify-between items-center text-sm">
              <span class="text-zinc-400">账号状态</span>
              <span class="text-green-400 font-medium bg-green-500/10 px-2 py-0.5 rounded border border-green-500/20">正常激活</span>
            </div>
            <div class="flex justify-between items-center text-sm">
              <span class="text-zinc-400">注册时间</span>
              <span class="text-white font-medium">{{ formatDate(user?.created_at) }}</span>
            </div>
          </div>
        </div>
        
        <!-- Third-party Bindings -->
        <div class="glass-panel p-6">
          <h3 class="text-lg font-semibold text-white mb-4">第三方绑定</h3>
          <div class="space-y-3">
            <!-- WeChat -->
            <div class="flex items-center justify-between p-3 rounded-lg border border-white/5 bg-white/5 group hover:border-green-500/30 transition-colors">
              <div class="flex items-center space-x-3">
                <div class="w-8 h-8 rounded-full bg-green-500/10 border border-green-500/20 flex items-center justify-center text-green-500">
                  <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M8.5 13.5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm7 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm-3.5-8c-4.14 0-7.5 2.8-7.5 6.25 0 1.95 1.05 3.68 2.68 4.88-.2.75-.62 2.22-.62 2.22s1.42-.02 2.37-1.12c.98.3 2.02.47 3.07.47 4.14 0 7.5-2.8 7.5-6.25S16.14 5.5 12 5.5z"/></svg>
                </div>
                <div>
                  <div class="text-sm font-medium text-white">微信</div>
                  <div class="text-xs" :class="user?.wechat_bound ? 'text-green-400' : 'text-zinc-500'">{{ user?.wechat_bound ? '已绑定' : '未绑定' }}</div>
                </div>
              </div>
              <button class="text-xs font-medium text-zinc-400 hover:text-white px-3 py-1.5 bg-white/5 rounded hover:bg-white/10 transition">
                {{ user?.wechat_bound ? '解绑' : '去绑定' }}
              </button>
            </div>
            
            <!-- GitHub -->
            <div class="flex items-center justify-between p-3 rounded-lg border border-white/5 bg-white/5 group hover:border-white/20 transition-colors">
              <div class="flex items-center space-x-3">
                <div class="w-8 h-8 rounded-full bg-zinc-800 border border-zinc-700 flex items-center justify-center text-white">
                  <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M12 2C6.477 2 2 6.477 2 12c0 4.42 2.865 8.166 6.839 9.489.5.092.682-.217.682-.482 0-.237-.008-.866-.013-1.7-2.782.603-3.369-1.34-3.369-1.34-.454-1.156-1.11-1.462-1.11-1.462-.908-.62.069-.608.069-.608 1.003.07 1.531 1.03 1.531 1.03.892 1.529 2.341 1.087 2.91.831.092-.646.35-1.086.636-1.336-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.029-2.683-.103-.253-.446-1.27.098-2.647 0 0 .84-.269 2.75 1.025A9.578 9.578 0 0112 6.836c.85.004 1.705.114 2.504.336 1.909-1.294 2.747-1.025 2.747-1.025.546 1.377.203 2.394.1 2.647.64.699 1.028 1.592 1.028 2.683 0 3.842-2.339 4.687-4.566 4.935.359.309.678.919.678 1.852 0 1.336-.012 2.415-.012 2.743 0 .267.18.578.688.48C19.138 20.161 22 16.418 22 12c0-5.523-4.477-10-10-10z"/></svg>
                </div>
                <div>
                  <div class="text-sm font-medium text-white">GitHub</div>
                  <div class="text-xs text-zinc-500">未绑定</div>
                </div>
              </div>
              <button class="text-xs font-medium text-zinc-400 hover:text-white px-3 py-1.5 bg-white/5 rounded hover:bg-white/10 transition">
                去绑定
              </button>
            </div>
          </div>
        </div>

      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useMessage } from '@/utils/message'
import { changePassword, updateProfile } from '@/api/user'

const authStore = useAuthStore()
const message = useMessage()
const user = computed(() => authStore.user)

const oldPassword = ref('')
const newPassword = ref('')
const updatingPassword = ref(false)

const alertEnabled = ref(user.value?.balance_notify_enabled ?? false)
const alertThreshold = ref(user.value?.balance_notify_threshold ?? 10)

const formatDate = (dateStr?: string) => {
  if (!dateStr) return '-'
  return new Date(dateStr).toLocaleDateString('zh-CN')
}

const handleUpdatePassword = async () => {
  if (!oldPassword.value || !newPassword.value) return
  updatingPassword.value = true
  try {
    await changePassword(oldPassword.value, newPassword.value)
    message.success('密码修改成功，请妥善保管')
    oldPassword.value = ''
    newPassword.value = ''
  } catch (err: any) {
    message.error(err.response?.data?.message || '密码修改失败')
  } finally {
    updatingPassword.value = false
  }
}

const saveAlertSettings = async () => {
  try {
    await updateProfile({
      balance_notify_enabled: alertEnabled.value,
      balance_notify_threshold: alertThreshold.value
    })
    message.success('预警设置保存成功')
    authStore.checkAuth()
  } catch (err) {
    message.error('保存失败')
  }
}
</script>
