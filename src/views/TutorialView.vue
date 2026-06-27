<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex flex-col md:flex-row justify-between md:items-end gap-4 mb-6">
      <div>
        <h2 class="text-2xl font-bold text-white mb-2">API 接入教程</h2>
        <p class="text-zinc-400">了解如何将 Hapi 的 API 集成到您的开发环境和应用中</p>
      </div>
    </div>

    <!-- Base URL Info -->
    <div class="glass-panel p-6 flex flex-col md:flex-row md:items-center justify-between gap-4 border-l-4 border-l-purple-500">
      <div>
        <h3 class="text-lg font-bold text-white mb-1">接口基础地址 (Base URL)</h3>
        <p class="text-sm text-zinc-400">所有的 API 请求都应指向此地址</p>
      </div>
      <div class="flex items-center space-x-2 bg-black/30 px-4 py-2 rounded-lg border border-white/10">
        <code class="text-purple-400 font-mono select-all">https://api.hapi.com/v1</code>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- Method 1: Config File -->
      <div class="glass-panel p-6 relative overflow-hidden group hover:border-blue-500/30 transition-colors">
        <div class="absolute top-0 right-0 w-32 h-32 bg-blue-500/10 rounded-full blur-3xl opacity-20 -translate-y-1/2 translate-x-1/2 group-hover:opacity-40 transition-opacity"></div>
        
        <div class="flex items-center space-x-3 mb-6 border-b border-white/10 pb-4 relative z-10">
          <div class="w-10 h-10 rounded-xl bg-blue-500/10 text-blue-400 flex items-center justify-center">
            <FileCode class="w-5 h-5" />
          </div>
          <h3 class="text-xl font-bold text-white">1. 改配置文件方式</h3>
        </div>
        
        <div class="space-y-4 text-zinc-300 text-sm leading-relaxed relative z-10">
          <p>适用于需要直接在代码、IDE 或第三方客户端中配置 API 代理地址的场景（例如 Cursor、Cline、NextChat 等）。</p>
          
          <div class="bg-black/20 p-4 rounded-xl border border-white/5 space-y-3">
            <h4 class="font-medium text-white mb-2">基本配置参数</h4>
            <div class="grid grid-cols-[80px_1fr] gap-y-3 gap-x-2">
              <div class="text-zinc-500 font-medium">Base URL</div>
              <code class="text-blue-400 font-mono text-xs bg-blue-400/10 px-2 py-0.5 rounded w-fit">https://api.hapi.com/v1</code>
              
              <div class="text-zinc-500 font-medium">API Key</div>
              <div>
                <code class="text-green-400 font-mono text-xs bg-green-400/10 px-2 py-0.5 rounded">sk-xxxxxxxxxxxxxxxxxxxx</code> 
                <p class="text-xs text-zinc-500 mt-1">在「API 密钥」页面生成</p>
              </div>
              
              <div class="text-zinc-500 font-medium">Model</div>
              <div>
                <code class="text-purple-400 font-mono text-xs bg-purple-400/10 px-2 py-0.5 rounded">claude-3-5-sonnet-20240620</code> 
                <p class="text-xs text-zinc-500 mt-1">参考「可用模型」页面</p>
              </div>
            </div>
          </div>
          
          <h4 class="font-medium text-white mt-4 mb-2">常见客户端配置指南</h4>
          
          <div class="flex space-x-2 border-b border-white/10 mb-4 overflow-x-auto custom-scrollbar pb-1">
            <button @click="activeTab = 'cursor'" :class="activeTab === 'cursor' ? 'text-blue-400 border-b-2 border-blue-400' : 'text-zinc-500 hover:text-zinc-300'" class="px-3 py-1.5 text-xs font-medium whitespace-nowrap transition-colors">Cursor</button>
            <button @click="activeTab = 'cline'" :class="activeTab === 'cline' ? 'text-blue-400 border-b-2 border-blue-400' : 'text-zinc-500 hover:text-zinc-300'" class="px-3 py-1.5 text-xs font-medium whitespace-nowrap transition-colors">Cline / CodeX</button>
            <button @click="activeTab = 'claude'" :class="activeTab === 'claude' ? 'text-blue-400 border-b-2 border-blue-400' : 'text-zinc-500 hover:text-zinc-300'" class="px-3 py-1.5 text-xs font-medium whitespace-nowrap transition-colors">Claude Code</button>
            <button @click="activeTab = 'openai'" :class="activeTab === 'openai' ? 'text-blue-400 border-b-2 border-blue-400' : 'text-zinc-500 hover:text-zinc-300'" class="px-3 py-1.5 text-xs font-medium whitespace-nowrap transition-colors">OpenAI SDK</button>
          </div>

          <div v-if="activeTab === 'cursor'" class="bg-[#1e1e1e] p-4 rounded-xl border border-white/10 text-xs text-zinc-300 space-y-3 leading-relaxed shadow-inner">
            <p>1. 打开 Cursor 设置面板页 (<code class="bg-black/30 px-1 py-0.5 rounded text-zinc-400">Cmd/Ctrl + ,</code>)。</p>
            <p>2. 导航至 <strong>Models</strong> 配置项卡。</p>
            <p>3. 找到 <strong>OpenAI API Key</strong> 处，填入本站的 <code class="text-green-400">sk-...</code> 密钥（或者开启 Anthropic API Key 开关填入均可）。</p>
            <p>4. 展开 <strong>Advanced</strong> 或 <strong>Base URL</strong> 覆盖设置（Override OpenAI Base URL），将其修改为：<br><code class="text-blue-400 bg-blue-400/10 px-2 py-0.5 rounded inline-block mt-2">https://api.hapi.com/v1</code></p>
          </div>

          <div v-if="activeTab === 'cline'" class="bg-[#1e1e1e] p-4 rounded-xl border border-white/10 text-xs text-zinc-300 space-y-3 leading-relaxed shadow-inner">
            <p>1. 在 VSCode 中打开 Cline 或 CodeX 的设置面板。</p>
            <p>2. <strong>API Provider</strong> 请选择 <code class="text-amber-400 bg-amber-400/10 px-1 py-0.5 rounded">OpenAI Compatible</code>。</p>
            <p>3. <strong>Base URL</strong> 填入：<code class="text-blue-400 bg-blue-400/10 px-1 py-0.5 rounded">https://api.hapi.com/v1</code></p>
            <p>4. <strong>API Key</strong> 填入本站生成的 <code class="text-green-400 bg-green-400/10 px-1 py-0.5 rounded">sk-...</code> 密钥。</p>
            <p>5. <strong>Model ID</strong> 手动输入本站支持的模型名称（例如 <code class="text-purple-400 bg-purple-400/10 px-1 py-0.5 rounded">claude-3-5-sonnet-20240620</code>）。</p>
          </div>

          <div v-if="activeTab === 'claude'" class="bg-[#1e1e1e] p-4 rounded-xl border border-white/10 text-xs font-mono text-zinc-300 space-y-3 shadow-inner">
            <div class="text-zinc-500 mb-1"># 在终端中通过环境变量使用 Claude Code</div>
            <pre class="leading-relaxed"><code><span class="text-purple-400">export</span> <span class="text-blue-400">ANTHROPIC_BASE_URL</span>=<span class="text-green-400">"https://api.hapi.com"</span>
