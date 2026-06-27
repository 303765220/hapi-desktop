<template>
  <div class="h-screen bg-[#09090b] text-zinc-100 flex flex-col font-sans selection:bg-purple-500/30 relative overflow-hidden">
    
    <!-- Top Navbar -->
    <header class="h-16 border-b border-white/10 bg-black/50 backdrop-blur-xl sticky top-0 z-40 flex items-center justify-between px-6">
      <div class="flex items-center space-x-4">
        <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-purple-500 to-indigo-600 flex items-center justify-center shadow-lg">
          <span class="font-bold text-white text-sm">S</span>
        </div>
        <h1 class="text-lg font-bold tracking-wide text-white">Sub2API Admin</h1>
        <span class="text-xs px-2 py-0.5 bg-red-500/10 text-red-400 border border-red-500/20 rounded font-medium ml-2">v0.2.0</span>
      </div>
      
      <div class="flex items-center space-x-4">
        <router-link to="/" class="text-sm font-medium text-zinc-400 hover:text-white transition flex items-center">
          <ExternalLink class="w-4 h-4 mr-2" />
          返回前台
        </router-link>
        <div class="h-4 w-px bg-white/10 mx-2"></div>
        <div class="flex items-center space-x-2">
          <div class="w-8 h-8 rounded-full bg-zinc-800 flex items-center justify-center border border-white/10">
            <span class="text-xs font-bold text-white">{{ user?.username?.charAt(0).toUpperCase() || 'A' }}</span>
          </div>
        </div>
      </div>
    </header>

    <div class="flex flex-1 overflow-hidden">
      <!-- Sidebar -->
      <aside class="w-64 border-r border-white/10 bg-black/20 flex flex-col">
        <div class="p-4 flex-1 overflow-y-auto space-y-1 custom-scrollbar">
          <div class="text-xs font-semibold text-zinc-500 uppercase tracking-wider mb-2 mt-4 px-3">系统概览</div>
          <router-link 
            v-for="item in overviewMenu" 
            :key="item.name"
            :to="item.path"
            class="flex items-center px-3 py-2 rounded-lg text-sm transition-colors group relative"
            :class="route.path === item.path ? 'bg-white/10 text-white font-medium' : 'text-zinc-400 hover:bg-white/5 hover:text-zinc-200'"
          >
            <div 
              class="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-0 bg-purple-500 rounded-r transition-all duration-300"
              :class="route.path === item.path ? 'h-5' : 'h-0'"
            ></div>
            <component :is="item.icon" class="w-4 h-4 mr-3" :class="route.path === item.path ? 'text-purple-400' : 'text-zinc-500 group-hover:text-zinc-400'" />
            {{ item.name }}
          </router-link>

          <div class="text-xs font-semibold text-zinc-500 uppercase tracking-wider mb-2 mt-6 px-3">核心资产</div>
          <router-link 
            v-for="item in coreMenu" 
            :key="item.name"
            :to="item.path"
            class="flex items-center px-3 py-2 rounded-lg text-sm transition-colors group relative"
            :class="route.path === item.path ? 'bg-white/10 text-white font-medium' : 'text-zinc-400 hover:bg-white/5 hover:text-zinc-200'"
          >
            <div 
              class="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-0 bg-purple-500 rounded-r transition-all duration-300"
              :class="route.path === item.path ? 'h-5' : 'h-0'"
            ></div>
            <component :is="item.icon" class="w-4 h-4 mr-3" :class="route.path === item.path ? 'text-purple-400' : 'text-zinc-500 group-hover:text-zinc-400'" />
            {{ item.name }}
          </router-link>

          <div class="text-xs font-semibold text-zinc-500 uppercase tracking-wider mb-2 mt-6 px-3">系统与安全</div>
          <router-link 
            v-for="item in systemMenu" 
            :key="item.name"
            :to="item.path"
            class="flex items-center px-3 py-2 rounded-lg text-sm transition-colors group relative"
            :class="route.path === item.path ? 'bg-white/10 text-white font-medium' : 'text-zinc-400 hover:bg-white/5 hover:text-zinc-200'"
          >
            <div 
              class="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-0 bg-purple-500 rounded-r transition-all duration-300"
              :class="route.path === item.path ? 'h-5' : 'h-0'"
            ></div>
            <component :is="item.icon" class="w-4 h-4 mr-3" :class="route.path === item.path ? 'text-purple-400' : 'text-zinc-500 group-hover:text-zinc-400'" />
            {{ item.name }}
          </router-link>
        </div>
      </aside>

      <!-- Main Content -->
      <main class="flex-1 overflow-y-auto relative p-8">
        <router-view v-slot="{ Component }">
          <transition name="fade" mode="out-in">
            <component :is="Component" />
          </transition>
        </router-view>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { 
  LayoutDashboard, 
  Users, 
  Server,
  Layers,
  ShieldAlert,
  Settings,
  ExternalLink,
  Activity,
  Ticket
} from '@lucide/vue'

const route = useRoute()
const authStore = useAuthStore()
const user = computed(() => authStore.user)

const overviewMenu = [
  { name: '仪表盘', path: '/admin', icon: LayoutDashboard },
  { name: '数据分析', path: '/admin/analytics', icon: Activity },
]

const coreMenu = [
  { name: '用户管理', path: '/admin/users', icon: Users },
  { name: '渠道管理', path: '/admin/channels', icon: Server },
  { name: '路由分组', path: '/admin/groups', icon: Layers },
  { name: '兑换码', path: '/admin/redeem', icon: Ticket },
]

const systemMenu = [
  { name: '风控系统', path: '/admin/risk', icon: ShieldAlert },
  { name: '系统设置', path: '/admin/settings', icon: Settings },
]
</script>

<style scoped>

.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 4px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.2);
}
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateY(5px);
}
</style>
