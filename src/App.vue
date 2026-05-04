<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useConfigStore } from "./stores/config";
import { useMonitorStore } from "./stores/monitor";
import { useThemeStore } from "./stores/theme";
import type { UpdateInfo } from "./types";
import TitleBar from "./components/layout/TitleBar.vue";
import Home from "./views/Home.vue";
import Settings from "./views/Settings.vue";
import ColorPicker from "./components/ui/ColorPicker.vue";
import ShareWindow from "./components/features/ShareWindow.vue";
import StatsOverlay from "./views/StatsOverlay.vue";
import UpdateWindow from "./views/UpdateWindow.vue";

const configStore = useConfigStore();
const monitorStore = useMonitorStore();
const themeStore = useThemeStore();
const showSettings = ref(false);
const showColorPicker = ref(false);
const showShare = ref(false);
const showUpdateWindow = ref(false);

// Update state
const showUpdate = ref(false);
const updateInfo = ref<UpdateInfo | null>(null);

// Check if this is the stats window
const isStatsWindow = computed(() => {
  const params = new URLSearchParams(window.location.search);
  return params.get("stats") === "true";
});

async function checkForUpdate() {
  try {
    const info = await invoke<UpdateInfo>("check_for_update");
    if (info.has_update) {
      updateInfo.value = info;
      showUpdate.value = true;
    }
  } catch (e) {
    console.error("检查更新失败:", e);
  }
}

function openReleaseUrl() {
  if (updateInfo.value?.release_url) {
    window.open(updateInfo.value.release_url, "_blank");
  }
  showUpdate.value = false;
}

onMounted(async () => {
  // Only initialize config stores for main window
  if (!isStatsWindow.value) {
    await configStore.fetchCurrentConfig();
    await configStore.fetchConfigList();
    await configStore.fetchSettings();
    
    // Check for updates if auto-update is enabled
    const autoUpdate = localStorage.getItem("killsay-auto-update");
    if (autoUpdate === "true") {
      setTimeout(checkForUpdate, 2000);
    }
  }
  
  // 设置监控事件监听
  monitorStore.setupEventListener();
});

onUnmounted(() => {
  monitorStore.cleanup();
});
</script>

<template>
  <!-- Stats Window -->
  <StatsOverlay v-if="isStatsWindow" />
  
  <!-- Main Window -->
  <div v-else class="app-container">
    <!-- 自定义标题栏 -->
    <TitleBar 
      @open-settings="showSettings = true" 
      @open-color-picker="showColorPicker = true"
      @open-share="showShare = true"
      @open-update="showUpdateWindow = true"
    />
    
    <!-- 背景网格 -->
    <div class="bg-grid dg-grid-bg"></div>
    
    <!-- 主内容 -->
    <Home />
    
    <!-- 设置页面 -->
    <Transition name="fade">
      <Settings v-if="showSettings" @close="showSettings = false" />
    </Transition>
    
    <!-- 颜色选择器 -->
    <Transition name="fade">
      <ColorPicker
        v-if="showColorPicker"
        :initial-color="themeStore.accentColor"
        @select="(color) => themeStore.setAccentColor(color)"
        @close="showColorPicker = false"
      />
    </Transition>
    
    <!-- 配置分享 -->
    <Transition name="fade">
      <ShareWindow
        v-if="showShare"
        @close="showShare = false"
      />
    </Transition>
    
    <!-- 更新窗口 -->
    <Transition name="fade">
      <UpdateWindow
        v-if="showUpdateWindow"
        @close="showUpdateWindow = false"
      />
    </Transition>
    
    <!-- 更新提示 -->
    <Transition name="fade">
      <div v-if="showUpdate && updateInfo" class="update-overlay" @mousedown.self="showUpdate = false">
        <div class="update-panel dg-panel">
          <div class="update-header">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="var(--accent-blue)" stroke-width="2">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M7 10l5 5 5-5M12 15V3" />
            </svg>
            <h3>发现新版本</h3>
          </div>
          
          <div class="update-info">
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
          
          <div class="update-actions">
            <button class="dg-btn dg-btn--primary" @click="openReleaseUrl">
              前往下载
            </button>
            <button class="dg-btn dg-btn--secondary" @click="showUpdate = false">
              稍后再说
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.app-container {
  width: 100%;
  height: 100vh;
  position: relative;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.bg-grid {
  position: fixed;
  top: 36px;
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none;
  z-index: 0;
}

/* 更新弹窗 */
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
  width: 420px;
  animation: slideUp 0.25s ease;
}

.update-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-lg);
}

.update-header h3 {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.update-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-md);
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
  margin-bottom: var(--spacing-md);
}

.notes-label {
  font-size: 12px;
  color: var(--text-secondary);
  display: block;
  margin-bottom: var(--spacing-xs);
}

.notes-content {
  font-size: 13px;
  color: var(--text-primary);
  background: rgba(0, 0, 0, 0.2);
  padding: var(--spacing-sm);
  border-radius: var(--radius-small);
  max-height: 150px;
  overflow-y: auto;
  white-space: pre-wrap;
}

.update-actions {
  display: flex;
  gap: var(--spacing-sm);
  justify-content: flex-end;
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
