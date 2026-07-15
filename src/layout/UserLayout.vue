<template>
  <div class="h-screen bg-[#09090b] text-[#f4f4f5] flex overflow-hidden">
    <!-- Sidebar -->
    <aside :class="['border-r border-white/10 bg-[#09090b]/80 backdrop-blur-xl flex flex-col transition-all duration-300', isTauriDesktop() ? 'w-[210px]' : 'w-64']">
      <div class="h-16 flex items-center px-6 border-b border-white/10">
        <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-purple-500 to-indigo-600 flex items-center justify-center mr-3 shadow-lg shadow-purple-500/20">
          <span class="font-bold text-white text-lg">H</span>
        </div>
        <span class="font-semibold text-lg tracking-wide">Hapi</span>
      </div>
      
      <nav class="flex-1 p-4 space-y-1 overflow-y-auto">
        <router-link 
          v-for="item in menuItems" 
          :key="item.name"
          :to="item.path"
          class="flex items-center px-3 py-2.5 rounded-lg text-sm font-medium transition-all duration-200"
          :class="[
            $route.path === item.path 
              ? 'bg-white/10 text-white shadow-sm' 
              : 'text-zinc-400 hover:bg-white/5 hover:text-zinc-200'
          ]"
        >
          <component :is="item.icon" class="w-4 h-4 mr-3" :class="[$route.path === item.path ? 'text-purple-400' : '']" />
          {{ item.name }}
        </router-link>
      </nav>

      <div class="px-6 pb-4">
        <div class="flex items-center space-x-2 text-xs text-zinc-500 bg-white/5 py-2 px-3 rounded-lg border border-white/5">
          <div class="w-1.5 h-1.5 rounded-full bg-green-500 animate-pulse shadow-[0_0_8px_rgba(34,197,94,0.6)]"></div>
          <span>本站已稳定运行 <span class="text-zinc-300 font-mono font-medium">{{ uptimeDays }}</span> 天</span>
        </div>
      </div>

      <div class="p-4 border-t border-white/10">
        <div class="flex items-center p-3 rounded-xl bg-white/5 border border-white/5 cursor-pointer hover:bg-white/10 transition">
          <div class="w-8 h-8 rounded-full bg-zinc-800 flex items-center justify-center border border-white/10 mr-3">
            <UserIcon class="w-4 h-4 text-zinc-400" />
          </div>
          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium text-white truncate">{{ user?.username || 'Guest' }}</p>
            <p class="text-xs text-zinc-400 truncate">{{ user?.email || 'Not logged in' }}</p>
          </div>
        </div>
      </div>
    </aside>

    <!-- Main Content -->
    <main class="flex-1 flex flex-col min-w-0 overflow-hidden relative">
      <!-- Ambient Background Glow -->
      <div class="absolute top-0 right-0 w-[800px] h-[600px] bg-purple-600/10 rounded-full blur-[120px] pointer-events-none -translate-y-1/2 translate-x-1/3"></div>
      
      <header class="h-16 flex items-center justify-between px-8 border-b border-white/5 z-10 bg-transparent">
        <div class="flex items-center">
          <h1 class="text-xl font-semibold">{{ currentRouteName }}</h1>
        </div>
        <div class="flex items-center space-x-4">
          <router-link 
            to="/billing"
            class="hidden md:flex items-center px-3 py-1.5 bg-white/5 border border-white/10 rounded-full text-sm font-bold text-white hover:bg-white/10 hover:border-purple-500/30 transition shadow-sm shadow-purple-500/5"
          >
            <Wallet class="w-4 h-4 mr-2 text-purple-400" />
            ¥{{ (user?.balance || 0).toLocaleString() }}
          </router-link>

          <button 
            @click="announcementStore.openModal()"
            class="relative p-2 bg-white/5 border border-white/10 rounded-lg hover:bg-white/10 transition text-zinc-300 ml-2"
            title="系统公告"
          >
            <BellIcon class="w-5 h-5" />
            <span v-if="announcementStore.unreadCount > 0" class="absolute top-1.5 right-1.5 w-2 h-2 bg-red-500 rounded-full border border-[#09090b]"></span>
          </button>
          
          <button @click="handleLogout" class="p-2 bg-white/5 border border-white/10 rounded-lg hover:bg-white/10 transition text-zinc-300 ml-2" title="退出登录">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"></path></svg>
          </button>
        </div>
      </header>

      <div class="flex-1 overflow-y-auto p-8 z-10">
        <div class="w-full">
          <router-view v-slot="{ Component }">
            <transition name="fade" mode="out-in">
              <component :is="Component" />
            </transition>
          </router-view>
        </div>
      </div>
    </main>
    
    <AnnouncementsModal 
      :is-open="announcementStore.isModalOpen" 
      @close="announcementStore.closeModal()" 
    />
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { useAnnouncementStore } from '@/stores/announcements'
import { isTauriDesktop } from '@/utils/desktop-env'
import { showConfirm } from '@/utils/message'
import { getStableUptimeDays } from '@/utils/uptime'
import { 
  LayoutDashboard, 
  Key, 
  BarChart3, 
  Settings,
  Server,
  User as UserIcon,
  Bell as BellIcon,
  Users,
  Wallet,
  Activity,
  Bookmark,
  SlidersHorizontal,
  Image as ImageIcon
} from '@lucide/vue'
import AnnouncementsModal from '@/components/AnnouncementsModal.vue'

const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()
const announcementStore = useAnnouncementStore()
const user = computed(() => authStore.user)

onMounted(() => {
  if (authStore.user) {
    announcementStore.fetchAnnouncements()
  }
})

const handleLogout = () => {
  showConfirm({
    title: '退出登录',
    content: '确定要退出当前账号吗？',
    onConfirm: async () => {
      await authStore.logout()
      router.push('/login')
    }
  })
}

const currentRouteName = computed(() => route.name || 'Dashboard')

const uptimeDays = computed(() => getStableUptimeDays())

const menuItems = computed(() => {
  return [
    { name: '仪表盘', path: '/', icon: LayoutDashboard },
    { name: '我的订阅', path: '/subscriptions', icon: Bookmark },
    { name: 'API 密钥', path: '/keys', icon: Key },
    { name: '图片创作', path: '/image-creation', icon: ImageIcon },
    { name: '一键配置', path: '/client-setup', icon: SlidersHorizontal },
    { name: '可用模型', path: '/models', icon: Server },
    { name: '渠道状态', path: '/status', icon: Activity },
    { name: '使用明细', path: '/usage', icon: BarChart3 },
    { name: '钱包与套餐', path: '/billing', icon: Wallet },
    { name: '邀请返利', path: '/affiliate', icon: Users },
    { name: '个人设置', path: '/profile', icon: Settings }
  ]
})
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateY(10px);
}
</style>
