<template>
  <Transition name="popup">
    <div 
      v-if="store.currentPopup" 
      class="fixed bottom-6 right-6 z-[100] w-80 sm:w-96 glass-panel overflow-hidden shadow-2xl shadow-purple-500/10 border-purple-500/30"
    >
      <div class="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-purple-500 to-indigo-500"></div>
      
      <div class="p-5">
        <div class="flex justify-between items-start mb-3">
          <h3 class="text-base font-bold text-white flex items-center">
            <BellRing class="w-4 h-4 mr-2 text-purple-400" />
            {{ store.currentPopup.title }}
          </h3>
          <button 
            @click="store.dismissPopup()"
            class="text-zinc-500 hover:text-white transition p-1"
          >
            <X class="w-4 h-4" />
          </button>
        </div>
        
        <div 
          class="text-sm text-zinc-300 mb-4 line-clamp-3 leading-relaxed markdown-body"
          v-html="renderMarkdown(store.currentPopup.content)"
        ></div>
        
        <div class="flex space-x-2">
          <button
            @click="viewDetails"
            class="flex-1 px-4 py-2 bg-purple-600/20 hover:bg-purple-600/40 text-purple-300 text-xs font-medium rounded-lg border border-purple-500/30 transition text-center"
          >
            查看详情
          </button>
          <button 
            @click="store.dismissPopup()"
            class="flex-1 px-4 py-2 bg-white/5 hover:bg-white/10 text-white text-xs font-medium rounded-lg border border-white/10 transition"
          >
            我知道了
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { BellRing, X } from '@lucide/vue'
import { useAnnouncementStore } from '@/stores/announcements'
import { marked } from 'marked'
import DOMPurify from 'dompurify'

const store = useAnnouncementStore()

const renderMarkdown = (content: string) => {
  if (!content) return ''
  const rawHtml = marked.parse(content)
  return DOMPurify.sanitize(rawHtml as string)
}

const viewDetails = () => {
  store.openModal()
  store.dismissPopup()
}
</script>

<style scoped>
.popup-enter-active,
.popup-leave-active {
  transition: all 0.4s cubic-bezier(0.16, 1, 0.3, 1);
}
.popup-enter-from {
  opacity: 0;
  transform: translateY(20px) scale(0.95);
}
.popup-leave-to {
  opacity: 0;
  transform: translateY(20px) scale(0.95);
}
</style>
