<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { useConfigStore } from "./stores/config";
import { useMonitorStore } from "./stores/monitor";
import { useThemeStore } from "./stores/theme";
import TitleBar from "./components/layout/TitleBar.vue";
import Home from "./views/Home.vue";
import Settings from "./views/Settings.vue";
import ColorPicker from "./components/ui/ColorPicker.vue";
import ShareWindow from "./components/features/ShareWindow.vue";
import StatsOverlay from "./views/StatsOverlay.vue";

const configStore = useConfigStore();
const monitorStore = useMonitorStore();
const themeStore = useThemeStore();
const showSettings = ref(false);
const showColorPicker = ref(false);
const showShare = ref(false);

// Check if this is the stats window
const isStatsWindow = computed(() => {
  return window.location.hash === "#/stats";
});

onMounted(async () => {
  // Only initialize config stores for main window
  if (!isStatsWindow.value) {
    await configStore.fetchCurrentConfig();
    await configStore.fetchConfigList();
    await configStore.fetchSettings();
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
</style>
