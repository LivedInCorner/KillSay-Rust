<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useConfigStore } from "@/stores/config";
import type { Config } from "@/types";

const emit = defineEmits<{
  close: [];
}>();

const configStore = useConfigStore();

// 编辑状态
const activeTab = ref("snipers");
const config = ref<Config>({
  sniper_list: [],
  join_patterns: [],
  anti_snipe_messages: [],
  kill_patterns: [],
  kill_messages: [],
  message_prefix: "",
  killsay_format: "{prefix}{message}",
});

// Toast状态
const showToast = ref(false);
const toastMessage = ref("");
const toastType = ref<"success" | "error">("success");

// 文本内容（用于编辑）
const sniperText = ref("");
const patternText = ref("");
const antiSnipeText = ref("");
const killPatternText = ref("");
const killMessageText = ref("");

// 标签页配置
const tabs = [
  { id: "snipers", label: "狙击手名单" },
  { id: "patterns", label: "加入检测模式" },
  { id: "messages", label: "反狙击消息" },
  { id: "killPatterns", label: "击杀检测模式" },
  { id: "killMessages", label: "击杀消息" },
  { id: "format", label: "消息格式" },
];

// 显示Toast
function showToastMessage(message: string, type: "success" | "error" = "success") {
  toastMessage.value = message;
  toastType.value = type;
  showToast.value = true;
  setTimeout(() => {
    showToast.value = false;
  }, 2000);
}

// 格式预览
const formatPreview = computed(() => {
  const format = config.value.killsay_format || "{prefix}{message}";
  const prefix = config.value.message_prefix || "";
  const sampleMessage = "十步杀一人，千里不留行。";
  
  return format
    .replace("{prefix}", prefix)
    .replace("{message}", sampleMessage)
    .replace("{k}", "Player")
    .replace("{d}", "Enemy")
    .replace("{kills}", "5");
});

// 加载配置
onMounted(() => {
  if (configStore.currentConfig) {
    config.value = { ...configStore.currentConfig };
    sniperText.value = config.value.sniper_list.join("\n");
    patternText.value = config.value.join_patterns.join("\n");
    antiSnipeText.value = config.value.anti_snipe_messages.join("\n");
    killPatternText.value = config.value.kill_patterns.join("\n");
    killMessageText.value = config.value.kill_messages.join("\n");
  }
});

// 保存配置
async function saveConfig() {
  // 从文本解析配置
  config.value.sniper_list = sniperText.value.split("\n").filter(Boolean);
  config.value.join_patterns = patternText.value.split("\n").filter(Boolean);
  config.value.anti_snipe_messages = antiSnipeText.value.split("\n").filter(Boolean);
  config.value.kill_patterns = killPatternText.value.split("\n").filter(Boolean);
  config.value.kill_messages = killMessageText.value.split("\n").filter(Boolean);
  
  try {
    // 获取当前配置路径
    const configList = configStore.configList;
    const currentName = configStore.currentConfigName;
    const currentPath = configList.find(c => c.name === currentName)?.path;
    
    if (currentPath) {
      await configStore.saveConfig(currentPath, config.value);
      showToastMessage("配置已保存");
      setTimeout(() => emit("close"), 500);
    } else {
      showToastMessage("无法找到当前配置路径", "error");
    }
  } catch (e) {
    showToastMessage(`保存失败: ${e}`, "error");
  }
}
</script>

