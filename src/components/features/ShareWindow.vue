<script setup lang="ts">
import { ref, computed } from "vue";
import { useConfigStore } from "@/stores/config";
import { message } from "@tauri-apps/plugin-dialog";
import GeoElement from "@/components/ui/GeoElement.vue";
import GeoLine from "@/components/ui/GeoLine.vue";

const emit = defineEmits<{
  close: [];
}>();

const configStore = useConfigStore();

const activeTab = ref<"generate" | "import">("generate");
const shareCode = ref("");
const importCode = ref("");
const includeName = ref(true);

// Generate share code
function generateShareCode() {
  if (!configStore.currentConfig) {
    return;
  }
  
  const data: Record<string, any> = {
    v: 1,
  };
  
  if (includeName.value && configStore.currentConfigName) {
    data.name = configStore.currentConfigName;
  }
  
  data.config = configStore.currentConfig;
  
  // Convert to compact JSON and base64 encode
  const jsonStr = JSON.stringify(data);
  const base64 = btoa(unescape(encodeURIComponent(jsonStr)));
  shareCode.value = `KS1-${base64}`;
}

// Copy to clipboard
async function copyToClipboard() {
  try {
    await navigator.clipboard.writeText(shareCode.value);
    await message("已复制到剪贴板", { title: "KILLSAY", kind: "info" });
  } catch (e) {
    // Fallback for older browsers
    const textArea = document.createElement("textarea");
    textArea.value = shareCode.value;
    document.body.appendChild(textArea);
    textArea.select();
    document.execCommand("copy");
    document.body.removeChild(textArea);
    await message("已复制到剪贴板", { title: "KILLSAY", kind: "info" });
  }
}

// Import share code
async function handleImport() {
  const code = importCode.value.trim();
  
  if (!code) {
    await message("请输入分享码", { title: "KILLSAY", kind: "warning" });
    return;
  }
  
  // Validate prefix
  if (!code.startsWith("KS1-")) {
    await message("无效的分享码格式", { title: "KILLSAY", kind: "error" });
    return;
  }
  
  try {
    // Remove prefix and decode base64
    const base64 = code.substring(4);
    const jsonStr = decodeURIComponent(escape(atob(base64)));
    const data = JSON.parse(jsonStr);
    
    // Validate version
    if (!data.v || data.v !== 1) {
      await message("不支持的分享码版本", { title: "KILLSAY", kind: "error" });
      return;
    }
    
    // Validate config
    if (!data.config) {
      await message("分享码中没有配置数据", { title: "KILLSAY", kind: "error" });
      return;
    }
    
    // Save config
    const configName = data.name || "imported";
    const existingConfig = configStore.configList.find(c => c.name === configName);
    
    if (existingConfig) {
      await configStore.saveConfig(existingConfig.path, data.config);
    } else {
      await configStore.createConfig(configName);
      const newConfig = configStore.configList.find(c => c.name === configName);
      if (newConfig) {
        await configStore.saveConfig(newConfig.path, data.config);
      }
    }
    
    // Reload config list
    await configStore.fetchConfigList();
    
    // Try to load the imported config
    const importedConfig = configStore.configList.find(c => c.name === configName);
    if (importedConfig) {
      await configStore.loadConfig(importedConfig.path);
    }
    
    await message("导入成功！", { title: "KILLSAY", kind: "info" });
    emit("close");
  } catch (e) {
    await message(`导入失败: 分享码格式错误`, { title: "KILLSAY", kind: "error" });
  }
}

// Auto-generate on mount
if (configStore.currentConfig) {
  generateShareCode();
}
</script>

