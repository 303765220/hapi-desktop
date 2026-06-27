<template>
  <div>
    <!-- Toast Messages -->
    <div class="fixed top-5 left-1/2 -translate-x-1/2 z-[100] flex flex-col items-center space-y-2 pointer-events-none">
      <TransitionGroup name="toast">
        <div 
          v-for="msg in messages" 
          :key="msg.id"
          class="px-4 py-3 rounded-lg shadow-lg shadow-black/50 border flex items-center space-x-3 text-sm font-medium pointer-events-auto backdrop-blur-xl"
          :class="{
            'bg-green-500/10 border-green-500/20 text-green-400': msg.type === 'success',
            'bg-red-500/10 border-red-500/20 text-red-400': msg.type === 'error',
            'bg-amber-500/10 border-amber-500/20 text-amber-400': msg.type === 'warning',
            'bg-blue-500/10 border-blue-500/20 text-blue-400': msg.type === 'info',
          }"
        >
          <CheckCircle v-if="msg.type === 'success'" class="w-4 h-4 shrink-0" />
          <AlertCircle v-else-if="msg.type === 'error'" class="w-4 h-4 shrink-0" />
          <AlertTriangle v-else-if="msg.type === 'warning'" class="w-4 h-4 shrink-0" />
          <Info v-else class="w-4 h-4 shrink-0" />
          
          <span>{{ msg.text }}</span>
        </div>
      </TransitionGroup>
    </div>

    <!-- Confirm Modal -->
    <Transition name="fade">
      <div v-if="confirmState" class="fixed inset-0 z-[100] flex items-center justify-center p-4">
        <!-- Backdrop -->
        <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="handleCancel"></div>
        
        <!-- Modal Content -->
        <div class="glass-panel p-6 max-w-sm w-full relative z-10 shadow-2xl border-white/10">
          <div class="flex items-center space-x-3 mb-4 text-white">
            <div class="w-10 h-10 rounded-full bg-purple-500/10 flex items-center justify-center border border-purple-500/20">
              <HelpCircle class="w-5 h-5 text-purple-400" />
            </div>
            <h3 class="text-lg font-semibold">{{ confirmState.title }}</h3>
          </div>
          
          <p class="text-sm text-zinc-300 mb-6 leading-relaxed">
            {{ confirmState.content }}
          </p>
          
          <div class="flex space-x-3 justify-end">
            <button 
              @click="handleCancel"
              class="px-4 py-2 bg-white/5 hover:bg-white/10 text-white text-sm font-medium rounded-lg transition border border-white/10"
            >
              {{ confirmState.cancelText || '取消' }}
            </button>
            <button 
              @click="handleConfirm"
              class="px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white text-sm font-medium rounded-lg transition shadow-lg shadow-purple-500/20"
            >
              {{ confirmState.confirmText || '确定' }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { messages, confirmState, closeConfirm } from '@/utils/message'
import { CheckCircle, AlertCircle, AlertTriangle, Info, HelpCircle } from '@lucide/vue'

const handleConfirm = () => {
  if (confirmState.value?.onConfirm) {
    confirmState.value.onConfirm()
  }
  closeConfirm()
}

const handleCancel = () => {
  if (confirmState.value?.onCancel) {
    confirmState.value.onCancel()
  }
  closeConfirm()
}
</script>

<style scoped>
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}
.toast-enter-from {
  opacity: 0;
  transform: translateY(-20px);
}
.toast-leave-to {
  opacity: 0;
  transform: translateY(-20px);
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
