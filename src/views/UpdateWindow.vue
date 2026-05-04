<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { UpdateInfo, DownloadProgress } from "@/types";
import GeoElement from "@/components/ui/GeoElement.vue";
import GeoLine from "@/components/ui/GeoLine.vue";

const emit = defineEmits<{
  close: [];
}>();

const updateInfo = ref<UpdateInfo | null>(null);
const loading = ref(false);
const downloading = ref(false);
const downloadProgress = ref<DownloadProgress | null>(null);
const installerPath = ref<string | null>(null);
const error = ref<string | null>(null);
const autoUpdate = ref(false);

let unlistenProgress: (() => void) | null = null;

// Check for update
async function checkForUpdate() {
  loading.value = true;
  error.value = null;
  try {
    updateInfo.value = await invoke<UpdateInfo>("check_for_update");
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
}

// Download update
async function downloadUpdate() {
  if (!updateInfo.value) return;
  
  downloading.value = true;
  downloadProgress.value = null;
  error.value = null;
  
  try {
    const path = await invoke<string>("download_update", { 
      version: updateInfo.value.latest_version 
    });
    installerPath.value = path;
  } catch (e) {
    error.value = String(e);
  } finally {
    downloading.value = false;
  }
}

// Install and restart
async function installAndRestart() {
  if (!installerPath.value) return;
  
  try {
    await invoke("install_update", { installerPath: installerPath.value });
  } catch (e) {
    error.value = String(e);
  }
}

// Load auto-update setting
onMounted(async () => {
  const saved = localStorage.getItem("killsay-auto-update");
  autoUpdate.value = saved === "true";
  
  // Listen for download progress
  unlistenProgress = await listen<DownloadProgress>("update-download-progress", (event) => {
    downloadProgress.value = event.payload;
  });
  
  checkForUpdate();
});

onUnmounted(() => {
  if (unlistenProgress) {
    unlistenProgress();
  }
});

// Toggle auto-update
function toggleAutoUpdate() {
  autoUpdate.value = !autoUpdate.value;
  localStorage.setItem("killsay-auto-update", String(autoUpdate.value));
}

// Format bytes
function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
}
</script>

<template>
  <div class="update-overlay" @mousedown.self="emit('close')">
    <div class="update-panel dg-panel">
      <!-- 标题栏 -->
      <div class="update-header">
        <div class="update-title-area">
          <GeoElement type="hexagon" :size="18" color="var(--accent-yellow)" />
          <h2 class="update-title">检查更新</h2>
        </div>
        <button class="dg-btn dg-btn--secondary dg-btn--sm" @click="emit('close')">关闭</button>
      </div>
      
      <GeoLine direction="horizontal" :length="100" />
      
      <!-- 更新设置 -->
      <div class="update-settings">
        <label class="setting-item" @click="toggleAutoUpdate">
          <div class="setting-toggle" :class="{ active: autoUpdate }">
            <div class="toggle-thumb"></div>
          </div>
          <div class="setting-info">
            <span class="setting-name">启动时自动检查更新</span>
            <span class="setting-desc">开启后每次启动会自动检查新版本</span>
          </div>
        </label>
      </div>
      
      <GeoLine direction="horizontal" :length="100" />
      
      <!-- 更新状态 -->
      <div class="update-status">
        <!-- 加载中 -->
        <div v-if="loading" class="status-loading">
          <div class="loading-spinner"></div>
          <span>正在检查更新...</span>
        </div>
        
        <!-- 错误 -->
        <div v-else-if="error" class="status-error">
          <GeoElement type="triangle" :size="16" color="var(--accent-red)" />
          <span>{{ error }}</span>
          <button class="dg-btn dg-btn--secondary dg-btn--sm" @click="checkForUpdate">重试</button>
        </div>
        
        <!-- 下载中 -->
        <div v-else-if="downloading" class="status-downloading">
          <div class="download-info">
            <span class="download-title">正在下载 v{{ updateInfo?.latest_version }}...</span>
            <div v-if="downloadProgress" class="download-progress">
              <div class="progress-bar">
                <div class="progress-fill" :style="{ width: downloadProgress.percentage + '%' }"></div>
              </div>
              <div class="progress-text">
                <span>{{ downloadProgress.percentage }}%</span>
                <span>{{ formatBytes(downloadProgress.downloaded) }} / {{ formatBytes(downloadProgress.total) }}</span>
              </div>
            </div>
            <div v-else class="download-waiting">
              <div class="loading-spinner"></div>
              <span>准备下载...</span>
            </div>
          </div>
        </div>
        
        <!-- 下载完成 -->
        <div v-else-if="installerPath" class="status-ready">
          <GeoElement type="circle" :size="24" color="var(--accent-green, #22c55e)" />
          <div class="ready-info">
            <span class="ready-title">下载完成</span>
            <span class="ready-desc">点击安装将关闭应用并启动安装程序</span>
          </div>
          <button class="dg-btn dg-btn--primary" @click="installAndRestart">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
              <polyline points="7 10 12 15 17 10" />
              <line x1="12" y1="15" x2="12" y2="3" />
            </svg>
            立即安装并重启
          </button>
        </div>
        
        <!-- 有更新 -->
        <div v-else-if="updateInfo?.has_update" class="status-has-update">
          <div class="update-version-info">
            <div class="version-row">
              <span class="label">当前版本:</span>
              <span class="value">{{ updateInfo.current_version }}</span>
            </div>
            <div class="version-row">
              <span class="label">最新版本:</span>
              <span class="value highlight">{{ updateInfo.latest_version }}</span>
            </div>
          </div>
          
          <div v-if="updateInfo.release_notes" class="update-notes">
            <span class="notes-label">更新内容:</span>
            <div class="notes-content">{{ updateInfo.release_notes }}</div>
          </div>
          
          <button class="dg-btn dg-btn--primary" @click="downloadUpdate">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
              <polyline points="7 10 12 15 17 10" />
              <line x1="12" y1="15" x2="12" y2="3" />
            </svg>
            下载更新
          </button>
        </div>
        
        <!-- 已是最新 -->
        <div v-else class="status-latest">
          <GeoElement type="circle" :size="24" color="var(--accent-green, #22c55e)" />
          <div class="latest-info">
            <span class="latest-text">已是最新版本</span>
            <span class="latest-version">v{{ updateInfo?.current_version }}</span>
          </div>
        </div>
      </div>
      
      <!-- 调试信息 -->
      <details v-if="updateInfo?.debug_info" class="debug-info">
        <summary>调试信息</summary>
        <pre>{{ updateInfo.debug_info }}</pre>
      </details>
      
      <!-- 刷新按钮 -->
      <div class="update-actions">
        <button 
          class="dg-btn dg-btn--secondary" 
          @click="checkForUpdate" 
          :disabled="loading || downloading"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" :class="{ spinning: loading }">
            <polyline points="23 4 23 10 17 10" />
            <polyline points="1 20 1 14 7 14" />
            <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15" />
          </svg>
          刷新
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.update-overlay {
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

.update-panel {
  width: 440px;
  animation: slideUp 0.25s ease;
}

.update-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-md);
}

