<template>
  <div class="space-y-6">
    <div class="flex flex-col gap-4 lg:flex-row lg:items-center lg:justify-between">
      <div>
        <h2 class="text-2xl font-semibold text-white">Codex 安装器</h2>
        <p class="mt-1 text-sm text-zinc-400">读取安装清单，下载校验后在本机执行 Codex 安装。</p>
      </div>
      <button
        v-if="isDesktop"
        @click="refreshStatus"
        class="inline-flex items-center justify-center rounded-lg border border-white/10 bg-white/5 px-4 py-2 text-sm font-medium text-white transition hover:bg-white/10 disabled:opacity-50"
        :disabled="loading"
      >
        <Loader2 v-if="loading" class="mr-2 h-4 w-4 animate-spin" />
        <RefreshCw v-else class="mr-2 h-4 w-4" />
        重新检测
      </button>
    </div>

    <div class="glass-panel p-6">
      <div v-if="!isDesktop" class="flex items-start gap-4">
        <div class="rounded-lg bg-amber-500/10 p-3 text-amber-400">
          <MonitorDown class="h-6 w-6" />
        </div>
        <div>
          <h3 class="text-lg font-semibold text-white">需要桌面客户端</h3>
          <p class="mt-2 text-sm leading-6 text-zinc-400">Codex 安装器需要调用本机下载、校验和安装能力。Web 页面保持原有功能，不执行本机安装。</p>
        </div>
      </div>

      <div v-else class="space-y-6">
        <div class="grid gap-4 md:grid-cols-5">
          <div class="rounded-lg border border-white/10 bg-black/20 p-4">
            <div class="text-xs font-medium text-zinc-500">系统</div>
            <div class="mt-2 text-sm font-semibold text-white">{{ status?.platform || '-' }}</div>
          </div>
          <div class="rounded-lg border border-white/10 bg-black/20 p-4">
            <div class="text-xs font-medium text-zinc-500">架构</div>
            <div class="mt-2 text-sm font-semibold text-white">{{ status?.architecture || '-' }}</div>
          </div>
          <div class="rounded-lg border border-white/10 bg-black/20 p-4">
            <div class="text-xs font-medium text-zinc-500">安装状态</div>
            <div class="mt-2 text-sm font-semibold" :class="status?.installed ? 'text-green-400' : 'text-amber-400'">
              {{ status?.installed ? '已检测到' : '未检测到' }}
            </div>
          </div>
          <div class="rounded-lg border border-white/10 bg-black/20 p-4">
            <div class="text-xs font-medium text-zinc-500">安装方式</div>
            <div class="mt-2 text-sm font-semibold text-purple-300">自动安装</div>
          </div>
          <div class="rounded-lg border border-white/10 bg-black/20 p-4">
            <div class="text-xs font-medium text-zinc-500">校验</div>
            <div class="mt-2 text-sm font-semibold" :class="status?.verified ? 'text-green-400' : 'text-zinc-400'">
              {{ status?.verified ? '已通过' : '待执行' }}
            </div>
          </div>
        </div>

        <div class="rounded-lg border border-white/10 bg-black/30 p-4">
          <div class="flex items-start gap-3">
            <Info class="mt-0.5 h-5 w-5 shrink-0 text-purple-400" />
            <div class="min-w-0">
              <div class="text-sm font-medium text-white">{{ status?.note || '正在检测 Codex 安装状态...' }}</div>
              <div class="mt-2 truncate font-mono text-xs text-zinc-500">{{ status?.installedPath || status?.stagedPath || installSourceLabel }}</div>
              <div v-if="status?.version" class="mt-1 text-xs text-zinc-500">版本：{{ status.version }}</div>
            </div>
          </div>
        </div>

        <div v-if="errorMsg" class="rounded-lg border border-red-500/20 bg-red-500/10 p-4 text-sm text-red-300">
          {{ errorMsg }}
        </div>

        <div class="flex flex-col gap-3 sm:flex-row">
          <button
            @click="handleInstall"
            class="inline-flex items-center justify-center rounded-lg bg-white px-5 py-3 text-sm font-bold text-black transition hover:bg-zinc-200 disabled:cursor-not-allowed disabled:opacity-50"
            :disabled="installing || loading || !status || status.platform === 'unsupported' || status.installed"
          >
            <Loader2 v-if="installing" class="mr-2 h-4 w-4 animate-spin" />
            <Download v-else class="mr-2 h-4 w-4" />
            {{ installing ? '正在下载并安装...' : status?.installed ? '已安装 Codex' : '一键下载并安装 Codex' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Download, Info, Loader2, MonitorDown, RefreshCw } from '@lucide/vue'
import { isTauriDesktop } from '@/utils/desktop-env'
import { installCodex, loadCodexInstallStatus, type CodexInstallStatus } from '@/services/codex-installer'
import { useMessage } from '@/utils/message'

const installSourceLabel = 'Codex 安装清单'
const message = useMessage()
const isDesktop = isTauriDesktop()
const loading = ref(false)
const installing = ref(false)
const errorMsg = ref('')
const status = ref<CodexInstallStatus | null>(null)

const refreshStatus = async () => {
  if (!isDesktop) {
    return
  }
  loading.value = true
  errorMsg.value = ''
  try {
    status.value = await loadCodexInstallStatus()
  } catch (err) {
    errorMsg.value = err instanceof Error ? err.message : String(err)
  } finally {
    loading.value = false
  }
}

const handleInstall = async () => {
  installing.value = true
  errorMsg.value = ''
  try {
    const nextStatus = await installCodex()
    if (nextStatus) {
      status.value = nextStatus
    }
    message.success('Codex 安装流程已执行完成')
  } catch (err) {
    errorMsg.value = err instanceof Error ? err.message : String(err)
  } finally {
    installing.value = false
  }
}

onMounted(() => {
  refreshStatus()
})
</script>