<template>
  <div class="share-overlay" @mousedown.self="emit('close')">
    <div class="share-panel dg-panel">
      <!-- 标题栏 -->
      <div class="share-header">
        <div class="share-title-area">
          <GeoElement type="hexagon" :size="18" color="var(--accent-yellow)" />
          <h2 class="share-title">配置分享码</h2>
        </div>
        <button class="dg-btn dg-btn--secondary dg-btn--sm" @click="emit('close')">关闭</button>
      </div>
      
      <!-- 标签页切换 -->
      <div class="share-tabs">
        <button
          class="tab-btn"
          :class="{ active: activeTab === 'generate' }"
          @click="activeTab = 'generate'"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
            <line x1="3" y1="9" x2="21" y2="9" />
            <line x1="9" y1="21" x2="9" y2="9" />
          </svg>
          生成分享码
        </button>
        <button
          class="tab-btn"
          :class="{ active: activeTab === 'import' }"
          @click="activeTab = 'import'"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M17 8l-5-5-5 5M12 3v12" />
          </svg>
          导入分享码
        </button>
      </div>
      
      <GeoLine direction="horizontal" :length="100" />
      
      <!-- 生成分享码 -->
      <div v-if="activeTab === 'generate'" class="share-content">
        <div class="current-config">
          <span class="label">当前配置:</span>
          <span class="value">{{ configStore.currentConfigName || "未加载" }}</span>
        </div>
        
        <label class="option-item">
          <input type="checkbox" v-model="includeName" @change="generateShareCode" />
          <div class="option-info">
            <span class="option-name">包含配置名称</span>
            <span class="option-desc">导入时将使用此名称</span>
          </div>
        </label>
        
        <div class="code-area">
          <label class="dg-label">分享码</label>
          <div class="code-box">
            <textarea
              v-model="shareCode"
              class="code-textarea"
              readonly
              rows="4"
              @click="copyToClipboard"
            ></textarea>
            <button class="copy-btn" @click="copyToClipboard" title="复制">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
                <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1" />
              </svg>
            </button>
          </div>
          <p class="code-hint">点击分享码或复制按钮复制到剪贴板</p>
        </div>
        
        <div class="protocol-info">
          <GeoElement type="diamond" :size="10" color="var(--accent-blue)" />
          <span>分享码格式: KS1-Base64编码JSON</span>
        </div>
      </div>
      
      <!-- 导入分享码 -->
      <div v-if="activeTab === 'import'" class="share-content">
        <div class="code-area">
          <label class="dg-label">粘贴分享码</label>
          <textarea
            v-model="importCode"
            class="code-textarea"
            rows="4"
            placeholder="KS1-..."
          ></textarea>
        </div>
        
        <div class="protocol-info">
          <GeoElement type="diamond" :size="10" color="var(--accent-blue)" />
          <span>支持 KS1 格式的分享码</span>
        </div>
      </div>
      
      <!-- 操作按钮 -->
      <div class="share-actions">
        <button
          v-if="activeTab === 'generate'"
          class="dg-btn dg-btn--primary"
          @click="generateShareCode"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="1 4 1 10 7 10" />
            <path d="M3.51 15a9 9 0 1 0 2.13-9.36L1 10" />
          </svg>
          重新生成
        </button>
        <button
          v-else
          class="dg-btn dg-btn--primary"
          @click="handleImport"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M17 8l-5-5-5 5M12 3v12" />
          </svg>
          导入配置
        </button>
        <button class="dg-btn dg-btn--secondary" @click="emit('close')">取消</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.share-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 200;
  backdrop-filter: blur(4px);
  animation: fadeIn 0.2s ease;
}

.share-panel {
  width: 480px;
  animation: slideUp 0.25s ease;
}

.share-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-md);
}

.share-title-area {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.share-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

/* Tabs */
.share-tabs {
  display: flex;
  gap: 4px;
  padding: 4px;
  background: rgba(0, 0, 0, 0.2);
  border-radius: var(--radius-small);
  margin-bottom: var(--spacing-md);
}

.tab-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 8px 12px;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
  background: transparent;
  border: none;
  border-radius: var(--radius-small);
  cursor: pointer;
  transition: all 0.15s ease;
}

.tab-btn:hover {
  color: var(--text-primary);
  background: rgba(255, 255, 255, 0.05);
}

.tab-btn.active {
  color: var(--bg-primary);
  background: var(--accent-yellow);
  font-weight: 600;
}

/* Content */
.share-content {
  margin-top: var(--spacing-md);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.current-config {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  background: rgba(0, 0, 0, 0.2);
  border-radius: var(--radius-small);
}

.current-config .label {
  font-size: 12px;
  color: var(--text-secondary);
}

.current-config .value {
  font-size: 13px;
  color: var(--accent-yellow);
  font-weight: 500;
}

.option-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  background: rgba(0, 0, 0, 0.2);
  border-radius: var(--radius-small);
  cursor: pointer;
}

.option-item input[type="checkbox"] {
  width: 16px;
  height: 16px;
  accent-color: var(--accent-yellow);
  cursor: pointer;
}

.option-info {
  display: flex;
  flex-direction: column;
}

.option-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.option-desc {
  font-size: 11px;
  color: var(--text-secondary);
}

/* Code area */
.code-area {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.code-box {
  position: relative;
}

.code-textarea {
  width: 100%;
  padding: 12px;
  padding-right: 44px;
  font-family: var(--font-mono);
  font-size: 11px;
  color: var(--text-primary);
  background: var(--bg-panel);
  border: 1px solid var(--border-light);
  border-radius: var(--radius-small);
  resize: none;
  outline: none;
  transition: border-color 0.2s ease;
  word-break: break-all;
}

.code-textarea:focus {
  border-color: var(--accent-yellow);
}

.copy-btn {
  position: absolute;
  top: 8px;
  right: 8px;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border-light);
  border-radius: 4px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.15s ease;
}

.copy-btn:hover {
  background: var(--accent-yellow);
  color: var(--bg-primary);
  border-color: var(--accent-yellow);
}

.code-hint {
  font-size: 11px;
  color: var(--text-secondary);
}

.protocol-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  background: rgba(96, 165, 250, 0.1);
  border-radius: var(--radius-small);
  font-size: 12px;
  color: var(--text-secondary);
}

/* Actions */
.share-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  margin-top: var(--spacing-lg);
}

/* Animations */
@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(20px) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}
</style>