.update-title-area {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.update-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

/* Settings */
.update-settings {
  margin: var(--spacing-md) 0;
}

.setting-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-sm);
  background: rgba(0, 0, 0, 0.2);
  border-radius: var(--radius-small);
  cursor: pointer;
}

.setting-toggle {
  width: 40px;
  height: 20px;
  background: var(--bg-panel);
  border: 1px solid var(--border-light);
  border-radius: 10px;
  position: relative;
  transition: all 0.3s ease;
  flex-shrink: 0;
}

.setting-toggle.active {
  background: var(--accent-yellow);
  border-color: var(--accent-yellow);
}

.toggle-thumb {
  width: 16px;
  height: 16px;
  background: white;
  border-radius: 50%;
  position: absolute;
  top: 1px;
  left: 1px;
  transition: all 0.3s ease;
}

.setting-toggle.active .toggle-thumb {
  transform: translateX(20px);
}

.setting-info {
  display: flex;
  flex-direction: column;
}

.setting-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.setting-desc {
  font-size: 11px;
  color: var(--text-secondary);
}

/* Status */
.update-status {
  margin: var(--spacing-md) 0;
  min-height: 120px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.status-loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-sm);
  color: var(--text-secondary);
  font-size: 13px;
}

.loading-spinner {
  width: 24px;
  height: 24px;
  border: 2px solid var(--border-light);
  border-top-color: var(--accent-yellow);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.status-error {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-sm);
  color: var(--accent-red);
  font-size: 13px;
}

/* Downloading */
.status-downloading {
  width: 100%;
}

.download-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.download-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  text-align: center;
}

.download-progress {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.progress-bar {
  width: 100%;
  height: 8px;
  background: rgba(0, 0, 0, 0.3);
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: var(--accent-yellow);
  border-radius: 4px;
  transition: width 0.3s ease;
}

.progress-text {
  display: flex;
  justify-content: space-between;
  font-size: 11px;
  color: var(--text-secondary);
  font-family: var(--font-mono);
}

.download-waiting {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-sm);
  color: var(--text-secondary);
  font-size: 13px;
}

/* Ready to install */
.status-ready {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-md);
}

.ready-info {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

.ready-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.ready-desc {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 4px;
}

/* Has update */
.status-has-update {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.update-version-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.version-row {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.version-row .label {
  font-size: 13px;
  color: var(--text-secondary);
}

.version-row .value {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  font-family: var(--font-mono);
}

.version-row .value.highlight {
  color: var(--accent-blue);
}

.update-notes {
  background: rgba(0, 0, 0, 0.2);
  border-radius: var(--radius-small);
  padding: var(--spacing-sm);
}

.notes-label {
  font-size: 11px;
  color: var(--text-secondary);
  display: block;
  margin-bottom: var(--spacing-xs);
}

.notes-content {
  font-size: 12px;
  color: var(--text-primary);
  max-height: 100px;
  overflow-y: auto;
  white-space: pre-wrap;
  line-height: 1.5;
}

/* Latest */
.status-latest {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-sm);
}

.latest-info {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.latest-text {
  font-size: 14px;
  color: var(--text-primary);
  font-weight: 500;
}

.latest-version {
  font-size: 12px;
  color: var(--text-secondary);
  font-family: var(--font-mono);
}

/* Debug Info */
.debug-info {
  margin-top: var(--spacing-sm);
  background: rgba(0, 0, 0, 0.2);
  border-radius: var(--radius-small);
  padding: var(--spacing-sm);
}

.debug-info summary {
  font-size: 11px;
  color: var(--text-secondary);
  cursor: pointer;
  user-select: none;
}

.debug-info pre {
  margin-top: var(--spacing-xs);
  font-size: 10px;
  color: var(--text-secondary);
  font-family: var(--font-mono);
  white-space: pre-wrap;
  word-break: break-all;
  max-height: 100px;
  overflow-y: auto;
}

/* Actions */
.update-actions {
  display: flex;
  justify-content: flex-end;
  margin-top: var(--spacing-md);
}

.spinning {
  animation: spin 0.8s linear infinite;
}

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
