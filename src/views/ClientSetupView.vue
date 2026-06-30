<template>
  <div class="space-y-8 relative">
    <!-- Ambient Background Glow -->
    <div class="absolute top-0 right-0 w-[600px] h-[600px] bg-purple-600/10 rounded-full blur-[120px] pointer-events-none -translate-y-1/2 translate-x-1/3 z-0"></div>

    <!-- Header Section -->
    <div class="relative z-10 glass-panel p-8 flex flex-col xl:flex-row xl:items-center justify-between gap-6 overflow-hidden">
      <!-- Decorative background inside header -->
      <div class="absolute inset-0 bg-gradient-to-r from-purple-500/5 to-transparent pointer-events-none"></div>

      <div class="relative z-10">
        <div class="flex items-center gap-3 mb-2">
          <div class="p-2 bg-purple-500/10 rounded-lg text-purple-400">
            <Wand2 class="w-6 h-6" />
          </div>
          <h2 class="text-3xl font-bold text-white tracking-tight">一键配置</h2>
        </div>
        <p class="text-zinc-400 max-w-xl leading-relaxed">
          把 Hapi API Key 写入本机客户端配置，告别繁琐的手动复制粘贴。
          <span class="text-purple-400/80">Codex 支持自动安装和配置。</span>
        </p>
      </div>

      <div class="relative z-10 flex flex-col sm:flex-row gap-3 w-full xl:w-auto">
        <button
          v-if="isDesktop"
          @click="refreshAll"
          class="h-12 inline-flex items-center justify-center rounded-xl bg-white/5 border border-white/10 px-6 text-sm font-medium text-white transition-all hover:bg-white/10 hover:border-purple-500/30 disabled:opacity-50 group shadow-sm shadow-black/50"
          :disabled="statusLoading || keysLoading"
        >
          <Loader2 v-if="statusLoading || keysLoading" class="mr-2 h-4 w-4 animate-spin" />
          <RefreshCw v-else class="mr-2 h-4 w-4 group-hover:rotate-180 transition-transform duration-500" />
          重新检测
        </button>
      </div>
    </div>

    <!-- Desktop Required Warning -->
    <div v-if="!isDesktop" class="relative z-10 glass-panel p-8 border-amber-500/20 bg-gradient-to-br from-amber-500/5 to-transparent">
      <div class="flex flex-col items-center text-center max-w-lg mx-auto py-8">
        <div class="w-20 h-20 rounded-full bg-amber-500/10 flex items-center justify-center text-amber-400 mb-6 shadow-[0_0_30px_rgba(245,158,11,0.2)]">
          <MonitorDown class="w-10 h-10" />
        </div>
        <h3 class="text-2xl font-bold text-white mb-3">需要桌面客户端</h3>
        <p class="text-zinc-400 leading-relaxed">
          由于安全限制，Web 端无法直接读取和写入您的本机配置文件。
          请使用 <strong>Hapi 桌面端应用</strong> 来体验一键配置功能。
        </p>
      </div>
    </div>

    <!-- Main Content -->
    <div v-else class="relative z-10 space-y-6">
      <div v-if="pageError" class="rounded-xl border border-red-500/20 bg-red-500/10 px-6 py-4 flex items-center gap-3 text-red-300">
        <div class="p-1 bg-red-500/20 rounded-full"><X class="w-4 h-4" /></div>
        <span class="text-sm font-medium">{{ pageError }}</span>
      </div>

      <!-- Loading Skeletons -->
      <div v-if="statusLoading" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div v-for="i in 6" :key="i" class="h-[280px] rounded-2xl border border-white/5 bg-white/5 animate-pulse"></div>
      </div>

      <!-- Client Grid Cards -->
      <div v-else class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
        <div
          v-for="item in statuses"
          :key="item.client"
          class="glass-panel p-6 flex flex-col relative overflow-hidden group transition-all duration-300 hover:-translate-y-1 hover:shadow-2xl hover:shadow-purple-500/10 hover:border-purple-500/30"
        >
          <!-- Hover highlight -->
          <div class="absolute inset-0 bg-gradient-to-br from-white/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none"></div>

          <div class="relative z-10 flex items-start justify-between mb-6">
            <div class="flex items-center gap-4">
              <div class="w-12 h-12 rounded-2xl flex items-center justify-center border border-white/10 shadow-inner"
                   :class="clientTheme(item.client).bg + ' ' + clientTheme(item.client).text">
                <component :is="clientIcon(item.client)" class="w-6 h-6 transition-transform group-hover:scale-110 duration-300" />
              </div>
              <div>
                <h3 class="font-bold text-lg text-white group-hover:text-purple-300 transition-colors">{{ item.name }}</h3>
                <span
                  class="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-[11px] font-bold uppercase tracking-wider mt-1.5 border"
                  :class="item.installed ? 'border-emerald-500/20 bg-emerald-500/10 text-emerald-400' : 'border-amber-500/20 bg-amber-500/10 text-amber-400'"
                >
                  <span class="w-1.5 h-1.5 rounded-full" :class="item.installed ? 'bg-emerald-500' : 'bg-amber-500'"></span>
                  {{ item.installed ? '已安装' : '未检测到' }}
                </span>
              </div>
            </div>
          </div>

          <div class="relative z-10 flex-1 flex flex-col gap-3 mb-6">
            <div class="bg-black/40 rounded-xl p-3 border border-white/5 transition-colors group-hover:bg-black/60">
              <div class="text-[10px] font-bold text-zinc-500 uppercase tracking-wider mb-1.5 flex items-center gap-1.5">
                <TerminalSquare class="w-3 h-3" /> 安装命令
              </div>
              <div class="font-mono text-xs text-zinc-300 truncate">{{ item.command }}</div>
            </div>

            <div class="bg-black/40 rounded-xl p-3 border border-white/5 transition-colors group-hover:bg-black/60">
               <div class="flex items-center justify-between mb-1.5">
                 <div class="text-[10px] font-bold text-zinc-500 uppercase tracking-wider flex items-center gap-1.5">
                   <Settings2 class="w-3 h-3" /> Hapi 状态
                 </div>
                 <span class="text-xs font-bold" :class="item.configured ? 'text-purple-400 drop-shadow-[0_0_8px_rgba(168,85,247,0.5)]' : 'text-zinc-500'">
                   {{ item.configured ? '✓ 已配置' : '未配置' }}
                 </span>
               </div>
               <div class="text-[10px] text-zinc-400/80 truncate font-mono">
                 {{ item.configPath }}
               </div>
               <div v-if="item.note" class="text-[10px] text-zinc-500 mt-2 italic">
                 * {{ item.note }}
               </div>
            </div>

            <div class="bg-black/40 rounded-xl p-3 border border-white/5 transition-colors group-hover:bg-black/60">
              <label class="text-[10px] font-bold text-zinc-500 uppercase tracking-wider mb-2 flex items-center gap-1.5">
                <Key class="w-3 h-3" /> {{ keyPickerLabel(item.client) }}
              </label>
              <div class="relative">
                <select
                  :value="selectedKeys[item.client] || ''"
                  class="w-full h-10 appearance-none rounded-lg border border-white/10 bg-black/60 pl-3 pr-9 text-xs text-white outline-none transition-all focus:border-purple-500 focus:ring-1 focus:ring-purple-500 hover:border-white/20 disabled:opacity-50"
                  :disabled="keysLoading || availableKeysForClient(item.client).length === 0"
                  @change="setSelectedKey(item.client, ($event.target as HTMLSelectElement).value)"
                >
                  <option value="" disabled>{{ keySelectLabel(item.client) }}</option>
                  <option v-for="key in availableKeysForClient(item.client)" :key="key.id" :value="key.key">
                    {{ key.name || `API Key #${key.id}` }} · {{ formatKey(key.key) }}
                  </option>
                </select>
                <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
                  <ChevronDown class="h-3.5 w-3.5 text-zinc-500" />
                </div>
              </div>
            </div>
          </div>

          <div class="relative z-10 mt-auto flex items-center gap-3 pt-4 border-t border-white/5">
            <button
              v-if="item.client === 'codex'"
              @click="handleInstallCodex"
              class="flex-1 h-10 inline-flex items-center justify-center rounded-xl border border-white/10 bg-white/5 px-4 text-sm font-medium text-white transition-all hover:bg-white/10 hover:border-white/20 disabled:cursor-not-allowed disabled:opacity-50"
              :disabled="installingCodex || item.installed"
            >
              <Loader2 v-if="installingCodex" class="mr-2 h-4 w-4 animate-spin" />
              <Download v-else class="mr-2 h-4 w-4" />
              {{ item.installed ? '已安装' : '安装 Codex' }}
            </button>

            <button
              @click="handleConfigure(item)"
              class="flex-1 h-10 inline-flex items-center justify-center rounded-xl px-4 text-sm font-bold text-white transition-all disabled:cursor-not-allowed disabled:opacity-50 shadow-lg"
              :class="item.installed ? 'bg-purple-600 hover:bg-purple-500 hover:shadow-purple-500/25' : 'bg-zinc-800 text-zinc-400 hover:bg-zinc-700'"
              :disabled="configuringClient === item.client || !item.installed || !selectedKeys[item.client]"
            >
              <Loader2 v-if="configuringClient === item.client" class="mr-2 h-4 w-4 animate-spin" />
              <Settings2 v-else class="mr-2 h-4 w-4" />
              {{ item.installed ? '写入配置' : '请先安装' }}
            </button>

            <button
              v-if="item.configured"
              @click="handleClearConfig(item)"
              class="h-10 inline-flex items-center justify-center rounded-xl border border-red-500/20 bg-red-500/10 px-3 text-sm font-bold text-red-300 transition-all hover:bg-red-500/20 disabled:cursor-not-allowed disabled:opacity-50"
              :disabled="configuringClient === item.client"
              title="清除 Hapi 配置"
            >
              <Loader2 v-if="configuringClient === item.client" class="h-4 w-4 animate-spin" />
              <X v-else class="h-4 w-4" />
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
  Key,
  ChevronDown,
  X,
  Wand2,
} from '@lucide/vue'
import keysAPI from '@/api/keys'
import type { ApiKey } from '@/types'
import { installCodex } from '@/services/codex-installer'
import {
  clearClientConfig,
  configureClient,
  filterApiKeysForClient,
  loadClientSetupStatus,
  requiredPlatformForClient,
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
const selectedKeys = ref<Partial<Record<ClientSetupClient, string>>>({})

const activeKeys = computed(() => apiKeys.value.filter((key) => key.status === 'active'))

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

const clientTheme = (client: ClientSetupClient) => {
  const themes: Record<ClientSetupClient, { bg: string, text: string }> = {
    geminiCli: { bg: 'bg-blue-500/10', text: 'text-blue-400' },
    codex: { bg: 'bg-emerald-500/10', text: 'text-emerald-400' },
    opencode: { bg: 'bg-indigo-500/10', text: 'text-indigo-400' },
    openclaw: { bg: 'bg-amber-500/10', text: 'text-amber-400' },
    hermes: { bg: 'bg-orange-500/10', text: 'text-orange-400' },
  }
  return themes[client] || { bg: 'bg-purple-500/10', text: 'text-purple-400' }
}

const availableKeysForClient = (client: ClientSetupClient) => filterApiKeysForClient(apiKeys.value, client)

const keyPickerLabel = (client: ClientSetupClient) => {
  const platform = requiredPlatformForClient(client)
  if (platform === 'openai') return '选择 OpenAI 分组 Key'
  if (platform === 'gemini') return '选择 Gemini 分组 Key'
  return '选择任意 active Key'
}

const keySelectLabel = (client: ClientSetupClient) => {
  if (keysLoading.value) return '正在加载 API Key...'
  if (activeKeys.value.length === 0) return '暂无可用 active API Key'
  const count = availableKeysForClient(client).length
  if (count > 0) return '请选择，不自动默认'
  const platform = requiredPlatformForClient(client)
  if (platform === 'openai') return '暂无 OpenAI 分组 active Key'
  if (platform === 'gemini') return '暂无 Gemini 分组 active Key'
  return '暂无可用 active API Key'
}

const setSelectedKey = (client: ClientSetupClient, key: string) => {
  selectedKeys.value = {
    ...selectedKeys.value,
    [client]: key,
  }
}

const loadKeys = async () => {
  keysLoading.value = true
  try {
    const response = await keysAPI.list(1, 100, { status: 'active' })
    apiKeys.value = response.items || []
    const nextSelected = { ...selectedKeys.value }
    for (const client of Object.keys(nextSelected) as ClientSetupClient[]) {
      const available = availableKeysForClient(client)
      if (nextSelected[client] && !available.some((key) => key.key === nextSelected[client])) {
        delete nextSelected[client]
      }
    }
    selectedKeys.value = nextSelected
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
  const apiKey = selectedKeys.value[item.client]
  if (!apiKey) {
    message.warning('请先选择匹配的 active API Key')
    return
  }
  const selectedApiKey = availableKeysForClient(item.client).find((key) => key.key === apiKey)
  configuringClient.value = item.client
  pageError.value = ''
  try {
    const result = await configureClient(item.client, apiKey, selectedApiKey?.group?.platform || null)
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

const handleClearConfig = async (item: ClientSetupStatus) => {
  configuringClient.value = item.client
  pageError.value = ''
  try {
    await clearClientConfig(item.client)
    message.success('Hapi 配置已清除')
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
