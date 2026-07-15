<template>
  <div class="min-h-screen w-full flex bg-[#09090b]">
    <div class="hidden lg:flex lg:w-1/2 relative bg-zinc-950/50 overflow-hidden items-center justify-center border-r border-white/5">
      <div class="absolute top-0 left-0 w-full h-full">
        <div class="absolute top-[20%] left-[20%] w-[500px] h-[500px] bg-purple-600/20 rounded-full blur-[100px] mix-blend-screen animate-blob"></div>
        <div class="absolute top-[30%] right-[20%] w-[400px] h-[400px] bg-indigo-600/20 rounded-full blur-[100px] mix-blend-screen animate-blob animation-delay-2000"></div>
        <div class="absolute bottom-[20%] left-[30%] w-[600px] h-[600px] bg-fuchsia-600/20 rounded-full blur-[100px] mix-blend-screen animate-blob animation-delay-4000"></div>
      </div>

      <div class="relative z-10 p-12 text-center">
        <div class="w-20 h-20 mx-auto rounded-2xl bg-gradient-to-br from-purple-500 to-indigo-600 flex items-center justify-center shadow-[0_0_30px_rgba(168,85,247,0.3)] mb-8 border border-white/10">
          <span class="font-bold text-white text-4xl tracking-tighter">H</span>
        </div>
        <h1 class="text-4xl lg:text-5xl font-extrabold text-white tracking-tight mb-6 leading-tight">
          开始使用 Hapi <br/>
          <span class="text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-indigo-400">统一 AI 接口入口</span>
        </h1>
        <p class="text-lg text-zinc-400 max-w-md mx-auto leading-relaxed">
          创建账号后即可管理 API 密钥、查看用量，并配置常用客户端。
        </p>
      </div>

      <div class="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.03] pointer-events-none"></div>
    </div>

    <div class="w-full lg:w-1/2 flex items-center justify-center p-8 sm:p-12 lg:p-24 relative bg-[#09090b]">
      <div class="lg:hidden absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-purple-600/10 rounded-full blur-[120px] pointer-events-none"></div>

      <div class="w-full max-w-[420px] relative z-10">
        <div class="lg:hidden w-12 h-12 mb-8 rounded-xl bg-gradient-to-br from-purple-500 to-indigo-600 flex items-center justify-center shadow-lg shadow-purple-500/30">
          <span class="font-bold text-white text-2xl">H</span>
        </div>

        <h2 class="text-3xl font-bold text-white mb-2 tracking-tight">创建账号</h2>
        <p class="text-zinc-400 mb-10 text-sm">填写邮箱和密码，注册后将自动进入控制台。</p>

        <form @submit.prevent="handleRegister" class="space-y-5">
          <div class="space-y-2">
            <label class="block text-sm font-medium text-zinc-300">邮箱</label>
            <input
              v-model="email"
              type="email"
              required
              autocomplete="email"
              class="w-full bg-zinc-900/50 border border-white/10 rounded-xl px-4 py-3.5 text-white placeholder-zinc-600 focus:outline-none focus:ring-2 focus:ring-purple-500/50 focus:border-purple-500/80 transition-all hover:bg-zinc-900"
              placeholder="请输入邮箱"
            />
          </div>

          <div class="space-y-2">
            <label class="block text-sm font-medium text-zinc-300">密码</label>
            <div class="relative">
              <input
                v-model="password"
                :type="showPassword ? 'text' : 'password'"
                required
                autocomplete="new-password"
                class="w-full bg-zinc-900/50 border border-white/10 rounded-xl px-4 py-3.5 pr-12 text-white placeholder-zinc-600 focus:outline-none focus:ring-2 focus:ring-purple-500/50 focus:border-purple-500/80 transition-all hover:bg-zinc-900"
                placeholder="至少 6 位"
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

          <div class="space-y-2">
            <label class="block text-sm font-medium text-zinc-300">确认密码</label>
            <input
              v-model="confirmPassword"
              :type="showPassword ? 'text' : 'password'"
              required
              autocomplete="new-password"
              class="w-full bg-zinc-900/50 border border-white/10 rounded-xl px-4 py-3.5 text-white placeholder-zinc-600 focus:outline-none focus:ring-2 focus:ring-purple-500/50 focus:border-purple-500/80 transition-all hover:bg-zinc-900"
              placeholder="请再次输入密码"
            />
          </div>

          <div class="space-y-2">
            <label class="block text-sm font-medium text-zinc-300">邀请码（可选）</label>
            <input
              v-model="invitationCode"
              type="text"
              autocomplete="off"
              class="w-full bg-zinc-900/50 border border-white/10 rounded-xl px-4 py-3.5 text-white placeholder-zinc-600 focus:outline-none focus:ring-2 focus:ring-purple-500/50 focus:border-purple-500/80 transition-all hover:bg-zinc-900"
              placeholder="如有邀请码请填写"
            />
          </div>

          <button
            type="submit"
            :disabled="loading"
            class="w-full bg-white text-black font-bold rounded-xl px-4 py-3.5 mt-8 hover:bg-zinc-200 transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center shadow-[0_0_15px_rgba(255,255,255,0.1)] hover:shadow-[0_0_25px_rgba(255,255,255,0.25)]"
          >
            <span v-if="!loading">注 册</span>
            <Loader2 v-else class="w-5 h-5 animate-spin" />
          </button>
        </form>

        <div v-if="errorMsg" class="mt-6 p-4 bg-red-500/10 border border-red-500/20 rounded-xl text-red-400 text-sm text-center font-medium animate-in fade-in slide-in-from-top-2 duration-300">
          {{ errorMsg }}
        </div>

        <p class="mt-10 text-center text-sm text-zinc-500">
          已有账号？
          <RouterLink to="/login" class="font-medium text-zinc-300 hover:text-white transition underline decoration-white/20 underline-offset-4 hover:decoration-white/60">返回登录</RouterLink>
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { Eye, EyeOff, Loader2 } from '@lucide/vue'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const authStore = useAuthStore()

const email = ref('')
const password = ref('')
const confirmPassword = ref('')
const invitationCode = ref('')
const showPassword = ref(false)
const loading = ref(false)
const errorMsg = ref('')

const handleRegister = async () => {
  errorMsg.value = ''
  if (password.value !== confirmPassword.value) {
    errorMsg.value = '两次输入的密码不一致。'
    return
  }

  loading.value = true
  try {
    await authStore.register({
      email: email.value,
      password: password.value,
      invitation_code: invitationCode.value.trim() || undefined
    })
    router.push('/')
  } catch (err: any) {
    errorMsg.value = err.response?.data?.message || err.response?.data?.detail || '注册失败，请检查填写的信息。'
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
