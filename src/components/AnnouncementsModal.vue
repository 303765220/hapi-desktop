<template>
  <Transition name="fade">
    <div v-if="isOpen" class="fixed inset-0 z-[100] flex items-center justify-center p-4 sm:p-6">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="close"></div>
      
      <div class="glass-panel w-full max-w-3xl max-h-[85vh] flex flex-col relative z-10 shadow-2xl border-purple-500/20 overflow-hidden">
        <!-- Header -->
        <div class="px-6 py-5 border-b border-white/10 flex justify-between items-center bg-white/[0.02]">
          <div>
            <h2 class="text-xl font-bold text-white flex items-center">
              <BellRing class="w-5 h-5 mr-2 text-purple-400" />
              系统公告
            </h2>
            <p class="text-xs text-zinc-400 mt-1">查看平台的最新动态和重要通知</p>
          </div>
          <div class="flex items-center space-x-3">
            <button 
              v-if="store.unreadCount > 0"
              @click="store.markAllAsRead()"
              class="px-3 py-1.5 bg-purple-500/10 hover:bg-purple-500/20 text-purple-400 text-xs font-medium rounded-lg transition border border-purple-500/20 flex items-center"
            >
              <CheckCheck class="w-3.5 h-3.5 mr-1" />
              全部已读
            </button>
            <button @click="close" class="p-2 text-zinc-400 hover:text-white bg-white/5 hover:bg-white/10 rounded-lg transition">
              <X class="w-5 h-5" />
            </button>
          </div>
        </div>

        <!-- Body -->
        <div class="flex-1 overflow-y-auto p-6 space-y-6 custom-scrollbar bg-black/20">
          <div v-if="store.loading" class="space-y-4">
            <div v-for="i in 3" :key="i" class="glass-panel p-6 animate-pulse">
              <div class="h-6 bg-white/10 w-1/3 rounded mb-4"></div>
              <div class="h-4 bg-white/5 w-full rounded mb-2"></div>
              <div class="h-4 bg-white/5 w-2/3 rounded"></div>
            </div>
          </div>
          
          <div v-else-if="store.announcements.length === 0" class="flex flex-col items-center justify-center py-16 text-zinc-500">
            <Bell class="w-12 h-12 mb-4 opacity-50" />
            <p>暂无系统公告</p>
          </div>

          <div v-else class="space-y-4">
            <Transition name="popup" mode="out-in">
              <div 
                v-if="store.announcements[currentIndex]"
                :key="currentIndex"
              class="glass-panel p-6 relative overflow-hidden transition-colors border-l-4"
              :class="!store.announcements[currentIndex].read_at ? 'border-l-purple-500' : 'border-l-transparent'"
            >
              <div class="flex flex-col sm:flex-row justify-between sm:items-center gap-2 mb-6">
                <div class="flex items-center space-x-3">
                  <h3 class="text-xl font-bold" :class="!store.announcements[currentIndex].read_at ? 'text-white' : 'text-zinc-300'">{{ store.announcements[currentIndex].title }}</h3>
                  <span v-if="!store.announcements[currentIndex].read_at" class="px-2 py-0.5 bg-purple-500/20 text-purple-400 text-[10px] font-bold rounded-full border border-purple-500/30">
                    NEW
                  </span>
                </div>
                <span class="text-xs text-zinc-500 font-mono">{{ formatDate(store.announcements[currentIndex].starts_at) }}</span>
              </div>
              
              <div 
                class="markdown-body text-sm text-zinc-300 leading-relaxed bg-black/30 p-5 rounded-xl border border-white/5 min-h-[200px]"
                v-html="renderMarkdown(store.announcements[currentIndex].content)"
              ></div>
              
              <div class="mt-6 flex justify-between items-center">
                <div class="text-xs text-zinc-500 font-medium">
                  {{ currentIndex + 1 }} / {{ store.announcements.length }}
                </div>
                <div class="flex items-center space-x-3">
                  <button 
                    v-if="!store.announcements[currentIndex].read_at"
                    @click="store.markAsRead(store.announcements[currentIndex].id)"
                    class="text-xs font-medium text-purple-400 hover:text-purple-300 transition flex items-center bg-purple-500/10 px-4 py-2 rounded-lg border border-purple-500/20"
                  >
                    <Check class="w-4 h-4 mr-1.5" />
                    标记为已读
                  </button>
                  <button 
                    @click="nextAnnouncement"
                    class="px-5 py-2 bg-purple-600 hover:bg-purple-500 text-white text-sm font-medium rounded-lg transition"
                  >
                    {{ currentIndex < store.announcements.length - 1 ? '下一条' : '我知道了' }}
                  </button>
                </div>
              </div>
              </div>
            </Transition>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { BellRing, X, CheckCheck, Check, Bell } from '@lucide/vue'
import { useAnnouncementStore } from '@/stores/announcements'
import { marked } from 'marked'
import DOMPurify from 'dompurify'

const currentIndex = ref(0)

const props = defineProps<{
  isOpen: boolean
}>()

const emit = defineEmits<{
  (e: 'close'): void
}>()

const store = useAnnouncementStore()

const close = () => {
  emit('close')
}

const nextAnnouncement = () => {
  if (currentIndex.value < store.announcements.length - 1) {
    // If not read, automatically mark as read when proceeding to next
    if (!store.announcements[currentIndex.value].read_at) {
      store.markAsRead(store.announcements[currentIndex.value].id)
    }
    currentIndex.value++
  } else {
    // Last one, close modal
    if (!store.announcements[currentIndex.value].read_at) {
      store.markAsRead(store.announcements[currentIndex.value].id)
    }
    close()
  }
}

watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    currentIndex.value = 0
    store.fetchAnnouncements(true)
  }
})

const renderMarkdown = (content: string) => {
  if (!content) return ''
  const rawHtml = marked.parse(content)
  return DOMPurify.sanitize(rawHtml as string)
}

const formatDate = (dateString?: string) => {
  if (!dateString) return ''
  const date = new Date(dateString)
  return new Intl.DateTimeFormat('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  }).format(date)
}
</script>

<style>
/* Custom Scrollbar for modal content */
.overflow-y-auto::-webkit-scrollbar {
  width: 6px;
}
.overflow-y-auto::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.1);
  border-radius: 4px;
}
.overflow-y-auto::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 4px;
}
.overflow-y-auto::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.2);
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.popup-enter-active,
.popup-leave-active {
  transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}
.popup-enter-from {
  opacity: 0;
  transform: translateY(20px) scale(0.95);
}
.popup-leave-to {
  opacity: 0;
  transform: translateY(-20px) scale(0.95);
}
</style>