<span class="text-purple-400">export</span> <span class="text-blue-400">ANTHROPIC_API_KEY</span>=<span class="text-green-400">"sk-your-api-key"</span>

<span class="text-zinc-500"># 然后正常启动即可</span>
<span class="text-sky-400">claude</span></code></pre>
          </div>

          <div v-if="activeTab === 'openai'" class="bg-[#1e1e1e] p-4 rounded-xl overflow-x-auto text-xs font-mono border border-white/10 shadow-inner">
            <pre class="leading-relaxed text-zinc-300"><code><span class="text-purple-400">from</span> openai <span class="text-purple-400">import</span> OpenAI

client = OpenAI(
    api_key=<span class="text-green-400">"sk-your-api-key"</span>,
    base_url=<span class="text-green-400">"https://api.hapi.com/v1"</span>
)

response = client.chat.completions.create(
    model=<span class="text-green-400">"gpt-4o"</span>,
    messages=[
        {<span class="text-blue-400">"role"</span>: <span class="text-green-400">"user"</span>, <span class="text-blue-400">"content"</span>: <span class="text-green-400">"Hello!"</span>}
    ]
)
<span class="text-sky-400">print</span>(response.choices[0].message.content)</code></pre>
          </div>
        </div>
      </div>

      <!-- Method 2: CCSwitch -->
      <div class="glass-panel p-6 relative overflow-hidden group hover:border-amber-500/30 transition-colors">
        <div class="absolute top-0 right-0 w-32 h-32 bg-amber-500/10 rounded-full blur-3xl opacity-20 -translate-y-1/2 translate-x-1/2 group-hover:opacity-40 transition-opacity"></div>

        <div class="flex items-center space-x-3 mb-6 border-b border-white/10 pb-4 relative z-10">
          <div class="w-10 h-10 rounded-xl bg-amber-500/10 text-amber-400 flex items-center justify-center">
            <ToggleRight class="w-5 h-5" />
          </div>
          <h3 class="text-xl font-bold text-white">2. 使用 ccswitch 方式</h3>
        </div>
        
        <div class="space-y-4 text-zinc-300 text-sm leading-relaxed relative z-10">
          <p>适用于需要全局代理应用流量、拦截客户端自带官方域名并无缝转接到本站 API 的场景。</p>
          
          <div class="bg-black/20 p-4 rounded-xl border border-white/5">
            <h4 class="font-medium text-white mb-2">配置步骤</h4>
            <ol class="list-decimal list-inside space-y-3 text-zinc-400">
              <li>安装并启动 <span class="text-amber-400 font-medium">ccswitch</span> 客户端。</li>
              <li>在配置中添加规则，将官方 API 域名重定向至本站 API：</li>
            </ol>
            
            <div class="mt-3 bg-[#1e1e1e] p-3 rounded-lg border border-white/10 font-mono text-xs shadow-inner">
              <div class="flex justify-between items-center mb-1.5 text-zinc-500 border-b border-white/10 pb-1.5">
                <span>Original Host (源域名)</span>
                <span>Target Host (目标)</span>
              </div>
              <div class="flex justify-between items-center text-zinc-300 py-1">
                <span>api.openai.com</span>
                <span class="text-purple-400">api.hapi.com</span>
              </div>
              <div class="flex justify-between items-center text-zinc-300 py-1">
                <span>api.anthropic.com</span>
                <span class="text-purple-400">api.hapi.com</span>
              </div>
            </div>
          </div>
          
          <div class="bg-amber-500/10 border border-amber-500/20 p-4 rounded-xl text-amber-200/80 shadow-[inset_0_0_20px_rgba(245,158,11,0.05)] mt-6">
            <h4 class="font-medium text-amber-400 mb-2 flex items-center"><Info class="w-4 h-4 mr-1.5" /> 替换 API Key</h4>
            <p class="text-xs leading-relaxed">
              在客户端中，您只需要把原来的官方 API Key，直接替换为在本站生成的 <code class="text-amber-300 bg-amber-500/20 px-1 py-0.5 rounded mx-0.5">sk-...</code> 密钥即可。其他代码或客户端配置完全无需修改，开箱即用。
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { FileCode, ToggleRight, Info } from '@lucide/vue'

const activeTab = ref('cursor')
</script>
