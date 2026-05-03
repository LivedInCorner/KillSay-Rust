<script setup lang="ts">
import { ref } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useThemeStore } from "@/stores/theme";

const emit = defineEmits<{
  openSettings: [];
  openColorPicker: [];
  openShare: [];
}>();

const themeStore = useThemeStore();
const isMaximized = ref(false);

async function minimizeWindow() {
  await getCurrentWindow().minimize();
}

async function toggleMaximize() {
  isMaximized.value = !isMaximized.value;
  await getCurrentWindow().toggleMaximize();
}

async function closeWindow() {
  await getCurrentWindow().close();
}
</script>

<template>
  <div class="title-bar" data-tauri-drag-region>
    <!-- 左侧图标和标题 -->
    <div class="title-bar__left">
      <img src="/icons/icon.svg" alt="KILLSAY" class="title-bar__icon" />
      <span class="title-bar__title">KILLSAY</span>
    </div>
    
    <!-- 右侧控制按钮 -->
    <div class="title-bar__right">
      <button class="title-bar__btn share-btn" @click="emit('openShare')" title="配置分享">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="18" cy="5" r="3" />
          <circle cx="6" cy="12" r="3" />
          <circle cx="18" cy="19" r="3" />
          <line x1="8.59" y1="13.51" x2="15.42" y2="17.49" />
          <line x1="15.41" y1="6.51" x2="8.59" y2="10.49" />
        </svg>
      </button>
      
      <button class="title-bar__btn theme-btn" @click="emit('openColorPicker')" title="主题色">
        <div class="theme-color-dot" :style="{ backgroundColor: themeStore.accentColor }"></div>
      </button>
      
      <button class="title-bar__btn settings-btn" @click="emit('openSettings')" title="设置">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="3" />
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
        </svg>
      </button>
      
      <div class="title-bar__separator"></div>
      
      <button class="title-bar__btn minimize-btn" @click="minimizeWindow">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <rect x="1" y="5.5" width="10" height="1" fill="currentColor" />
        </svg>
      </button>
      <button class="title-bar__btn maximize-btn" @click="toggleMaximize">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <rect x="1.5" y="1.5" width="9" height="9" fill="none" stroke="currentColor" stroke-width="1" />
        </svg>
      </button>
      <button class="title-bar__btn close-btn" @click="closeWindow">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <path d="M1 1L11 11M1 11L11 1" stroke="currentColor" stroke-width="1.5" />
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.title-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 36px;
  padding: 0 12px;
  background: rgba(10, 10, 18, 0.95);
  border-bottom: 1px solid var(--border-light);
  user-select: none;
  position: relative;
  z-index: 100;
}

.title-bar__left {
  display: flex;
  align-items: center;
  gap: 8px;
  pointer-events: none;
}

.title-bar__icon {
  width: 18px;
  height: 18px;
  object-fit: contain;
}

.title-bar__title {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-primary);
  letter-spacing: 2px;
}

.title-bar__right {
  display: flex;
  align-items: center;
  gap: 2px;
}

.title-bar__btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 28px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.15s ease;
}

.title-bar__btn:hover {
  background: rgba(255, 255, 255, 0.08);
  color: var(--text-primary);
}

.title-bar__separator {
  width: 1px;
  height: 16px;
  background: var(--border-light);
  margin: 0 4px;
}

.theme-color-dot {
  width: 14px;
  height: 14px;
  border-radius: 50%;
  border: 2px solid rgba(255, 255, 255, 0.2);
  transition: all 0.2s ease;
}

.theme-btn:hover .theme-color-dot {
  transform: scale(1.1);
  border-color: rgba(255, 255, 255, 0.4);
}

.share-btn:hover {
  color: var(--accent-blue);
}

.settings-btn:hover {
  color: var(--accent-yellow);
}

.close-btn:hover {
  background: rgba(239, 68, 68, 0.8);
  color: white;
}
</style>