<template>
  <div class="dialog-overlay" @mousedown.self="emit('close')">
    <div class="config-editor dg-panel">
      <!-- 标题栏 -->
      <div class="editor-header">
        <h2 class="dg-title">编辑配置</h2>
        <button class="dg-btn dg-btn--secondary" @click="emit('close')">关闭</button>
      </div>
      
      <!-- 标签页导航 -->
      <div class="tabs-nav">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          class="tab-btn"
          :class="{ active: activeTab === tab.id }"
          @click="activeTab = tab.id"
        >
          {{ tab.label }}
        </button>
      </div>
      
      <!-- 标签页内容 -->
      <div class="tab-content">
        <!-- 狙击手名单 -->
        <div v-if="activeTab === 'snipers'" class="tab-pane">
          <label class="dg-label">狙击手名单（每行一个名字）</label>
          <textarea
            v-model="sniperText"
            class="dg-textarea"
            rows="10"
            placeholder="输入玩家名称，每行一个"
          ></textarea>
        </div>
        
        <!-- 加入检测模式 -->
        <div v-if="activeTab === 'patterns'" class="tab-pane">
          <label class="dg-label">玩家加入游戏检测模式（每行一个正则表达式）</label>
          <textarea
            v-model="patternText"
            class="dg-textarea"
            rows="10"
            placeholder="输入正则表达式，每行一个"
          ></textarea>
          <p class="help-text">示例: (.+?)加入了游戏</p>
        </div>
        
        <!-- 反狙击消息 -->
        <div v-if="activeTab === 'messages'" class="tab-pane">
          <label class="dg-label">检测到狙击手时发送的消息（每行一个）</label>
          <textarea
            v-model="antiSnipeText"
            class="dg-textarea"
            rows="10"
            placeholder="输入消息，每行一个"
          ></textarea>
        </div>
        
        <!-- 击杀检测模式 -->
        <div v-if="activeTab === 'killPatterns'" class="tab-pane">
          <label class="dg-label">击杀检测模式（使用 %u、%o 和 %n 占位符）</label>
          <textarea
            v-model="killPatternText"
            class="dg-textarea"
            rows="10"
            placeholder="输入击杀模式，每行一个"
          ></textarea>
          <p class="help-text">
            提示: %u=用户(击杀者), %o=对象(被击杀者), %n=任意数字<br/>
            示例: %o被%u击败 或 %u击败了%o
          </p>
        </div>
        
        <!-- 击杀消息 -->
        <div v-if="activeTab === 'killMessages'" class="tab-pane">
          <label class="dg-label">击杀后发送的消息（可使用 %u 和 %o 占位符）</label>
          <textarea
            v-model="killMessageText"
            class="dg-textarea"
            rows="10"
            placeholder="输入击杀消息，每行一个"
          ></textarea>
          <p class="help-text">
            提示: %u=用户(击杀者), %o=对象(被击杀者)<br/>
            如果没有占位符，则发送原消息
          </p>
        </div>
        
        <!-- 消息格式 -->
        <div v-if="activeTab === 'format'" class="tab-pane">
          <div class="format-section">
            <label class="dg-label">消息前缀</label>
            <input
              v-model="config.message_prefix"
              class="dg-input"
              placeholder="例如: [KILLSAY]"
            />
            <p class="help-text">发送消息前添加的前缀文本</p>
          </div>
          
          <div class="format-section">
            <label class="dg-label">消息格式模板</label>
            <input
              v-model="config.killsay_format"
              class="dg-input"
              placeholder="{prefix}{message}"
            />
            <p class="help-text">
              可用占位符:<br/>
              {prefix} - 消息前缀<br/>
              {message} - 格式化后的击杀消息<br/>
              {k} - 击杀者名称<br/>
              {d} - 被击杀者名称<br/>
              {kills} - 当前击杀数
            </p>
          </div>
          
          <div class="format-preview">
            <label class="dg-label">预览效果</label>
            <div class="preview-box">
              {{ formatPreview }}
            </div>
          </div>
        </div>
      </div>
      
      <!-- 操作按钮 -->
      <div class="editor-actions">
        <button class="dg-btn dg-btn--primary" @click="saveConfig">保存</button>
        <button class="dg-btn dg-btn--secondary" @click="emit('close')">取消</button>
      </div>
    </div>
    
    <!-- Toast 提示 -->
    <Transition name="toast">
      <div v-if="showToast" class="toast" :class="`toast-${toastType}`">
        <svg v-if="toastType === 'success'" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
          <polyline points="22 4 12 14.01 9 11.01" />
        </svg>
        <svg v-else width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10" />
          <line x1="15" y1="9" x2="9" y2="15" />
          <line x1="9" y1="9" x2="15" y2="15" />
        </svg>
        <span>{{ toastMessage }}</span>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.config-editor {
  width: 700px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-md);
}

/* 标签页导航 */
.tabs-nav {
  display: flex;
  gap: 4px;
  padding: 4px;
  background: rgba(0, 0, 0, 0.2);
  border-radius: var(--radius-small);
  margin-bottom: var(--spacing-md);
}

.tab-btn {
  flex: 1;
  padding: 8px 12px;
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
  background: transparent;
  border: none;
  border-radius: var(--radius-small);
  cursor: pointer;
  transition: all 0.2s ease;
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

/* 标签页内容 */
.tab-content {
  flex: 1;
  overflow-y: auto;
  margin-bottom: var(--spacing-md);
}

.tab-pane {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.help-text {
  font-size: 12px;
  color: var(--text-secondary);
  line-height: 1.6;
}

/* 格式设置 */
.format-section {
  margin-bottom: var(--spacing-md);
}

.format-preview {
  margin-top: var(--spacing-md);
}

.preview-box {
  padding: var(--spacing-sm);
  background: rgba(0, 0, 0, 0.3);
  border-radius: var(--radius-small);
  font-family: var(--font-mono);
  font-size: 13px;
  color: var(--accent-yellow);
  word-break: break-all;
}

/* 操作按钮 */
.editor-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
  animation: fadeIn 0.2s ease;
}

.config-editor {
  animation: slideUp 0.25s ease;
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

/* Toast */
.toast {
  position: fixed;
  top: 50px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  border-radius: var(--radius-card);
  font-size: 13px;
  font-weight: 500;
  z-index: 200;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
}

.toast-success {
  background: rgba(34, 197, 94, 0.9);
  color: white;
}

.toast-error {
  background: rgba(239, 68, 68, 0.9);
  color: white;
}

.toast-enter-active {
  transition: all 0.3s ease-out;
}

.toast-leave-active {
  transition: all 0.3s ease-in;
}

.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateX(-50%) translateY(-20px);
}
</style>
