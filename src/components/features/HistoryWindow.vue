<script setup lang="ts">
import { ref, computed } from "vue";
import GeoElement from "@/components/ui/GeoElement.vue";
import GeoLine from "@/components/ui/GeoLine.vue";
import type { HistoryItem } from "@/types";

const props = defineProps<{
  title: string;
  items: HistoryItem[];
  type: "user" | "path";
  currentValue: string;
}>();

const emit = defineEmits<{
  select: [value: string];
  setDefault: [value: string];
  remove: [value: string];
  close: [];
}>();

const searchQuery = ref("");

const filteredItems = computed(() => {
  if (!searchQuery.value) return props.items;
  const query = searchQuery.value.toLowerCase();
  return props.items.filter(item => 
    item.value.toLowerCase().includes(query)
  );
});

function handleSelect(value: string) {
  emit("select", value);
  emit("close");
}
</script>

<template>
  <div class="history-overlay" @mousedown.self="emit('close')">
    <div class="history-panel dg-panel">
      <!-- 标题栏 -->
      <div class="history-header">
        <div class="history-title-area">
          <GeoElement type="hexagon" :size="18" color="var(--accent-yellow)" />
          <h2 class="history-title">{{ title }}</h2>
        </div>
        <button class="dg-btn dg-btn--secondary dg-btn--sm" @click="emit('close')">
          关闭
        </button>
      </div>
      
      <!-- 搜索框 -->
      <div class="history-search">
        <input
          v-model="searchQuery"
          class="dg-input"
          placeholder="搜索..."
        />
      </div>
      
      <!-- 当前值 -->
      <div class="current-value" v-if="currentValue">
        <span class="current-label">当前:</span>
        <span class="current-text">{{ currentValue }}</span>
      </div>
      
      <GeoLine direction="horizontal" :length="100" />
      
      <!-- 历史列表 -->
      <div class="history-list">
        <div v-if="filteredItems.length === 0" class="history-empty">
          <GeoElement type="circle" :size="24" color="var(--text-secondary)" :opacity="0.3" />
          <span>暂无历史记录</span>
        </div>
        
        <div
          v-for="item in filteredItems"
          :key="item.value"
          class="history-item"
          :class="{ 'history-item--active': item.value === currentValue }"
        >
          <div class="history-item-main" @click="handleSelect(item.value)">
            <div class="history-item-info">
              <span class="history-item-value" :class="{ 'path-value': type === 'path' }">
                {{ item.value }}
              </span>
              <span v-if="item.is_default" class="history-item-badge">默认</span>
            </div>
          </div>
          
          <div class="history-item-actions">
            <button
              v-if="!item.is_default"
              class="dg-btn dg-btn--secondary dg-btn--sm"
              @click="emit('setDefault', item.value)"
              title="设为默认"
            >
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" />
              </svg>
            </button>
            <button
              class="dg-btn dg-btn--danger dg-btn--sm"
              @click="emit('remove', item.value)"
              title="删除"
            >
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2" />
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.history-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 150;
  backdrop-filter: blur(4px);
  animation: fadeIn 0.2s ease;
}

.history-panel {
  width: 480px;
  max-height: 70vh;
  display: flex;
  flex-direction: column;
  animation: slideUp 0.25s ease;
}

.history-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-md);
}

.history-title-area {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.history-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.history-search {
  margin-bottom: var(--spacing-sm);
}

.current-value {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-xs) var(--spacing-sm);
  background: rgba(249, 211, 66, 0.1);
  border-radius: var(--radius-small);
  margin-bottom: var(--spacing-sm);
}

.current-label {
  font-size: 11px;
  color: var(--text-secondary);
  text-transform: uppercase;
}

.current-text {
  font-size: 13px;
  color: var(--accent-yellow);
  font-family: var(--font-mono);
}

.history-list {
  flex: 1;
  overflow-y: auto;
  min-height: 200px;
  max-height: 400px;
  margin-top: var(--spacing-sm);
}

.history-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-xl);
  color: var(--text-secondary);
  font-size: 13px;
}

.history-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-sm);
  background: rgba(0, 0, 0, 0.2);
  border-radius: var(--radius-small);
  margin-bottom: 4px;
  transition: all 0.15s ease;
  border: 1px solid transparent;
}

.history-item:hover {
  background: rgba(249, 211, 66, 0.05);
  border-color: var(--border-light);
}

.history-item--active {
  border-color: rgba(249, 211, 66, 0.3);
  background: rgba(249, 211, 66, 0.05);
}

.history-item-main {
  flex: 1;
  cursor: pointer;
  min-width: 0;
}

.history-item-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.history-item-value {
  font-size: 13px;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.history-item-value.path-value {
  font-family: var(--font-mono);
  font-size: 11px;
}

.history-item-badge {
  font-size: 10px;
  padding: 2px 6px;
  background: var(--accent-yellow);
  color: var(--bg-primary);
  border-radius: 3px;
  font-weight: 600;
  flex-shrink: 0;
}

.history-item-actions {
  display: flex;
  gap: 4px;
  flex-shrink: 0;
  margin-left: var(--spacing-sm);
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
