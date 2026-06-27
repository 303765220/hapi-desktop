<template>
  <div class="min-h-screen w-full flex bg-[#09090b]">
    <!-- Left Side: Branding / Graphic -->
    <div class="hidden lg:flex lg:w-1/2 relative bg-zinc-950/50 overflow-hidden items-center justify-center border-r border-white/5">
      <!-- Cool ambient glowing orbs -->
      <div class="absolute top-0 left-0 w-full h-full">
        <div class="absolute top-[20%] left-[20%] w-[500px] h-[500px] bg-purple-600/20 rounded-full blur-[100px] mix-blend-screen animate-blob"></div>
        <div class="absolute top-[30%] right-[20%] w-[400px] h-[400px] bg-indigo-600/20 rounded-full blur-[100px] mix-blend-screen animate-blob animation-delay-2000"></div>
        <div class="absolute bottom-[20%] left-[30%] w-[600px] h-[600px] bg-fuchsia-600/20 rounded-full blur-[100px] mix-blend-screen animate-blob animation-delay-4000"></div>
      </div>
      
      <!-- Brand Text -->
      <div class="relative z-10 p-12 text-center">
        <div class="w-20 h-20 mx-auto rounded-2xl bg-gradient-to-br from-purple-500 to-indigo-600 flex items-center justify-center shadow-[0_0_30px_rgba(168,85,247,0.3)] mb-8 border border-white/10">
          <span class="font-bold text-white text-4xl tracking-tighter">H</span>
        </div>
        <h1 class="text-4xl lg:text-5xl font-extrabold text-white tracking-tight mb-6 leading-tight">
          探索无限可能 <br/>
          <span class="text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-indigo-400">Hapi 统一分发平台</span>
        </h1>
        <p class="text-lg text-zinc-400 max-w-md mx-auto leading-relaxed">
          最先进、优雅且安全的控制台，一站式管理您的 AI 模型、接口密钥与订阅配置。
        </p>
      </div>

      <!-- Glassmorphism decorative grid -->
      <div class="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.03] pointer-events-none"></div>
      
      <!-- Bottom decorative cards -->
      <div class="absolute -bottom-24 left-1/2 -translate-x-1/2 flex gap-6 pointer-events-none opacity-30 w-[120%] justify-center">
        <div class="h-48 w-64 glass-panel rounded-2xl border border-white/10 transform -rotate-6 translate-y-12 shadow-2xl"></div>
        <div class="h-64 w-80 glass-panel rounded-2xl border border-white/10 transform -translate-y-4 shadow-2xl"></div>
        <div class="h-48 w-64 glass-panel rounded-2xl border border-white/10 transform rotate-6 translate-y-16 shadow-2xl"></div>
      </div>
    </div>

    <!-- Right Side: Login Form -->
    <div class="w-full lg:w-1/2 flex items-center justify-center p-8 sm:p-12 lg:p-24 relative bg-[#09090b]">
      <!-- Ambient glow for mobile when left side is hidden -->
      <div class="lg:hidden absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-purple-600/10 rounded-full blur-[120px] pointer-events-none"></div>
      
      <div class="w-full max-w-[420px] relative z-10">
        <div class="lg:hidden w-12 h-12 mb-8 rounded-xl bg-gradient-to-br from-purple-500 to-indigo-600 flex items-center justify-center shadow-lg shadow-purple-500/30">
          <span class="font-bold text-white text-2xl">H</span>
        </div>
        
        <h2 class="text-3xl font-bold text-white mb-2 tracking-tight">欢迎回来</h2>
        <p class="text-zinc-400 mb-10 text-sm">请输入您的凭据以登录到控制台。</p>

        <form @submit.prevent="handleLogin" class="space-y-6">
          <div class="space-y-2">
            <label class="block text-sm font-medium text-zinc-300">邮箱 / 用户名</label>
            <div class="relative">
              <input 
                v-model="email" 
                type="text" 
                required
                autocomplete="username"
                class="w-full bg-zinc-900/50 border border-white/10 rounded-xl px-4 py-3.5 text-white placeholder-zinc-600 focus:outline-none focus:ring-2 focus:ring-purple-500/50 focus:border-purple-500/80 transition-all hover:bg-zinc-900"
                placeholder="请输入邮箱或用户名"
              />
            </div>
          </div>
          
          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <label class="block text-sm font-medium text-zinc-300">密码</label>
            </div>
            <div class="relative">
              <input 
                v-model="password" 
                :type="showPassword ? 'text' : 'password'"
                required
                autocomplete="current-password"
                class="w-full bg-zinc-900/50 border border-white/10 rounded-xl px-4 py-3.5 pr-12 text-white placeholder-zinc-600 focus:outline-none focus:ring-2 focus:ring-purple-500/50 focus:border-purple-500/80 transition-all hover:bg-zinc-900"
                placeholder="••••••••"
              />
              <button
                type="button"
                class="absolute right-3 top-1/2 -translate-y-1/2 rounded-lg p-1.5 text-zinc-500 transition hover:bg-white/5 hover:text-zinc-200 focus:outline-none focus:ring-2 focus:ring-purple-500/40"
                :title="showPassword ? '隐藏密码' : '显示密码'"
                :aria-label="showPassword ? '隐藏密码' : '显示密码'"
                @click="showPassword = !showPassword"
              >
                <EyeOff v-if="showPassword" class="h-4 w-4" />
                <Eye v-else class="h-4 w-4" />
              </button>
            </div>
          </div>

          <label v-if="isDesktop" class="flex items-center justify-between gap-4 text-sm text-zinc-400">
            <span class="flex items-center gap-2">
              <input
                v-model="rememberPassword"
                type="checkbox"
                class="h-4 w-4 rounded border-white/10 bg-zinc-900 text-purple-500 focus:ring-purple-500/50"
              />
              记住密码
            </span>
            <span class="text-xs text-zinc-600">保存到本机</span>
          </label>

          <button 
            type="submit" 
            :disabled="loading"
            class="w-full bg-white text-black font-bold rounded-xl px-4 py-3.5 mt-8 hover:bg-zinc-200 transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center shadow-[0_0_15px_rgba(255,255,255,0.1)] hover:shadow-[0_0_25px_rgba(255,255,255,0.25)]"
          >
            <span v-if="!loading">登 录</span>
            <Loader2 v-else class="w-5 h-5 animate-spin" />
          </button>
        </form>
        
        <div v-if="errorMsg" class="mt-6 p-4 bg-red-500/10 border border-red-500/20 rounded-xl text-red-400 text-sm text-center font-medium animate-in fade-in slide-in-from-top-2 duration-300">
          {{ errorMsg }}
        </div>
        
        <p class="mt-10 text-center text-sm text-zinc-500">
          还没有账号？ 
          <a href="#" class="font-medium text-zinc-300 hover:text-white transition underline decoration-white/20 underline-offset-4 hover:decoration-white/60">联系管理员</a>
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { Eye, EyeOff, Loader2 } from '@lucide/vue'
import { isTauriDesktop } from '@/utils/desktop-env'
import { loadSavedCredentials, saveCredentialsAfterLogin } from '@/services/saved-credentials'

