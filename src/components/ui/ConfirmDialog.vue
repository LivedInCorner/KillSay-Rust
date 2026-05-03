<script setup lang="ts">
const props = defineProps<{
  title: string;
  message: string;
  kind?: "info" | "warning" | "error";
  showCancel?: boolean;
}>();

const emit = defineEmits<{
  confirm: [];
  cancel: [];
}>();
</script>

<template>
  <div class="confirm-overlay" @mousedown.self="emit('cancel')">
    <div class="confirm-panel dg-panel" :class="`confirm-${kind || 'info'}`">
      <div class="confirm-icon">
        <svg v-if="kind === 'warning'" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M10.29 3.86L1.82 18a2 2 0 001.71 3h16.94a2 2 0 001.71-3L13.71 3.86a2 2 0 00-3.42 0z" />
          <line x1="12" y1="9" x2="12" y2="13" />
          <line x1="12" y1="17" x2="12.01" y2="17" />
        </svg>
        <svg v-else-if="kind === 'error'" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10" />
          <line x1="15" y1="9" x2="9" y2="15" />
          <line x1="9" y1="9" x2="15" y2="15" />
        </svg>
        <svg v-else width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10" />
          <line x1="12" y1="16" x2="12" y2="12" />
          <line x1="12" y1="8" x2="12.01" y2="8" />
        </svg>
      </div>
      
      <div class="confirm-content">
        <h3 class="confirm-title">{{ title }}</h3>
        <p class="confirm-message">{{ message }}</p>
      </div>
      
      <div class="confirm-actions">
        <button class="dg-btn dg-btn--primary" @click="emit('confirm')">
          {{ showCancel === false ? '确定' : '确认' }}
        </button>
        <button
          v-if="showCancel !== false"
          class="dg-btn dg-btn--secondary"
          @click="emit('cancel')"
        >
          取消
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.confirm-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 300;
  backdrop-filter: blur(4px);
  animation: fadeIn 0.15s ease;
}

.confirm-panel {
  width: 360px;
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: var(--spacing-lg);
  animation: slideUp 0.2s ease;
}

.confirm-icon {
  margin-bottom: var(--spacing-md);
}

.confirm-warning .confirm-icon {
  color: var(--accent-yellow);
}

.confirm-error .confirm-icon {
  color: var(--accent-red);
}

.confirm-info .confirm-icon {
  color: var(--accent-blue);
}

.confirm-content {
  margin-bottom: var(--spacing-lg);
}

.confirm-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 var(--spacing-sm) 0;
}

.confirm-message {
  font-size: 13px;
  color: var(--text-secondary);
  margin: 0;
  line-height: 1.5;
}

.confirm-actions {
  display: flex;
  gap: 8px;
  width: 100%;
}

.confirm-actions .dg-btn {
  flex: 1;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(10px) scale(0.98);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}
</style>
