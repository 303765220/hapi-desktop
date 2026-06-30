<template>
  <div class="space-y-6">
    <div class="flex flex-col gap-4 xl:flex-row xl:items-end xl:justify-between">
      <div>
        <h2 class="text-2xl font-semibold text-white">一键配置</h2>
        <p class="mt-1 text-sm text-zinc-400">把 Hapi API Key 写入本机客户端配置。Codex 安装继续使用 agentsmirror 源。</p>
      </div>

      <div class="flex w-full flex-col gap-3 sm:flex-row xl:w-auto">
        <select
          v-model="selectedKey"
          class="min-h-10 w-full rounded-lg border border-white/10 bg-black/50 px-3 py-2 text-sm text-white outline-none transition focus:border-purple-500/50 sm:min-w-[320px]"
          :disabled="keysLoading || activeKeys.length === 0"
        >
          <option value="" disabled>{{ keySelectLabel }}</option>
          <option v-for="key in activeKeys" :key="key.id" :value="key.key">
            {{ key.name || `API Key #${key.id}` }} · {{ formatKey(key.key) }}
          </option>
        </select>

        <button
          v-if="isDesktop"
          @click="refreshAll"
          class="inline-flex min-h-10 items-center justify-center rounded-lg border border-white/10 bg-white/5 px-4 py-2 text-sm font-medium text-white transition hover:bg-white/10 disabled:opacity-50"
          :disabled="statusLoading || keysLoading"
        >
          <Loader2 v-if="statusLoading || keysLoading" class="mr-2 h-4 w-4 animate-spin" />
          <RefreshCw v-else class="mr-2 h-4 w-4" />
          重新检测
        </button>
      </div>
    </div>

    <div v-if="!isDesktop" class="glass-panel p-6">
      <div class="flex items-start gap-4">
        <div class="rounded-lg bg-amber-500/10 p-3 text-amber-400">
          <MonitorDown class="h-6 w-6" />
        </div>
        <div>
          <h3 class="text-lg font-semibold text-white">需要桌面客户端</h3>
          <p class="mt-2 text-sm leading-6 text-zinc-400">本页面需要读取和写入本机配置文件。Web 端不会执行本机检测、安装或配置写入。</p>
        </div>
      </div>
    </div>

    <div v-else class="glass-panel overflow-hidden">
      <div v-if="pageError" class="border-b border-red-500/20 bg-red-500/10 px-6 py-4 text-sm text-red-300">
        {{ pageError }}
      </div>

      <div class="divide-y divide-white/5">
        <div v-if="statusLoading" class="space-y-4 p-6">
          <div v-for="i in 5" :key="i" class="h-[118px] animate-pulse rounded-lg border border-white/10 bg-white/5"></div>
        </div>

        <div
          v-for="item in statuses"
          v-else
          :key="item.client"
          class="grid gap-4 p-6 transition hover:bg-white/[0.03] lg:grid-cols-[minmax(220px,280px)_1fr_auto]"
        >
          <div class="flex min-w-0 items-start gap-3">
            <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg border border-white/10 bg-white/5 text-purple-300">
              <component :is="clientIcon(item.client)" class="h-5 w-5" />
            </div>
            <div class="min-w-0">
              <div class="flex items-center gap-2">
                <h3 class="font-semibold text-white">{{ item.name }}</h3>
                <span
                  class="inline-flex rounded-full border px-2 py-0.5 text-xs font-medium"
                  :class="item.installed ? 'border-green-500/20 bg-green-500/10 text-green-400' : 'border-amber-500/20 bg-amber-500/10 text-amber-400'"
                >
                  {{ item.installed ? '已安装' : '未检测到' }}
                </span>
              </div>
              <div class="mt-2 flex items-center gap-2 text-xs text-zinc-500">
                <TerminalSquare class="h-3.5 w-3.5 shrink-0" />
                <span class="font-mono">{{ item.command }}</span>
              </div>
            </div>
          </div>

          <div class="min-w-0">
            <div class="flex flex-wrap items-center gap-2">
              <span
                class="inline-flex rounded-full border px-2 py-0.5 text-xs font-medium"
                :class="item.configured ? 'border-purple-500/20 bg-purple-500/10 text-purple-300' : 'border-white/10 bg-white/5 text-zinc-400'"
              >
                {{ item.configured ? '已配置 Hapi' : '未配置 Hapi' }}
              </span>
              <span class="text-sm text-zinc-400">{{ item.note }}</span>
            </div>
            <div class="mt-3 truncate rounded-md border border-white/5 bg-black/30 px-3 py-2 font-mono text-xs text-zinc-500">
              {{ item.configPath }}
            </div>
          </div>

          <div class="flex flex-col gap-2 sm:flex-row lg:justify-end">
            <button
              v-if="item.client === 'codex'"
              @click="handleInstallCodex"
              class="inline-flex min-h-10 items-center justify-center rounded-lg border border-white/10 bg-white/5 px-4 py-2 text-sm font-medium text-white transition hover:bg-white/10 disabled:cursor-not-allowed disabled:opacity-50"
              :disabled="installingCodex || item.installed"
            >
              <Loader2 v-if="installingCodex" class="mr-2 h-4 w-4 animate-spin" />
              <Download v-else class="mr-2 h-4 w-4" />
              {{ item.installed ? '已安装' : '安装 Codex' }}
            </button>

            <button
              @click="handleConfigure(item)"
              class="inline-flex min-h-10 items-center justify-center rounded-lg bg-white px-4 py-2 text-sm font-bold text-black transition hover:bg-zinc-200 disabled:cursor-not-allowed disabled:opacity-50"
              :disabled="configuringClient === item.client || !item.installed || !selectedKey"
            >
              <Loader2 v-if="configuringClient === item.client" class="mr-2 h-4 w-4 animate-spin" />
              <Settings2 v-else class="mr-2 h-4 w-4" />
              {{ item.installed ? '配置' : '请先安装' }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import {
  Bot,
  Code2,
  Download,
  Feather,
  Loader2,
  MonitorDown,
  RefreshCw,
  Settings2,
  Sparkles,
  TerminalSquare,
  Wand2,
} from '@lucide/vue'
import keysAPI from '@/api/keys'
import type { ApiKey } from '@/types'
import { installCodex } from '@/services/codex-installer'
import {
  configureClient,
  loadClientSetupStatus,
  type ClientSetupClient,
  type ClientSetupStatus,
} from '@/services/client-setup'
import { isTauriDesktop } from '@/utils/desktop-env'
import { useMessage } from '@/utils/message'

const message = useMessage()
const isDesktop = isTauriDesktop()
const keysLoading = ref(false)
const statusLoading = ref(false)
const installingCodex = ref(false)
const configuringClient = ref<ClientSetupClient | ''>('')
const pageError = ref('')
const apiKeys = ref<ApiKey[]>([])
const statuses = ref<ClientSetupStatus[]>([])
const selectedKey = ref('')

const activeKeys = computed(() => apiKeys.value.filter((key) => key.status === 'active'))
const keySelectLabel = computed(() => {
  if (keysLoading.value) return '正在加载 API Key...'
  if (activeKeys.value.length === 0) return '暂无可用 active API Key'
  return '选择一个 Hapi API Key'
})

const formatKey = (key: string) => {
  if (!key) return ''
  if (key.length <= 12) return key
  return `${key.slice(0, 6)}...${key.slice(-4)}`
}

const clientIcon = (client: ClientSetupClient) => {
  const icons: Record<ClientSetupClient, unknown> = {
    geminiCli: Sparkles,
    codex: Bot,
    opencode: Code2,
    openclaw: Wand2,
    hermes: Feather,
  }
  return icons[client]
}

const loadKeys = async () => {
  keysLoading.value = true
  try {
    const response = await keysAPI.list(1, 100, { status: 'active' })
    apiKeys.value = response.items || []
  } catch (err) {
    pageError.value = err instanceof Error ? err.message : String(err)
  } finally {
    keysLoading.value = false
  }
}

const loadStatuses = async () => {
  if (!isDesktop) return
  statusLoading.value = true
  try {
    statuses.value = await loadClientSetupStatus()
  } catch (err) {
    pageError.value = err instanceof Error ? err.message : String(err)
  } finally {
    statusLoading.value = false
  }
}

const refreshAll = async () => {
  pageError.value = ''
  await Promise.all([loadKeys(), loadStatuses()])
}

const handleInstallCodex = async () => {
  installingCodex.value = true
  pageError.value = ''
  try {
    await installCodex()
    message.success('Codex 安装流程已执行完成')
    await loadStatuses()
  } catch (err) {
    pageError.value = err instanceof Error ? err.message : String(err)
  } finally {
    installingCodex.value = false
  }
}

const handleConfigure = async (item: ClientSetupStatus) => {
  if (!selectedKey.value) {
    message.warning('请先选择一个 active API Key')
    return
  }
  configuringClient.value = item.client
  pageError.value = ''
  try {
    const result = await configureClient(item.client, selectedKey.value)
    if (result?.backupPath) {
      message.success('配置已写入，原配置已备份')
    } else {
      message.success('配置已写入')
    }
    await loadStatuses()
  } catch (err) {
    pageError.value = err instanceof Error ? err.message : String(err)
  } finally {
    configuringClient.value = ''
  }
}

onMounted(() => {
  refreshAll()
})
</script>
