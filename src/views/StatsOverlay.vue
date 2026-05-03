<script setup lang="ts">
import { computed, onMounted, onUnmounted } from "vue";
import { useMonitorStore } from "@/stores/monitor";

const monitorStore = useMonitorStore();

const kdr = computed(() => {
  if (monitorStore.deaths === 0) return monitorStore.kills.toFixed(1);
  return (monitorStore.kills / monitorStore.deaths).toFixed(1);
});

onMounted(() => {
  monitorStore.setupEventListener();
});

onUnmounted(() => {
  monitorStore.cleanup();
});
</script>

<template>
  <div class="stats-overlay">
    <div class="stats-container">
      <div class="stat-item kills">
        <span class="stat-value">{{ monitorStore.kills }}</span>
        <span class="stat-label">击杀</span>
      </div>
      
      <div class="stat-divider"></div>
      
      <div class="stat-item deaths">
        <span class="stat-value">{{ monitorStore.deaths }}</span>
        <span class="stat-label">死亡</span>
      </div>
      
      <div class="stat-divider"></div>
      
      <div class="stat-item kdr">
        <span class="stat-value">{{ kdr }}</span>
        <span class="stat-label">K/D</span>
      </div>
    </div>
    
    <div class="status-dot" :class="{ active: monitorStore.isRunning }"></div>
  </div>
</template>

<style scoped>
.stats-overlay {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #1A1D22;
  border: 1px solid rgba(249, 211, 66, 0.3);
  border-radius: 8px;
  position: relative;
}

.stats-loading {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #1A1D22;
  border: 1px solid rgba(249, 211, 66, 0.3);
  border-radius: 8px;
  color: rgba(255, 255, 255, 0.5);
  font-size: 12px;
}

.stats-container {
  display: flex;
  align-items: center;
  gap: 20px;
  padding: 0 24px;
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  min-width: 60px;
}

.stat-value {
  font-size: 28px;
  font-weight: 700;
  font-family: "JetBrains Mono", "Consolas", monospace;
  line-height: 1;
}

.stat-label {
  font-size: 10px;
  color: rgba(255, 255, 255, 0.5);
  text-transform: uppercase;
  letter-spacing: 1px;
  margin-top: 4px;
}

.kills .stat-value {
  color: #F9D342;
}

.deaths .stat-value {
  color: #EF4444;
}

.kdr .stat-value {
  color: #60A5FA;
}

.stat-divider {
  width: 1px;
  height: 32px;
  background: rgba(255, 255, 255, 0.15);
}

.status-dot {
  position: absolute;
  top: 8px;
  right: 8px;
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.3);
}

.status-dot.active {
  background: #22c55e;
  box-shadow: 0 0 8px rgba(34, 197, 94, 0.5);
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}
</style>