const router = useRouter()
const authStore = useAuthStore()

const email = ref('')
const password = ref('')
const loading = ref(false)
const errorMsg = ref('')
const isDesktop = isTauriDesktop()
const rememberPassword = ref(false)
const showPassword = ref(false)

onMounted(async () => {
  if (!isDesktop) {
    return
  }

  try {
    const saved = await loadSavedCredentials()
    if (saved) {
      email.value = saved.email
      password.value = saved.password
      rememberPassword.value = true
    }
  } catch {
    // 本地凭据文件不可用时不阻断登录；用户仍可手动输入账号密码。
  }
})

const handleLogin = async () => {
  loading.value = true
  errorMsg.value = ''
  
  try {
    await authStore.login({ email: email.value, password: password.value })
    await saveCredentialsAfterLogin(isDesktop, undefined, rememberPassword.value, {
      email: email.value,
      password: password.value
    })
    router.push('/')
  } catch (err: any) {
    errorMsg.value = err.response?.data?.message || '登录失败，请检查您的账号和密码。'
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
@keyframes blob {
  0% { transform: translate(0px, 0px) scale(1); }
  33% { transform: translate(30px, -50px) scale(1.1); }
  66% { transform: translate(-20px, 20px) scale(0.9); }
  100% { transform: translate(0px, 0px) scale(1); }
}
.animate-blob {
  animation: blob 7s infinite;
}
.animation-delay-2000 {
  animation-delay: 2s;
}
.animation-delay-4000 {
  animation-delay: 4s;
}
</style>
