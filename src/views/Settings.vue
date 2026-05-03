<script setup lang="ts">
import GeoElement from "@/components/ui/GeoElement.vue";
import GeoLine from "@/components/ui/GeoLine.vue";

const emit = defineEmits<{
  close: [];
}>();

const appInfo = {
  name: "KILLSAY",
  version: "1.0.0",
  team: "Hurricane Centre",
  copyright: "Copyright 2025-2026 Hurricane Centre",
  description: "Minecraft 击杀喊话脚本",
};
</script>

<template>
  <div class="settings-overlay" @mousedown.self="emit('close')">
    <div class="settings-panel dg-panel">
      <!-- 标题栏 -->
      <div class="settings-header">
        <div class="settings-title-area">
          <GeoElement type="hexagon" :size="20" color="var(--accent-yellow)" />
          <h2 class="dg-title">设置</h2>
        </div>
        <button class="dg-btn dg-btn--secondary" @click="emit('close')">
          <GeoElement type="cross" :size="10" color="var(--text-secondary)" />
          关闭
        </button>
      </div>
      
      <!-- 版本信息 -->
      <div class="version-section">
        <div class="section-header">
          <GeoElement type="diamond" :size="12" color="var(--accent-yellow)" />
          <span class="section-title">VERSION INFO</span>
          <GeoLine direction="horizontal" :length="100" />
        </div>
        
        <div class="version-card dg-card">
          <!-- Logo 区域 -->
          <div class="logo-area">
            <div class="logo-decoration">
              <img src="/icons/icon.svg" alt="KILLSAY Logo" class="logo-icon" />
            </div>
            <div class="logo-text">
              <h1 class="app-name">{{ appInfo.name }}</h1>
              <span class="app-version">v{{ appInfo.version }}</span>
            </div>
          </div>
          
          <GeoLine direction="horizontal" :length="100" />
          
          <!-- 详细信息 -->
          <div class="info-grid">
            <div class="info-item">
              <span class="info-label">软件名称</span>
              <span class="info-value">{{ appInfo.name }}</span>
            </div>
            <div class="info-item">
              <span class="info-label">版本号</span>
              <span class="info-value mono">{{ appInfo.version }}</span>
            </div>
            <div class="info-item">
              <span class="info-label">开发团队</span>
              <span class="info-value highlight">{{ appInfo.team }}</span>
            </div>
            <div class="info-item">
              <span class="info-label">功能说明</span>
              <span class="info-value">{{ appInfo.description }}</span>
            </div>
          </div>
          
          <GeoLine direction="horizontal" :length="100" />
          
          <!-- 版权信息 -->
          <div class="copyright-area">
            <GeoElement type="circle" :size="8" color="var(--accent-yellow)" :opacity="0.6" />
            <span class="copyright-text">{{ appInfo.copyright }}</span>
          </div>
        </div>
      </div>
      
      <!-- 关于 -->
      <div class="about-section">
        <div class="about-card">
          <p class="about-text">
            KILLSAY 是一款 Minecraft 击杀喊话工具，支持自定义击杀消息、
            击杀统计和反狙击功能。
          </p>
          <div class="about-decoration">
            <GeoElement type="triangle" :size="10" color="var(--accent-blue)" :opacity="0.5" />
            <GeoLine direction="horizontal" :length="60" />
            <GeoElement type="triangle" :size="10" color="var(--accent-blue)" :opacity="0.5" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-overlay {
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

.settings-panel {
  width: 500px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
  animation: slideUp 0.25s ease;
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-lg);
}

.settings-title-area {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.settings-title-area h2 {
  margin-bottom: 0;
}

/* 版本信息区域 */
.version-section {
  margin-bottom: var(--spacing-lg);
}

.section-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-md);
}

.section-title {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
  letter-spacing: 2px;
  white-space: nowrap;
}

.version-card {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

/* Logo 区域 */
.logo-area {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-md) 0;
}

.logo-decoration {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 64px;
  height: 64px;
  background: rgba(249, 211, 66, 0.1);
  border-radius: var(--radius-card);
  border: 1px solid rgba(249, 211, 66, 0.2);
  overflow: hidden;
}

.logo-icon {
  width: 48px;
  height: 48px;
  object-fit: contain;
}

.logo-text {
  display: flex;
  flex-direction: column;
}

.app-name {
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
  letter-spacing: 4px;
  line-height: 1;
}

.app-version {
  font-size: 14px;
  font-family: var(--font-mono);
  color: var(--accent-yellow);
  margin-top: 4px;
}

/* 信息网格 */
.info-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-md);
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.info-label {
  font-size: 11px;
  font-weight: 500;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 1px;
}

.info-value {
  font-size: 14px;
  color: var(--text-primary);
  font-weight: 500;
}

.info-value.mono {
  font-family: var(--font-mono);
  color: var(--accent-yellow);
}

.info-value.highlight {
  color: var(--accent-blue);
}

/* 版权区域 */
.copyright-area {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) 0;
}

.copyright-text {
  font-size: 12px;
  color: var(--text-secondary);
  font-family: var(--font-mono);
}

/* 关于区域 */
.about-section {
  margin-top: auto;
}

.about-card {
  background: rgba(0, 0, 0, 0.2);
  border-radius: var(--radius-small);
  padding: var(--spacing-md);
  text-align: center;
}

.about-text {
  font-size: 13px;
  color: var(--text-secondary);
  line-height: 1.6;
  margin-bottom: var(--spacing-sm);
}

.about-decoration {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-sm);
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
