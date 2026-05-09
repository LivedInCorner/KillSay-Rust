<script setup lang="ts">
import { ref } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useThemeStore } from "@/stores/theme";

const props = defineProps<{
  chatKey?: string;
}>();

const emit = defineEmits<{
  openSettings: [];
  openColorPicker: [];
  openShare: [];
  openUpdate: [];
  toggleTheme: [];
  updateChatKey: [key: string];
}>();

const themeStore = useThemeStore();
const isMaximized = ref(false);
const isEditingKey = ref(false);
let clickTimer: ReturnType<typeof setTimeout> | null = null;

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

function handleTitleClick() {
  if (clickTimer) {
    clearTimeout(clickTimer);
    clickTimer = null;
    toggleMaximize();
  } else {
    clickTimer = setTimeout(() => {
      clickTimer = null;
    }, 300);
  }
}

function handleKeyInput(e: Event) {
  const input = e.target as HTMLInputElement;
  emit("updateChatKey", input.value || "t");
}
</script>

<template>
  <div class="title-bar" data-tauri-drag-region @dblclick="toggleMaximize">
    <!-- 左侧图标和标题 -->
    <div class="title-bar__left" @click="handleTitleClick">
      <img src="/icons/icon.svg" alt="KILLSAY" class="title-bar__icon" />
      <span class="title-bar__title">KILLSAY</span>
    </div>
    
    <!-- 中间聊天按键 -->
    <div class="title-bar__center">
      <div class="chat-key-badge" @click="isEditingKey = true">
        <span class="key-label">聊天键:</span>
        <input
          v-if="isEditingKey"
          :value="chatKey || 't'"
          class="key-input"
          maxlength="5"
          @blur="isEditingKey = false"
          @keyup.enter="isEditingKey = false"
          @input="handleKeyInput"
          autofocus
        />
        <span v-else class="key-value">{{ chatKey || 'T' }}</span>
      </div>
    </div>
    
    <!-- 右侧控制按钮 -->
    <div class="title-bar__right">
      <button class="title-bar__btn update-btn" @click="emit('openUpdate')" title="检查更新">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
          <polyline points="7 10 12 15 17 10" />
          <line x1="12" y1="15" x2="12" y2="3" />
        </svg>
      </button>
      
      <button class="title-bar__btn share-btn" @click="emit('openShare')" title="配置分享">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="18" cy="5" r="3" />
          <circle cx="6" cy="12" r="3" />
          <circle cx="18" cy="19" r="3" />
          <line x1="8.59" y1="13.51" x2="15.42" y2="17.49" />
          <line x1="15.41" y1="6.51" x2="8.59" y2="10.49" />
        </svg>
      </button>
      
      <button class="title-bar__btn theme-btn" @click="emit('toggleTheme')" title="切换主题">
        <svg v-if="themeStore.isDark" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="5" />
          <line x1="12" y1="1" x2="12" y2="3" />
          <line x1="12" y1="21" x2="12" y2="23" />
          <line x1="4.22" y1="4.22" x2="5.64" y2="5.64" />
          <line x1="18.36" y1="18.36" x2="19.78" y2="19.78" />
          <line x1="1" y1="12" x2="3" y2="12" />
          <line x1="21" y1="12" x2="23" y2="12" />
          <line x1="4.22" y1="19.78" x2="5.64" y2="18.36" />
          <line x1="18.36" y1="5.64" x2="19.78" y2="4.22" />
        </svg>
        <svg v-else width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />
        </svg>
      </button>
      
      <button class="title-bar__btn color-btn" @click="emit('openColorPicker')" title="主题色">
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

.title-bar__center {
  position: absolute;
  left: 50%;
  transform: translateX(-50%);
}

.chat-key-badge {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  background: rgba(255, 255, 255, 0.06);
  border: 1px solid var(--border-light);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.chat-key-badge:hover {
  background: rgba(255, 255, 255, 0.1);
  border-color: var(--accent-yellow);
}

.key-label {
  font-size: 10px;
  color: var(--text-secondary);
}

.key-value {
  font-size: 12px;
  font-weight: 700;
  font-family: var(--font-mono);
  color: var(--accent-yellow);
  text-transform: uppercase;
  min-width: 16px;
  text-align: center;
}

.key-input {
  width: 24px;
  height: 18px;
  background: transparent;
  border: 1px solid var(--accent-yellow);
  border-radius: 3px;
  color: var(--accent-yellow);
  font-size: 12px;
  font-weight: 700;
  font-family: var(--font-mono);
  text-align: center;
  text-transform: uppercase;
  outline: none;
  padding: 0;
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

.update-btn:hover {
  color: var(--accent-green, #22c55e);
}

.settings-btn:hover {
  color: var(--accent-yellow);
}

.close-btn:hover {
  background: rgba(239, 68, 68, 0.8);
  color: white;
}
</style>
