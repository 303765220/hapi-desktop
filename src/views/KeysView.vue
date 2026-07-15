<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
      <div class="inline-flex w-full max-w-full flex-col gap-2 rounded-xl border border-white/10 bg-black/30 px-4 py-3 sm:w-auto sm:min-w-[360px]">
        <span class="text-xs font-medium text-zinc-500">API 端点地址</span>
        <div class="flex items-center gap-2">
          <span class="truncate font-mono text-sm font-medium text-zinc-100">{{ API_ENDPOINT_URL }}</span>
          <button
            @click="copyToClipboard(API_ENDPOINT_URL)"
            class="shrink-0 text-zinc-500 transition-colors hover:text-white"
            title="复制 API 端点地址"
          >
            <Copy class="w-4 h-4" />
          </button>
        </div>
      </div>

      <button @click="handleCreate" class="px-4 py-2 bg-white text-black font-semibold rounded-lg hover:bg-zinc-200 transition-colors flex items-center disabled:opacity-50" :disabled="creating">
        <Loader2 v-if="creating" class="w-4 h-4 mr-2 animate-spin" />
        <Plus v-else class="w-4 h-4 mr-2" />
        新建密钥
      </button>
    </div>

    <!-- Main Content -->
    <div class="glass-panel overflow-hidden">
      <div v-if="loading" class="w-full">
        <table class="w-full text-left border-collapse min-w-[1380px]">
          <thead>
            <tr class="border-b border-white/10 bg-black/20">
              <th
                v-for="i in 8"
                :key="i"
                class="px-6 py-4"
                :class="i === 8 ? 'sticky right-0 z-20 bg-[#111114] border-l border-white/10 shadow-[-12px_0_24px_rgba(0,0,0,0.28)]' : ''"
              >
                <div class="h-4 bg-white/10 rounded w-16"></div>
              </th>
            </tr>
          </thead>
          <tbody class="divide-y divide-white/5">
            <tr v-for="i in 5" :key="i" class="animate-pulse">
              <td
                v-for="j in 8"
                :key="j"
                class="px-6 py-4"
                :class="j === 8 ? 'sticky right-0 z-10 bg-[#0f0f12] border-l border-white/10 shadow-[-12px_0_24px_rgba(0,0,0,0.22)]' : ''"
              >
                <div class="h-4 bg-white/5 rounded" :class="j === 3 ? 'w-48' : 'w-24'"></div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      
      <div v-else-if="keys.length === 0" class="p-12 text-center">
        <div class="w-16 h-16 mx-auto bg-white/5 rounded-full flex items-center justify-center mb-4">
          <Key class="w-8 h-8 text-zinc-500" />
        </div>
        <h3 class="text-lg font-medium text-white mb-2">暂无 API 密钥</h3>
        <p class="text-zinc-400 mb-6 max-w-md mx-auto">您尚未创建任何 API 密钥。创建一个来开始您的 API 调用之旅吧。</p>
      </div>

      <div v-else class="w-full overflow-x-auto relative">
        <table class="w-full text-left border-collapse min-w-[1380px]">
          <thead>
            <tr class="border-b border-white/10 bg-black/20">
              <th class="px-6 py-4 text-sm font-medium text-zinc-400">名称</th>
              <th class="px-6 py-4 text-sm font-medium text-zinc-400">分组</th>
              <th class="px-6 py-4 text-sm font-medium text-zinc-400">密钥</th>
              <th class="px-6 py-4 text-sm font-medium text-zinc-400">用量</th>
              <th class="px-6 py-4 text-sm font-medium text-zinc-400 whitespace-nowrap">创建时间</th>
              <th class="px-6 py-4 text-sm font-medium text-zinc-400 whitespace-nowrap">上次使用</th>
              <th class="px-6 py-4 text-sm font-medium text-zinc-400">状态</th>
              <th class="sticky right-0 z-20 px-6 py-4 text-sm font-medium text-zinc-400 text-right bg-[#111114] border-l border-white/10 shadow-[-12px_0_24px_rgba(0,0,0,0.28)]">操作</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-white/5">
            <tr v-for="item in keys" :key="item.id" class="hover:bg-white/5 transition-colors group">
              <td class="px-6 py-4 min-w-[160px]">
                <div class="font-medium text-white">{{ item.name || 'Untitled Key' }}</div>
              </td>
              <td class="px-6 py-4 min-w-[180px]">
                <div>
                  <div 
                    class="flex items-center space-x-1 cursor-pointer hover:bg-white/5 px-2 py-1 -ml-2 rounded transition-colors text-sm font-medium w-max" 
                    :class="item.group_id ? 'text-purple-400' : 'text-zinc-500'"
                    @click.stop="(e) => toggleDropdown(e, item.id)"
                  >
                    <span class="w-1.5 h-1.5 rounded-full mr-1.5" :class="getGroupColorClass(item.group_id)"></span>
                    <span>{{ getGroupName(item.group_id) }}</span>
                    <span v-if="availableGroups.find(g => g.id === item.group_id)?.is_exclusive" class="px-1.5 py-0.5 rounded text-[10px] font-bold bg-amber-500/10 text-amber-500 border border-amber-500/20 ml-2">独占</span>
                    <ChevronDown class="w-3 h-3 transition-transform" :class="openDropdownId === item.id ? 'rotate-180' : ''" />
                  </div>
                </div>
              </td>
              <td class="px-6 py-4 min-w-[220px]">
                <div class="flex items-center space-x-2 text-zinc-300 font-mono text-sm bg-black/40 px-3 py-1.5 rounded-md border border-white/5 w-max max-w-[240px]">
                  <span>{{ formatKey(item.key) }}</span>
                  <button @click="copyToClipboard(item.key)" class="text-zinc-500 hover:text-white transition-colors">
                    <Copy class="w-4 h-4" />
                  </button>
                </div>
              </td>
              <td class="px-6 py-4 min-w-[210px]">
                <div class="space-y-1.5 text-sm">
                  <div class="flex items-center gap-1.5 whitespace-nowrap">
                    <span class="text-zinc-500">今日:</span>
                    <span class="font-mono font-medium text-zinc-100">
                      ${{ formatUsageCost(usageStats[item.id]?.today_actual_cost) }}
                    </span>
                  </div>
                  <div class="flex items-center gap-1.5 whitespace-nowrap">
                    <span class="text-zinc-500">近30天:</span>
                    <span class="font-mono font-medium text-zinc-100">
                      ${{ formatUsageCost(usageStats[item.id]?.total_actual_cost) }}
                    </span>
                  </div>
                </div>
              </td>
              <td class="px-6 py-4 text-sm text-zinc-400 whitespace-nowrap">
                {{ formatBeijingTime(item.created_at) }}
              </td>
              <td class="px-6 py-4 text-sm text-zinc-400 whitespace-nowrap">
                {{ formatBeijingTime(item.last_used_at) }}
              </td>
              <td class="px-6 py-4">
                <span
                  class="inline-flex text-xs font-medium px-2 py-1 rounded-full border whitespace-nowrap"
                  :class="item.status === 'active' ? 'bg-green-500/10 text-green-400 border-green-500/20' : 'bg-red-500/10 text-red-400 border-red-500/20'"
                >
                  {{ getApiKeyStatusLabel(item.status) }}
                </span>
              </td>
              <td class="sticky right-0 z-10 px-6 py-4 text-right min-w-[190px] bg-[#0f0f12] border-l border-white/10 shadow-[-12px_0_24px_rgba(0,0,0,0.22)] group-hover:bg-[#17171a]">
                <div class="flex items-center justify-end space-x-3">
                  <button
                    @click="handleImportToCcSwitch(item)"
                    class="text-zinc-400 hover:text-blue-400 transition-colors"
                    title="导入 CC Switch"
                  >
                    <Upload class="w-4 h-4" />
                  </button>
                  <button
                    @click="handleToggleStatus(item)"
                    class="text-zinc-400 transition-colors disabled:opacity-50"
                    :class="item.status === 'active' ? 'hover:text-amber-400' : 'hover:text-green-400'"
                    :disabled="item.toggling"
                    :title="getToggleApiKeyStatusAction(item.status).title"
                  >
                    <Loader2 v-if="item.toggling" class="w-4 h-4 animate-spin" />
                    <Power v-else class="w-4 h-4" />
                  </button>
                  <button @click="handleEdit(item)" class="text-zinc-400 hover:text-purple-400 transition-colors" title="编辑">
                    <Edit class="w-4 h-4" />
                  </button>
                  <button @click="handleDelete(item.id)" class="text-zinc-400 hover:text-red-400 transition-colors disabled:opacity-50" :disabled="item.deleting" title="删除">
                    <Loader2 v-if="item.deleting" class="w-4 h-4 animate-spin" />
                    <Trash2 v-else class="w-4 h-4" />
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>

        <!-- Pagination -->
        <div v-if="totalPages > 1" class="px-6 py-4 border-t border-white/5 flex items-center justify-between bg-black/20">
          <div class="text-sm text-zinc-400">
            共 <span class="text-white font-medium">{{ totalKeys }}</span> 个密钥
          </div>
          <div class="flex items-center space-x-2">
            <button 
              @click="page > 1 ? (page--, fetchKeys()) : null"
              :disabled="page === 1"
              class="px-3 py-1.5 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg text-sm text-white disabled:opacity-50 transition-colors"
            >
              上一页
            </button>
            <div class="px-3 py-1.5 text-sm text-zinc-400 font-medium">
              {{ page }} / {{ totalPages }}
            </div>
            <button 
              @click="page < totalPages ? (page++, fetchKeys()) : null"
              :disabled="page === totalPages"
              class="px-3 py-1.5 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg text-sm text-white disabled:opacity-50 transition-colors"
            >
              下一页
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Global Teleport Dropdown for Group Switching -->
    <Teleport to="body">
      <div 
        v-if="openDropdownId" 
        class="fixed z-[9999] bg-[#18181b] border border-white/10 rounded-lg shadow-xl w-max min-w-[160px] overflow-hidden shadow-purple-500/10"
        :style="dropdownPos"
        @click.stop
      >
        <div class="py-1 max-h-64 overflow-y-auto custom-scrollbar">
          <div 
            class="px-3 py-2.5 hover:bg-white/5 border-b border-white/5 last:border-0 cursor-pointer transition-colors flex flex-col gap-1.5"
            @click="() => { const item = keys.find(k => k.id === openDropdownId); if(item) handleUpdateGroup(item, null); openDropdownId = null }"
          >
            <div class="flex items-center gap-2">
              <span class="w-1.5 h-1.5 rounded-full bg-zinc-500"></span>
              <span class="text-xs text-zinc-400">无分组 (默认)</span>
            </div>
          </div>
          <div 
            v-for="g in availableGroups" :key="g.id"
            class="px-3 py-2.5 hover:bg-white/5 border-b border-white/5 last:border-0 cursor-pointer transition-colors flex flex-col gap-1.5"
            @click="() => { const item = keys.find(k => k.id === openDropdownId); if(item) handleUpdateGroup(item, g.id); openDropdownId = null }"
          >
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <div class="flex items-center gap-1.5 px-2 py-0.5 rounded-full" :class="getGroupTheme(g.name).bg + ' ' + getGroupTheme(g.name).text">
                  <component :is="getGroupTheme(g.name).icon" class="w-3.5 h-3.5 shrink-0" />
                  <span class="text-xs font-bold">{{ g.name }}</span>
                </div>
              </div>
              <div class="flex items-center gap-2">
                <div class="px-2 py-0.5 rounded-full text-[10px] font-bold" :class="getGroupTheme(g.name).bg + ' ' + getGroupTheme(g.name).text">
                  {{ g.rate_multiplier }}x 倍率
                </div>
                <Check v-if="keys.find(k => k.id === openDropdownId)?.group_id === g.id" class="w-4 h-4 text-emerald-500 shrink-0" />
              </div>
            </div>
            <div v-if="g.description" class="text-xs text-zinc-400 pl-1 mt-0.5">
              {{ g.description }}
            </div>
          </div>
        </div>
      </div>
    </Teleport>

    <Transition name="fade">
      <div v-if="showCcsClientSelect" class="fixed inset-0 z-50 flex items-center justify-center p-4">
        <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="closeCcsClientSelect"></div>
        <div class="glass-panel p-6 max-w-md w-full relative z-10 shadow-2xl border-white/10">
          <h3 class="text-lg font-bold text-white mb-2">选择导入目标</h3>
          <p class="text-sm text-zinc-400 mb-5">Antigravity 分组可导入到 Claude Code 或 Gemini CLI，请选择当前要使用的客户端。</p>
          <div class="grid grid-cols-2 gap-3">
            <button
              @click="handleCcsClientSelect('claude')"
              class="rounded-xl border border-white/10 bg-white/5 p-4 text-left transition hover:border-purple-400/60 hover:bg-purple-500/10"
            >
              <Bot class="w-5 h-5 text-purple-400 mb-3" />
              <div class="font-medium text-white">Claude Code</div>
              <div class="text-xs text-zinc-500 mt-1">Claude 兼容入口</div>
            </button>
            <button
              @click="handleCcsClientSelect('gemini')"
              class="rounded-xl border border-white/10 bg-white/5 p-4 text-left transition hover:border-blue-400/60 hover:bg-blue-500/10"
            >
              <Zap class="w-5 h-5 text-blue-400 mb-3" />
              <div class="font-medium text-white">Gemini CLI</div>
              <div class="text-xs text-zinc-500 mt-1">Gemini 兼容入口</div>
            </button>
          </div>
          <div class="mt-6 flex justify-end">
            <button
              @click="closeCcsClientSelect"
              class="px-4 py-2 bg-white/5 hover:bg-white/10 text-white text-sm font-medium rounded-lg transition border border-white/10"
            >
              取消
            </button>
          </div>
        </div>
      </div>
    </Transition>

    <!-- Create Key Modal -->
    <Transition name="fade">
      <div v-if="showCreateModal" class="fixed inset-0 z-50 flex items-center justify-center p-4">
        <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="showCreateModal = false"></div>
        <div class="glass-panel p-6 max-w-2xl w-full relative z-10 shadow-2xl border-white/10 max-h-[90vh] overflow-y-auto custom-scrollbar">
          <h3 class="text-lg font-semibold text-white mb-6">{{ isEditing ? '编辑 API 密钥' : '新建 API 密钥' }}</h3>
          
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Basic info -->
            <div class="space-y-4">
              <h4 class="text-sm font-medium text-purple-400 border-b border-white/5 pb-2">基本设置</h4>
              
              <div>
                <label class="block text-sm font-medium text-zinc-300 mb-1">名称 <span class="text-red-500">*</span></label>
                <input 
                  v-model="newKeyForm.name"
                  type="text" 
                  placeholder="例如：开发环境密钥"
                  class="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-white focus:outline-none focus:ring-2 focus:ring-purple-500/50"
                />
              </div>

              <div class="relative">
                <label class="block text-sm font-medium text-zinc-300 mb-1">路由分组</label>
                <div 
                  class="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-white hover:border-white/20 transition-colors cursor-pointer flex justify-between items-center"
                  @click="showGroupDropdown = !showGroupDropdown"
                >
                  <span :class="newKeyForm.group_id ? 'text-white' : 'text-zinc-500'">
                    {{ newKeyForm.group_id ? availableGroups.find(g => g.id === newKeyForm.group_id)?.name : '-- 不绑定分组 --' }}
                  </span>
                  <ChevronDown class="w-4 h-4 text-zinc-500 transition-transform" :class="showGroupDropdown ? 'rotate-180' : ''" />
                </div>
                
                <div v-if="showGroupDropdown" class="fixed inset-0 z-40" @click="showGroupDropdown = false"></div>
                <div 
                  v-if="showGroupDropdown" 
                  class="absolute left-0 right-0 top-full mt-1 z-50 bg-[#18181b] border border-white/10 rounded-lg shadow-xl overflow-hidden py-1 max-h-64 overflow-y-auto custom-scrollbar shadow-purple-500/10"
                >
                  <div 
                    class="px-3 py-2.5 hover:bg-white/5 border-b border-white/5 last:border-0 cursor-pointer transition-colors flex flex-col gap-1.5"
                    @click="newKeyForm.group_id = null; showGroupDropdown = false"
                  >
                    <div class="flex items-center gap-2">
                      <span class="w-1.5 h-1.5 rounded-full bg-zinc-500"></span>
                      <span class="text-xs text-zinc-400">-- 不绑定分组 --</span>
                    </div>
                  </div>
                  <div 
                    v-for="g in availableGroups" :key="g.id"
                    class="px-3 py-2.5 hover:bg-white/5 border-b border-white/5 last:border-0 cursor-pointer transition-colors flex flex-col gap-1.5"
                    @click="newKeyForm.group_id = g.id; showGroupDropdown = false"
                  >
                    <div class="flex items-center justify-between">
                      <div class="flex items-center gap-2">
                        <div class="flex items-center gap-1.5 px-2 py-0.5 rounded-full" :class="getGroupTheme(g.name).bg + ' ' + getGroupTheme(g.name).text">
                          <component :is="getGroupTheme(g.name).icon" class="w-3.5 h-3.5 shrink-0" />
                          <span class="text-xs font-bold">{{ g.name }}</span>
                        </div>
                      </div>
                      <div class="flex items-center gap-2">
                        <div class="px-2 py-0.5 rounded-full text-[10px] font-bold" :class="getGroupTheme(g.name).bg + ' ' + getGroupTheme(g.name).text">
                          {{ g.rate_multiplier }}x 倍率
                        </div>
                        <Check v-if="newKeyForm.group_id === g.id" class="w-4 h-4 text-emerald-500 shrink-0" />
                      </div>
                    </div>
                    <div v-if="g.description" class="text-xs text-zinc-400 pl-1 mt-0.5">
                      {{ g.description }}
                    </div>
                  </div>
                </div>
              </div>

              <div v-if="!isEditing">
                <label class="block text-sm font-medium text-zinc-300 mb-1">自定义 Key (可选)</label>
                <input 
                  v-model="newKeyForm.custom_key"
                  type="text" 
                  placeholder="自选字符串作为 Key"
                  class="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-white focus:outline-none focus:ring-2 focus:ring-purple-500/50"
                />
              </div>
              
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label class="block text-sm font-medium text-zinc-300 mb-1">额度限制 ($)</label>
                  <input 
                    v-model.number="newKeyForm.quota"
                    type="number" 
                    placeholder="无限制"
                    class="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-white focus:outline-none focus:ring-2 focus:ring-purple-500/50"
                  />
                </div>
                <div v-if="!isEditing">
                  <label class="block text-sm font-medium text-zinc-300 mb-1">过期时间 (天)</label>
                  <input 
                    v-model.number="newKeyForm.expiresInDays"
                    type="number" 
                    placeholder="永不过期"
                    class="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-white focus:outline-none focus:ring-2 focus:ring-purple-500/50"
                  />
                </div>
              </div>
            </div>

            <!-- Advanced / Security -->
            <div class="space-y-4">
              <h4 class="text-sm font-medium text-purple-400 border-b border-white/5 pb-2">风控与速率 (留空不限制)</h4>
              
              <div class="grid grid-cols-3 gap-2">
                <div>
                  <label class="block text-xs font-medium text-zinc-400 mb-1">5小时速率</label>
                  <input v-model.number="newKeyForm.rate_limit_5h" type="number" class="w-full bg-black/50 border border-white/10 rounded px-2 py-1.5 text-sm text-white" />
                </div>
                <div>
                  <label class="block text-xs font-medium text-zinc-400 mb-1">单日速率</label>
                  <input v-model.number="newKeyForm.rate_limit_1d" type="number" class="w-full bg-black/50 border border-white/10 rounded px-2 py-1.5 text-sm text-white" />
                </div>
                <div>
                  <label class="block text-xs font-medium text-zinc-400 mb-1">七日速率</label>
                  <input v-model.number="newKeyForm.rate_limit_7d" type="number" class="w-full bg-black/50 border border-white/10 rounded px-2 py-1.5 text-sm text-white" />
                </div>
              </div>

              <div>
                <label class="block text-sm font-medium text-zinc-300 mb-1">IP 白名单 (换行分隔)</label>
                <textarea 
                  v-model="newKeyForm.ip_whitelist"
                  rows="2"
                  class="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:ring-2 focus:ring-purple-500/50 custom-scrollbar"
                ></textarea>
              </div>

              <div>
                <label class="block text-sm font-medium text-zinc-300 mb-1">IP 黑名单 (换行分隔)</label>
                <textarea 
                  v-model="newKeyForm.ip_blacklist"
                  rows="2"
                  class="w-full bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:ring-2 focus:ring-purple-500/50 custom-scrollbar"
                ></textarea>
              </div>
            </div>
          </div>
          
          <div class="flex space-x-3 justify-end mt-8 pt-4 border-t border-white/5">
            <button 
              @click="showCreateModal = false"
              class="px-4 py-2 bg-white/5 hover:bg-white/10 text-white text-sm font-medium rounded-lg transition border border-white/10"
            >
              取消
            </button>
            <button 
              @click="submitCreateKey"
              :disabled="creating || !newKeyForm.name"
              class="px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white text-sm font-medium rounded-lg transition shadow-lg shadow-purple-500/20 disabled:opacity-50 flex items-center"
            >
              <Loader2 v-if="creating" class="w-4 h-4 mr-2 animate-spin" />
              确定{{ isEditing ? '保存' : '创建' }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { Plus, Key, Copy, Trash2, Loader2, ChevronDown, Edit, Bot, Sun, Zap, Check, Power, Upload } from '@lucide/vue'
import { list as getKeysList, create as createKey, deleteKey, toggleStatus, update as updateKey } from '@/api/keys'
import { getDashboardApiKeysUsage, type BatchApiKeyUsageStats } from '@/api/usage'
import { userGroupsAPI } from '@/api/groups'
import { useMessage, showConfirm } from '@/utils/message'
import type { ApiKey, Group, GroupPlatform } from '@/types'
import { getApiKeyStatusLabel, getToggleApiKeyStatusAction } from './api-key-status'
import { buildCcSwitchImportDeeplink, type CcSwitchClientType } from '@/utils/ccswitchImport'

const message = useMessage()
const API_ENDPOINT_URL = 'https://www.hapi666.com/'
// CC Switch 深链使用站点根地址作为 provider homepage/endpoint。这里去掉尾斜杠，是为了避免 antigravity endpoint 被拼成 `//antigravity`；改回带尾斜杠会让导入配置出现重复斜杠。用 keys-view.spec 和 ccswitchImport.spec 共同验证按钮接线和平台映射。
const CC_SWITCH_BASE_URL = API_ENDPOINT_URL.replace(/\/+$/, '')
// CC Switch 的用量脚本沿用原 frontend 逻辑，通过 Bearer 当前 key 请求 `/v1/usage` 并抽取 remaining/unit；改成其他接口会导致 CC Switch 用量展示和站内 key 余额口径不一致。用线上 `/api` 代理不参与此脚本，发布后由 CC Switch 实际导入验证。
const CC_SWITCH_USAGE_SCRIPT = `({
  request: {
    url: "{{baseUrl}}/v1/usage",
    method: "GET",
    headers: { "Authorization": "Bearer {{apiKey}}" }
  },
  extractor: function(response) {
    const remaining = response?.remaining ?? response?.quota?.remaining ?? response?.balance;
    const unit = response?.unit ?? response?.quota?.unit ?? "USD";
    return {
      isValid: response?.is_active ?? response?.isValid ?? true,
      remaining,
      unit
    };
  }
})`
const keys = ref<any[]>([])
const usageStats = ref<Record<string, BatchApiKeyUsageStats>>({})
const availableGroups = ref<Group[]>([])
const loading = ref(true)
const creating = ref(false)
const showCcsClientSelect = ref(false)
const pendingCcsKey = ref<ApiKey | null>(null)

const page = ref(1)
const pageSize = ref(20)
const totalKeys = ref(0)
const totalPages = computed(() => Math.max(1, Math.ceil(totalKeys.value / pageSize.value)))

// Dropdown state for group switching
const openDropdownId = ref<number | null>(null)
const dropdownPos = ref({ top: '0px', left: '0px' })

let closeDropdownHandler: ((e: MouseEvent) => void) | null = null

const toggleDropdown = (e: MouseEvent, id: number) => {
  if (openDropdownId.value === id) {
    openDropdownId.value = null
    if (closeDropdownHandler) window.removeEventListener('click', closeDropdownHandler)
    return
  }
  
  const rect = (e.currentTarget as HTMLElement).getBoundingClientRect()
  dropdownPos.value = {
    top: `${rect.bottom + 4}px`,
    left: `${rect.left}px`
  }
  openDropdownId.value = id
  
  // Clean up old listener if exists
  if (closeDropdownHandler) window.removeEventListener('click', closeDropdownHandler)
  
  // Set up new listener to close dropdown when clicking outside
  closeDropdownHandler = () => {
    openDropdownId.value = null
    if (closeDropdownHandler) window.removeEventListener('click', closeDropdownHandler)
  }
  setTimeout(() => {
    if (closeDropdownHandler) window.addEventListener('click', closeDropdownHandler)
  }, 0)
}

const fetchGroups = async () => {
  try {
    const res = await userGroupsAPI.getAvailable()
    availableGroups.value = res
  } catch (err) {
    console.error("Failed to load groups", err)
  }
}

const fetchKeys = async () => {
  loading.value = true
  try {
    const res = await getKeysList(page.value, pageSize.value)
    keys.value = res.items || []
    totalKeys.value = res.total || 0
    await fetchKeysUsage(keys.value)
  } catch (err) {
    console.error("Failed to load keys", err)
  } finally {
    loading.value = false
  }
}

const fetchKeysUsage = async (items: ApiKey[]) => {
  usageStats.value = {}
  const keyIds = items.map(item => item.id)
  if (keyIds.length === 0) return
  try {
    const res = await getDashboardApiKeysUsage(keyIds)
    usageStats.value = res.stats || {}
  } catch (err) {
    console.error("Failed to load key usage", err)
  }
}

onMounted(() => {
  fetchKeys()
  fetchGroups()
})

const formatKey = (key: string) => {
  if (!key) return ''
  if (key.length <= 12) return key
  return `${key.substring(0, 8)}...${key.substring(key.length - 4)}`
}

const formatBeijingTime = (dateStr: string | null | undefined) => {
  if (!dateStr) return '-'
  const d = new Date(dateStr)
  return new Intl.DateTimeFormat('zh-CN', {
    timeZone: 'Asia/Shanghai',
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  }).format(d).replace(/\//g, '-')
}

const copyToClipboard = (text: string) => {
  navigator.clipboard.writeText(text)
  message.success('已复制到剪贴板！')
}

const showCreateModal = ref(false)
const showGroupDropdown = ref(false)
const newKeyForm = ref({
  name: '',
  group_id: null as number | null,
  custom_key: '',
  quota: null as number | null,
  expiresInDays: null as number | null,
  rate_limit_5h: null as number | null,
  rate_limit_1d: null as number | null,
  rate_limit_7d: null as number | null,
  ip_whitelist: '',
  ip_blacklist: ''
})

const isEditing = ref(false)
const editingKeyId = ref<number | null>(null)

const handleCreate = () => {
  isEditing.value = false
  editingKeyId.value = null
  newKeyForm.value = {
    name: '', group_id: null, custom_key: '', quota: null, expiresInDays: null,
    rate_limit_5h: null, rate_limit_1d: null, rate_limit_7d: null,
    ip_whitelist: '', ip_blacklist: ''
  }
  showCreateModal.value = true
}

const handleEdit = (item: any) => {
  isEditing.value = true
  editingKeyId.value = item.id
  newKeyForm.value = {
    name: item.name || '',
    group_id: item.group_id || null,
    custom_key: '',
    quota: item.quota || null,
    expiresInDays: null,
    rate_limit_5h: item.rate_limit_5h || null,
    rate_limit_1d: item.rate_limit_1d || null,
    rate_limit_7d: item.rate_limit_7d || null,
    ip_whitelist: item.ip_whitelist ? item.ip_whitelist.join('\n') : '',
    ip_blacklist: item.ip_blacklist ? item.ip_blacklist.join('\n') : ''
  }
  showCreateModal.value = true
}

const submitCreateKey = async () => {
  if (!newKeyForm.value.name) return
  creating.value = true
  
  const whitelist = newKeyForm.value.ip_whitelist.split('\n').map(i => i.trim()).filter(Boolean)
  const blacklist = newKeyForm.value.ip_blacklist.split('\n').map(i => i.trim()).filter(Boolean)
  
  try {
    if (isEditing.value && editingKeyId.value) {
      await updateKey(editingKeyId.value, {
        name: newKeyForm.value.name,
        group_id: newKeyForm.value.group_id,
        ip_whitelist: whitelist.length > 0 ? whitelist : [],
        ip_blacklist: blacklist.length > 0 ? blacklist : [],
        quota: newKeyForm.value.quota === null ? 0 : newKeyForm.value.quota,
        rate_limit_5h: newKeyForm.value.rate_limit_5h || undefined,
        rate_limit_1d: newKeyForm.value.rate_limit_1d || undefined,
        rate_limit_7d: newKeyForm.value.rate_limit_7d || undefined,
      })
      message.success("密钥更新成功！")
    } else {
      await createKey(
        newKeyForm.value.name,
        newKeyForm.value.group_id,
        newKeyForm.value.custom_key,
        whitelist.length > 0 ? whitelist : undefined,
        blacklist.length > 0 ? blacklist : undefined,
        newKeyForm.value.quota || undefined,
        newKeyForm.value.expiresInDays || undefined,
        {
          rate_limit_5h: newKeyForm.value.rate_limit_5h || undefined,
          rate_limit_1d: newKeyForm.value.rate_limit_1d || undefined,
          rate_limit_7d: newKeyForm.value.rate_limit_7d || undefined,
        }
      )
      message.success("密钥创建成功！")
    }
    showCreateModal.value = false
    await fetchKeys()
  } catch (err) {
    console.error("Save key failed", err)
    message.error(isEditing.value ? "更新失败，请检查填写的内容" : "创建失败，请检查填写的内容")
  } finally {
    creating.value = false
  }
}

const handleDelete = async (id: number) => {
  showConfirm({
    title: '确认删除',
    content: '确定要删除这个 API 密钥吗？',
    onConfirm: async () => {
      const target = keys.value.find(k => k.id === id)
      if (target) target.deleting = true
      try {
        await deleteKey(id)
        keys.value = keys.value.filter(k => k.id !== id)
      } catch (err) {
        console.error("Delete failed", err)
        message.error("删除失败")
        if (target) target.deleting = false
      }
    }
  })
}

const handleToggleStatus = async (item: any) => {
  item.toggling = true
  const newStatus = getToggleApiKeyStatusAction(item.status).nextStatus
  try {
    await toggleStatus(item.id, newStatus)
    item.status = newStatus
    message.success(newStatus === 'active' ? '密钥已启用' : '密钥已禁用')
  } catch (err) {
    console.error("Toggle status failed", err)
    message.error("状态切换失败")
  } finally {
    item.toggling = false
  }
}

const getGroupName = (groupId: number | null) => {
  if (!groupId) return '无分组'
  const group = availableGroups.value.find(g => g.id === groupId)
  return group ? group.name : '未知分组'
}

const getGroupColorClass = (groupId: number | null) => {
  if (!groupId) return 'bg-zinc-500 shadow-none'
  const group = availableGroups.value.find(g => g.id === groupId)
  return group?.is_exclusive ? 'bg-purple-500 shadow-[0_0_8px_rgba(168,85,247,0.5)]' : 'bg-green-500 shadow-[0_0_8px_rgba(34,197,94,0.5)]'
}

const getGroupTheme = (name: string) => {
  const lower = name.toLowerCase()
  if (lower.includes('cc-')) {
    return { icon: Sun, text: 'text-amber-500', bg: 'bg-amber-500/10' }
  }
  if (lower.includes('gpt')) {
    return { icon: Bot, text: 'text-emerald-500', bg: 'bg-emerald-500/10' }
  }
  return { icon: Zap, text: 'text-blue-500', bg: 'bg-blue-500/10' }
}

const formatUsageCost = (value: number | null | undefined) => {
  return (value || 0).toFixed(4)
}

const handleUpdateGroup = async (item: any, groupId: number | null) => {
  if (item.group_id === groupId) return
  
  const oldGroupId = item.group_id
  item.group_id = groupId
  
  try {
    await updateKey(item.id, { group_id: groupId })
    message.success('已切换分组')
  } catch (err) {
    console.error('Failed to update group', err)
    item.group_id = oldGroupId
    message.error('切换分组失败')
  }
}

const getKeyPlatform = (item: ApiKey): GroupPlatform => {
  return item.group?.platform || availableGroups.value.find(g => g.id === item.group_id)?.platform || 'anthropic'
}

const handleImportToCcSwitch = (item: ApiKey) => {
  const platform = getKeyPlatform(item)
  if (platform === 'antigravity') {
    pendingCcsKey.value = item
    showCcsClientSelect.value = true
    return
  }

  executeCcSwitchImport(item, platform === 'gemini' ? 'gemini' : 'claude')
}

const executeCcSwitchImport = (item: ApiKey, clientType: CcSwitchClientType) => {
  const deeplink = buildCcSwitchImportDeeplink({
    baseUrl: CC_SWITCH_BASE_URL,
    platform: getKeyPlatform(item),
    clientType,
    providerName: 'Hapi',
    apiKey: item.key,
    usageScript: CC_SWITCH_USAGE_SCRIPT
  })

  try {
    window.open(deeplink, '_self')
    message.info('正在打开 CC Switch，如未唤起请先安装并注册协议。')
  } catch (err) {
    console.error('Import to CC Switch failed', err)
    message.error('无法打开 CC Switch，请确认已安装并注册协议。')
  }
}

const handleCcsClientSelect = (clientType: CcSwitchClientType) => {
  if (pendingCcsKey.value) {
    executeCcSwitchImport(pendingCcsKey.value, clientType)
  }
  closeCcsClientSelect()
}

const closeCcsClientSelect = () => {
  showCcsClientSelect.value = false
  pendingCcsKey.value = null
}
</script>
