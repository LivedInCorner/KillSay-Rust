<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useConfigStore } from "@/stores/config";
import { useMonitorStore } from "@/stores/monitor";
import { open } from "@tauri-apps/plugin-dialog";
import type { MonitorParams } from "@/types";
import ConfigEditor from "@/components/features/ConfigEditor.vue";
import HistoryWindow from "@/components/features/HistoryWindow.vue";
import ConfirmDialog from "@/components/ui/ConfirmDialog.vue";
import GeoElement from "@/components/ui/GeoElement.vue";
import GeoLine from "@/components/ui/GeoLine.vue";
import RingStat from "@/components/ui/RingStat.vue";

const configStore = useConfigStore();
const monitorStore = useMonitorStore();

// 表单状态
const user = ref("");
const windowName = ref("布吉岛");
const logFilePath = ref("D:\\MCLDownload\\Game\\.minecraft\\logs\\latest.log");
const antiSnipeEnabled = ref(false);

// UI状态
const showConfigEditor = ref(false);
const showConfigList = ref(false);
const showNewConfigDialog = ref(false);
const showUserHistory = ref(false);
const showPathHistory = ref(false);
const newConfigName = ref("");

// Confirm dialog state
const showConfirm = ref(false);
const confirmTitle = ref("");
const confirmMessage = ref("");
const confirmKind = ref<"info" | "warning" | "error">("warning");
const confirmShowCancel = ref(true);
const confirmCallback = ref<(() => void) | null>(null);

// 计算属性
const kdr = computed(() => monitorStore.kdr);
const isRunning = computed(() => monitorStore.isRunning);
const statusClass = computed(() => ({
  "dg-status--active": isRunning.value,
}));

// 检查是否是当前加载的配置
function isCurrentConfig(path: string): boolean {
  return configStore.currentConfigPath === path;
}

onMounted(() => {
  // 加载默认值
  if (configStore.defaultUser) {
    user.value = configStore.defaultUser;
  }
  if (configStore.defaultPath) {
    logFilePath.value = configStore.defaultPath;
  }
});

// Show custom confirm dialog
function showConfirmDialog(
  title: string,
  message: string,
  kind: "info" | "warning" | "error",
  callback: () => void,
  showCancel = true
) {
  confirmTitle.value = title;
  confirmMessage.value = message;
  confirmKind.value = kind;
  confirmShowCancel.value = showCancel;
  confirmCallback.value = callback;
  showConfirm.value = true;
}

// Handle confirm
function handleConfirm() {
  if (confirmCallback.value) {
    confirmCallback.value();
  }
  showConfirm.value = false;
}

// 开始/停止监控
async function toggleMonitoring() {
  if (isRunning.value) {
    await monitorStore.stopMonitoring();
  } else {
    if (!configStore.currentConfig) {
      showConfirmDialog("提示", "请先加载配置", "warning", () => {}, false);
      return;
    }
    
    if (!user.value.trim()) {
      showConfirmDialog("提示", "请输入游戏名称", "warning", () => {}, false);
      return;
    }
    
    // 保存到历史
    await configStore.addUserHistory(user.value.trim());
    await configStore.addPathHistory(logFilePath.value.trim());
    
    const params: MonitorParams = {
      user: user.value,
      window_name: windowName.value,
      log_file_path: logFilePath.value,
      ...configStore.currentConfig,
      anti_snipe_enabled: antiSnipeEnabled.value,
    };
    
    await monitorStore.startMonitoring(params);
  }
}

// 选择日志文件
async function selectLogFile() {
  try {
    const selected = await open({
      title: "选择 Minecraft 日志文件",
      defaultPath: logFilePath.value,
      filters: [
        {
          name: "日志文件",
          extensions: ["log", "txt"],
        },
        {
          name: "所有文件",
          extensions: ["*"],
        },
      ],
    });
    
    if (selected) {
      logFilePath.value = selected as string;
    }
  } catch (e) {
    console.error("选择文件失败:", e);
  }
}

// 加载配置
async function loadConfig(path: string) {
  await configStore.loadConfig(path);
  showConfigList.value = false;
}

// 创建新配置
async function createNewConfig() {
  if (!newConfigName.value.trim()) {
    showConfirmDialog("提示", "请输入配置名称", "warning", () => {}, false);
    return;
  }
  
  try {
    await configStore.createConfig(newConfigName.value.trim());
    newConfigName.value = "";
    showNewConfigDialog.value = false;
    showConfirmDialog("成功", "配置创建成功", "info", () => {}, false);
  } catch (e) {
    showConfirmDialog("错误", `创建失败: ${e}`, "error", () => {}, false);
  }
}

// 删除配置
async function deleteConfig(path: string) {
  showConfirmDialog(
    "确认删除",
    "确定要删除此配置吗？",
    "warning",
    async () => {
      try {
        await configStore.deleteConfig(path);
        showConfirmDialog("成功", "配置已删除", "info", () => {}, false);
      } catch (e) {
        showConfirmDialog("错误", `删除失败: ${e}`, "error", () => {}, false);
      }
    }
  );
}

// 设置默认配置
async function setDefaultConfig(path: string) {
  try {
    await configStore.setDefaultConfig(path);
    showConfirmDialog("成功", "已设置为默认配置", "info", () => {}, false);
  } catch (e) {
    showConfirmDialog("错误", `设置失败: ${e}`, "error", () => {}, false);
  }
}

// 选择用户历史
function selectUser(value: string) {
  user.value = value;
  showUserHistory.value = false;
}

// 设置默认用户
async function setDefaultUser(value: string) {
  try {
    await configStore.setDefaultUser(value);
    showConfirmDialog("成功", "已设置为默认游戏名称", "info", () => {}, false);
  } catch (e) {
    showConfirmDialog("错误", `设置失败: ${e}`, "error", () => {}, false);
  }
}

// 删除用户历史
function removeUser(value: string) {
  showConfirmDialog(
    "确认删除",
    "确定要删除此历史记录吗？",
    "warning",
    async () => {
      await configStore.removeUserHistory(value);
    }
  );
}

// 选择路径历史
function selectPath(value: string) {
  logFilePath.value = value;
  showPathHistory.value = false;
}

// 设置默认路径
async function setDefaultPath(value: string) {
  try {
    await configStore.setDefaultPath(value);
    showConfirmDialog("成功", "已设置为默认路径", "info", () => {}, false);
  } catch (e) {
    showConfirmDialog("错误", `设置失败: ${e}`, "error", () => {}, false);
  }
}

// 删除路径历史
function removePath(value: string) {
  showConfirmDialog(
    "确认删除",
    "确定要删除此历史记录吗？",
    "warning",
    async () => {
      await configStore.removePathHistory(value);
    }
  );
}

// 切换悬浮统计窗口
async function toggleStatsWindow() {
  try {
    await invoke("toggle_stats_window");
  } catch (e) {
    console.error("切换统计窗口失败:", e);
  }
}
</script>

<template>
  <div class="home-view">
    <!-- 背景装饰 -->
    <div class="bg-decoration">
      <!-- 网格 -->
      <div class="grid-overlay"></div>
      
      <!-- 放射状线条 -->
      <div class="radial-lines">
        <div v-for="i in 12" :key="i" class="radial-line" :style="{ transform: `rotate(${i * 30}deg)` }"></div>
      </div>
      
      <!-- 漂浮几何元素 -->
      <div class="floating-geo geo-1">
        <GeoElement type="hexagon" :size="60" color="var(--accent-yellow)" :opacity="0.1" />
      </div>
      <div class="floating-geo geo-2">
        <GeoElement type="triangle" :size="40" color="var(--accent-blue)" :opacity="0.08" />
      </div>
      <div class="floating-geo geo-3">
        <GeoElement type="circle" :size="80" color="var(--accent-yellow)" :opacity="0.05" />
      </div>
      <div class="floating-geo geo-4">
        <GeoElement type="diamond" :size="35" color="var(--accent-yellow)" :opacity="0.12" />
      </div>
    </div>
    
    <!-- 主布局 -->
    <div class="main-layout">
      <!-- 左侧控制面板 -->
      <div class="control-panel">
        <!-- 标题区 -->
        <div class="header-section">
          <div class="header-decoration">
            <GeoElement type="hexagon" :size="16" color="var(--accent-yellow)" :opacity="0.8" />
            <GeoLine direction="horizontal" :length="100" :animated="true" />
            <GeoElement type="hexagon" :size="16" color="var(--accent-yellow)" :opacity="0.8" />
          </div>
          
          <div class="title-area">
            <h1 class="main-title">
              <span class="title-text">KILLSAY</span>
              <span class="title-version">v1.0</span>
            </h1>
            <p class="subtitle">MINECRAFT KILL SAY SCRIPT</p>
          </div>
          
          <div class="header-decoration bottom">
            <GeoElement type="triangle" :size="10" color="var(--accent-yellow)" :opacity="0.6" />
            <GeoLine direction="horizontal" :length="100" />
            <GeoElement type="triangle" :size="10" color="var(--accent-yellow)" :opacity="0.6" />
          </div>
        </div>
        
        <!-- 配置区 -->
        <div class="config-section">
          <div class="section-header">
            <GeoElement type="diamond" :size="12" color="var(--accent-yellow)" />
            <span class="section-title">CONFIGURATION</span>
            <GeoLine direction="horizontal" :length="100" />
          </div>
          
          <div class="config-card dg-card">
            <div class="config-info">
              <span class="config-label">当前配置</span>
              <span class="config-name">{{ configStore.currentConfigName || "未加载" }}</span>
            </div>
            
            <div class="config-actions">
              <button class="dg-btn dg-btn--secondary" @click="showConfigEditor = true">
                <GeoElement type="cross" :size="10" color="var(--text-secondary)" />
                编辑
              </button>
              <button class="dg-btn dg-btn--secondary" @click="showNewConfigDialog = true">
                <GeoElement type="hexagon" :size="10" color="var(--text-secondary)" />
                新建
              </button>
              <button class="dg-btn dg-btn--secondary" @click="showConfigList = !showConfigList">
                <GeoElement type="triangle" :size="10" color="var(--text-secondary)" />
                管理
              </button>
            </div>
          </div>
          
        </div>
        
        <!-- 输入区 -->
        <div class="input-section">
          <div class="section-header">
            <GeoElement type="diamond" :size="12" color="var(--accent-yellow)" />
            <span class="section-title">PARAMETERS</span>
            <GeoLine direction="horizontal" :length="100" />
          </div>
          
          <div class="input-group">
            <label class="dg-label">
              <GeoElement type="hexagon" :size="8" color="var(--accent-yellow)" :opacity="0.6" />
              游戏名称
            </label>
            <div class="input-with-btn">
              <input
                v-model="user"
                class="dg-input"
                placeholder="输入你的游戏名称"
              />
              <button class="dg-btn dg-btn--secondary" @click="showUserHistory = true">
                <GeoElement type="triangle" :size="10" color="var(--text-secondary)" />
                历史
              </button>
            </div>
          </div>
          
          <div class="input-group">
            <label class="dg-label">
              <GeoElement type="hexagon" :size="8" color="var(--accent-yellow)" :opacity="0.6" />
              窗口名称
            </label>
            <input
              v-model="windowName"
              class="dg-input"
              placeholder="游戏窗口标题"
            />
          </div>
          
          <div class="input-group">
            <label class="dg-label">
              <GeoElement type="hexagon" :size="8" color="var(--accent-yellow)" :opacity="0.6" />
              日志文件路径
            </label>
            <div class="input-with-btn">
              <input
                v-model="logFilePath"
                class="dg-input"
                placeholder="Minecraft 日志文件路径"
              />
              <button class="dg-btn dg-btn--secondary" @click="showPathHistory = true">
                <GeoElement type="triangle" :size="10" color="var(--text-secondary)" />
                历史
              </button>
              <button class="dg-btn dg-btn--secondary" @click="selectLogFile">
                <GeoElement type="circle" :size="10" color="var(--text-secondary)" />
                浏览
              </button>
            </div>
          </div>
        </div>
        
        <!-- 控制区 -->
        <div class="control-section">
          <div class="section-header">
            <GeoElement type="diamond" :size="12" color="var(--accent-yellow)" />
            <span class="section-title">CONTROL</span>
            <GeoLine direction="horizontal" :length="100" />
          </div>
          
          <!-- 反狙击开关 -->
          <div class="anti-snipe-toggle">
            <button
              class="toggle-btn"
              :class="{ active: antiSnipeEnabled }"
              @click="antiSnipeEnabled = !antiSnipeEnabled"
            >
              <div class="toggle-track">
                <div class="toggle-thumb"></div>
              </div>
              <span class="toggle-label">反狙击</span>
            </button>
            
            <Transition name="fade">
              <span v-if="monitorStore.antiSnipePlayer" class="anti-snipe-alert">
                <GeoElement type="triangle" :size="12" color="var(--accent-red)" />
                检测到: {{ monitorStore.antiSnipePlayer }}
              </span>
            </Transition>
          </div>
          
          <!-- 监控按钮 -->
          <button
            class="monitor-btn"
            :class="{ running: isRunning }"
            @click="toggleMonitoring"
          >
            <div class="btn-inner">
              <div class="btn-icon">
                <GeoElement v-if="!isRunning" type="triangle" :size="20" color="var(--bg-primary)" :opacity="1" />
                <GeoElement v-else type="cross" :size="20" color="var(--text-primary)" :opacity="1" />
              </div>
              <span class="btn-text">{{ isRunning ? "停止监控" : "开始监控" }}</span>
            </div>
            <div class="btn-decoration">
              <div v-for="i in 6" :key="i" class="btn-dot" :style="{ transform: `rotate(${i * 60}deg)` }"></div>
            </div>
          </button>
          
          <!-- 状态指示 -->
          <div class="status-indicator" :class="statusClass">
            <div class="status-dot"></div>
            <span>{{ isRunning ? "监控中..." : "就绪" }}</span>
          </div>
        </div>
        
      </div>
      
      <!-- 中间连接区 -->
      <div class="connection-area">
        <GeoLine direction="vertical" :length="100" :animated="true" />
        <div class="connection-dots">
          <div v-for="i in 5" :key="i" class="connection-dot"></div>
        </div>
      </div>
      
      <!-- 右侧统计面板 -->
      <div class="stats-panel">
        <div class="stats-header">
          <GeoElement type="hexagon" :size="16" color="var(--accent-yellow)" />
          <span class="section-title">STATISTICS</span>
          <GeoLine direction="horizontal" :length="100" />
        </div>
        
        <!-- 环形统计 -->
        <div class="ring-stats">
          <div class="ring-stat-item">
            <RingStat
              :value="monitorStore.kills"
              :max="50"
              label="击杀"
              color="var(--accent-yellow)"
              :size="140"
            />
          </div>
          
          <div class="ring-stat-item">
            <RingStat
              :value="monitorStore.deaths"
              :max="50"
              label="死亡"
              color="var(--accent-red)"
              :size="140"
            />
          </div>
          
          <div class="ring-stat-item kdr-ring">
            <RingStat
              :value="Number(kdr)"
              :max="5"
              label="K/D"
              color="var(--accent-blue)"
              :size="160"
            />
          </div>
        </div>
        
        <!-- 统计详情 -->
        <div class="stats-details">
          <div class="detail-item">
            <GeoElement type="diamond" :size="10" color="var(--accent-yellow)" />
            <span class="detail-label">总击杀</span>
            <span class="detail-value">{{ monitorStore.kills }}</span>
          </div>
          <GeoLine direction="horizontal" :length="100" />
          <div class="detail-item">
            <GeoElement type="diamond" :size="10" color="var(--accent-red)" />
            <span class="detail-label">总死亡</span>
            <span class="detail-value">{{ monitorStore.deaths }}</span>
          </div>
          <GeoLine direction="horizontal" :length="100" />
          <div class="detail-item">
            <GeoElement type="diamond" :size="10" color="var(--accent-blue)" />
            <span class="detail-label">K/D 比</span>
            <span class="detail-value kdr-value">{{ kdr }}</span>
          </div>
        </div>
        
        <!-- 提示信息 -->
        <div class="info-card">
          <div class="info-decoration">
            <GeoElement type="circle" :size="8" color="var(--accent-blue)" />
          </div>
          <p>确保游戏窗口处于活动状态，脚本将自动发送击杀喊话</p>
        </div>
        
        <!-- 悬浮统计窗口按钮 -->
        <button class="dg-btn dg-btn--secondary stats-window-btn" @click="toggleStatsWindow">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="2" y="3" width="20" height="14" rx="2" ry="2" />
            <line x1="8" y1="21" x2="16" y2="21" />
            <line x1="12" y1="17" x2="12" y2="21" />
          </svg>
          悬浮统计
        </button>
      </div>
    </div>
    
    <!-- 新建配置对话框 -->
    <Transition name="fade">
      <div v-if="showNewConfigDialog" class="dialog-overlay" @mousedown.self="showNewConfigDialog = false">
        <div class="dialog dg-panel">
          <div class="dialog-header">
            <GeoElement type="hexagon" :size="20" color="var(--accent-yellow)" />
            <h3>新建配置</h3>
          </div>
          <div class="input-group">
            <label class="dg-label">配置名称</label>
            <input
              v-model="newConfigName"
              class="dg-input"
              placeholder="输入配置名称"
              @keyup.enter="createNewConfig"
            />
          </div>
          <div class="dialog-actions">
            <button class="dg-btn dg-btn--primary" @click="createNewConfig">
              <GeoElement type="hexagon" :size="12" color="var(--bg-primary)" :opacity="1" />
              创建
            </button>
            <button class="dg-btn dg-btn--secondary" @click="showNewConfigDialog = false">取消</button>
          </div>
        </div>
      </div>
    </Transition>
    
    <!-- 配置编辑器 -->
    <ConfigEditor
      v-if="showConfigEditor"
      @close="showConfigEditor = false"
    />
    
    <!-- 配置列表弹窗 -->
    <Transition name="fade">
      <div v-if="showConfigList" class="config-list-overlay" @mousedown.self="showConfigList = false">
        <div class="config-list-panel dg-panel">
          <div class="config-list-header">
            <span>可用配置</span>
            <button class="dg-btn dg-btn--secondary dg-btn--sm" @click="showConfigList = false">关闭</button>
          </div>
          <div class="config-items">
            <div
              v-for="config in configStore.configList"
              :key="config.path"
              class="config-item"
              :class="{
                'config-item--current': isCurrentConfig(config.path),
                'config-item--default': config.is_default && !isCurrentConfig(config.path)
              }"
              @click="loadConfig(config.path)"
            >
              <div class="config-item-info">
                <div class="config-item-name-row">
                  <span class="config-item-name">{{ config.name }}</span>
                  <span v-if="isCurrentConfig(config.path)" class="config-item-badge badge-current">当前</span>
                  <span v-if="config.is_default" class="config-item-badge badge-default">默认</span>
                </div>
                <span class="config-item-time">{{ config.modified }}</span>
              </div>
              <div class="config-item-actions">
                <button
                  v-if="!config.is_default"
                  class="dg-btn dg-btn--secondary dg-btn--sm"
                  @click.stop="setDefaultConfig(config.path)"
                >
                  设为默认
                </button>
                <button
                  v-if="!isCurrentConfig(config.path)"
                  class="dg-btn dg-btn--danger dg-btn--sm"
                  @click.stop="deleteConfig(config.path)"
                >
                  删除
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Transition>
    
    <!-- 用户历史窗口 -->
    <HistoryWindow
      v-if="showUserHistory"
      title="游戏名称历史"
      :items="configStore.userHistory"
      type="user"
      :current-value="user"
      @select="(v) => user = v"
      @setDefault="setDefaultUser"
      @remove="removeUser"
      @close="showUserHistory = false"
    />
    
    <!-- 路径历史窗口 -->
    <HistoryWindow
      v-if="showPathHistory"
      title="日志路径历史"
      :items="configStore.pathHistory"
      type="path"
      :current-value="logFilePath"
      @select="(v) => logFilePath = v"
      @setDefault="setDefaultPath"
      @remove="removePath"
      @close="showPathHistory = false"
    />
    
    <!-- 确认对话框 -->
    <ConfirmDialog
      v-if="showConfirm"
      :title="confirmTitle"
      :message="confirmMessage"
      :kind="confirmKind"
      :showCancel="confirmShowCancel"
      @confirm="handleConfirm"
      @cancel="showConfirm = false"
    />
    
    <!-- 错误提示 (Toast) -->
    <Transition name="toast">
      <div v-if="monitorStore.error" class="error-toast">
        <GeoElement type="triangle" :size="14" color="var(--accent-red)" />
        <span>{{ monitorStore.error }}</span>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.home-view {
  width: 100%;
  height: calc(100vh - 36px);
  margin-top: 36px;
  position: relative;
  overflow: hidden;
}

/* 背景装饰 */
.bg-decoration {
  position: fixed;
  top: 36px;
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none;
  z-index: 0;
}

.grid-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-image: 
    linear-gradient(var(--border-light) 1px, transparent 1px),
    linear-gradient(90deg, var(--border-light) 1px, transparent 1px);
  background-size: 60px 60px;
  opacity: 0.05;
}

.radial-lines {
  position: absolute;
  top: 50%;
  left: 30%;
  width: 0;
  height: 0;
}

.radial-line {
  position: absolute;
  width: 200px;
  height: 1px;
  background: linear-gradient(90deg, var(--accent-yellow), transparent);
  opacity: 0.05;
  transform-origin: 0 0;
}

.floating-geo {
  position: absolute;
  animation: float 6s ease-in-out infinite;
}

.geo-1 {
  top: 10%;
  left: 5%;
  animation-delay: 0s;
}

.geo-2 {
  top: 60%;
  left: 15%;
  animation-delay: 1s;
}

.geo-3 {
  top: 20%;
  right: 10%;
  animation-delay: 2s;
}

.geo-4 {
  bottom: 20%;
  right: 20%;
  animation-delay: 3s;
}

@keyframes float {
  0%, 100% {
    transform: translateY(0) rotate(0deg);
  }
  50% {
    transform: translateY(-10px) rotate(5deg);
  }
}

/* 主布局 */
.main-layout {
  display: flex;
  gap: var(--spacing-lg);
  padding: var(--spacing-lg);
  height: calc(100vh - 36px);
  position: relative;
  z-index: 1;
}

/* 控制面板 */
.control-panel {
  width: 400px;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
  overflow-y: auto;
  overflow-x: hidden;
  padding-right: 4px;
  /* 隐藏滚动条但保留滚动功能 */
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.control-panel::-webkit-scrollbar {
  display: none;
}

/* 标题区 */
.header-section {
  text-align: center;
  padding: var(--spacing-lg);
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  border: 1px solid var(--glass-border);
  border-radius: var(--radius-panel);
}

.header-decoration {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-sm);
}

.header-decoration.bottom {
  margin-bottom: 0;
  margin-top: var(--spacing-sm);
}

.title-area {
  margin: var(--spacing-md) 0;
}

.main-title {
  display: flex;
  align-items: baseline;
  justify-content: center;
  gap: var(--spacing-sm);
}

.title-text {
  font-size: 32px;
  font-weight: 700;
  color: var(--text-primary);
  letter-spacing: 4px;
}

.title-version {
  font-size: 12px;
  color: var(--accent-yellow);
  font-family: var(--font-mono);
}

.subtitle {
  font-size: 11px;
  color: var(--text-secondary);
  letter-spacing: 3px;
  margin-top: var(--spacing-xs);
}

/* 配置区 */
.config-section,
.input-section,
.control-section {
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  border: 1px solid var(--glass-border);
  border-radius: var(--radius-panel);
  padding: var(--spacing-md);
  position: relative;
  z-index: 1;
}

.input-section {
  z-index: 10;
  overflow: visible;
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

.config-card {
  background: rgba(0, 0, 0, 0.2);
  padding: var(--spacing-md);
}

.config-info {
  display: flex;
  flex-direction: column;
  margin-bottom: var(--spacing-sm);
}

.config-label {
  font-size: 11px;
  color: var(--text-secondary);
}

.config-name {
  font-size: 14px;
  color: var(--accent-yellow);
  font-weight: 500;
}

.config-actions {
  display: flex;
  gap: 8px;
}

/* 配置列表弹窗 */
.config-list-overlay {
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
  backdrop-filter: blur(4px);
  animation: fadeIn 0.2s ease;
}

.config-list-panel {
  width: 450px;
  max-height: 70vh;
  display: flex;
  flex-direction: column;
  animation: slideUp 0.25s ease;
}

.config-list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-md);
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.config-items {
  display: flex;
  flex-direction: column;
  gap: 8px;
  overflow-y: auto;
  max-height: 50vh;
}

.config-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-sm);
  background: rgba(0, 0, 0, 0.2);
  border-radius: var(--radius-small);
  cursor: pointer;
  transition: all 0.2s ease;
  border: 1px solid transparent;
}

.config-item:hover {
  background: rgba(249, 211, 66, 0.1);
}

/* 当前配置 - 黄色 */
.config-item--current {
  border-color: rgba(249, 211, 66, 0.4);
  background: rgba(249, 211, 66, 0.08);
}

/* 默认配置 - 紫色 */
.config-item--default {
  border-color: rgba(168, 85, 247, 0.4);
  background: rgba(168, 85, 247, 0.08);
}

.config-item-info {
  display: flex;
  flex-direction: column;
}

.config-item-name-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.config-item-name {
  font-weight: 500;
  color: var(--text-primary);
  font-size: 14px;
}

.config-item-badge {
  font-size: 10px;
  padding: 2px 6px;
  border-radius: 3px;
  font-weight: 600;
}

.badge-current {
  background: var(--accent-yellow);
  color: var(--bg-primary);
}

.badge-default {
  background: #a855f7;
  color: white;
}

.config-item-time {
  font-size: 11px;
  color: var(--text-secondary);
  font-family: var(--font-mono);
}

.config-item-actions {
  display: flex;
  gap: 4px;
}

/* 历史记录下拉 */
.history-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  margin-top: 4px;
  background: var(--bg-panel);
  border: 1px solid var(--border-light);
  border-radius: var(--radius-small);
  max-height: 200px;
  overflow-y: auto;
  z-index: 50;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
}

.history-items {
  display: flex;
  flex-direction: column;
}

.history-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  cursor: pointer;
  transition: all 0.15s ease;
  border-bottom: 1px solid var(--border-light);
}

.history-item:last-child {
  border-bottom: none;
}

.history-item:hover {
  background: rgba(249, 211, 66, 0.1);
}

.history-item-info {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
  min-width: 0;
}

.history-item-value {
  font-size: 13px;
  color: var(--text-primary);
}

.history-item-value.path-value {
  font-family: var(--font-mono);
  font-size: 11px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.history-item-badge {
  font-size: 10px;
  padding: 1px 4px;
  background: var(--accent-yellow);
  color: var(--bg-primary);
  border-radius: 2px;
  font-weight: 600;
}

.history-item-actions {
  display: flex;
  gap: 4px;
  flex-shrink: 0;
}

/* 输入区 */
.input-group {
  display: flex;
  flex-direction: column;
  margin-bottom: var(--spacing-sm);
  position: relative;
}

.input-group:last-child {
  margin-bottom: 0;
}

.input-with-btn {
  display: flex;
  gap: 8px;
}

.input-with-btn .dg-input {
  flex: 1;
}

/* 反狙击开关 */
.anti-snipe-toggle {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  margin-bottom: var(--spacing-md);
}

.toggle-btn {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  background: none;
  border: none;
  cursor: pointer;
  padding: 0;
}

.toggle-track {
  width: 40px;
  height: 20px;
  background: var(--bg-panel);
  border: 1px solid var(--border-light);
  border-radius: 4px;
  position: relative;
  transition: all 0.3s ease;
}

.toggle-btn.active .toggle-track {
  background: var(--accent-yellow);
  border-color: var(--accent-yellow);
}

.toggle-thumb {
  width: 16px;
  height: 16px;
  background: var(--text-primary);
  border-radius: 50%;
  position: absolute;
  top: 1px;
  left: 1px;
  transition: all 0.3s ease;
}

.toggle-btn.active .toggle-thumb {
  transform: translateX(20px);
  background: var(--bg-primary);
}

.toggle-label {
  font-size: 13px;
  color: var(--text-secondary);
}

.anti-snipe-alert {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: var(--accent-yellow);
  animation: pulse 1s infinite;
}

/* 监控按钮 */
.monitor-btn {
  width: 100%;
  height: 60px;
  background: var(--accent-yellow);
  border: none;
  border-radius: var(--radius-card);
  cursor: pointer;
  position: relative;
  overflow: hidden;
  transition: all 0.3s ease;
  margin-bottom: var(--spacing-sm);
}

.monitor-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(249, 211, 66, 0.4);
}

.monitor-btn:active {
  transform: translateY(0);
}

.monitor-btn.running {
  background: var(--accent-red);
}

.monitor-btn.running:hover {
  box-shadow: 0 4px 20px rgba(239, 68, 68, 0.4);
}

.btn-inner {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-sm);
  position: relative;
  z-index: 1;
}

.btn-icon {
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-text {
  font-size: 16px;
  font-weight: 600;
  color: var(--bg-primary);
}

.monitor-btn.running .btn-text {
  color: var(--text-primary);
}

.btn-decoration {
  position: absolute;
  top: 50%;
  left: 50%;
  width: 0;
  height: 0;
}

.btn-dot {
  position: absolute;
  width: 4px;
  height: 4px;
  background: rgba(0, 0, 0, 0.1);
  border-radius: 50%;
  transform-origin: 0 0;
}

/* 状态指示 */
.status-indicator {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  font-size: 13px;
  color: var(--text-secondary);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--text-secondary);
  transition: all 0.3s ease;
}

.status-indicator.dg-status--active .status-dot {
  background: var(--accent-yellow);
  box-shadow: 0 0 8px rgba(249, 211, 66, 0.5);
  animation: pulse 2s infinite;
}

/* 错误提示 Toast */
.error-toast {
  position: fixed;
  top: var(--spacing-lg);
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  background: rgba(239, 68, 68, 0.15);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(239, 68, 68, 0.4);
  border-radius: var(--radius-card);
  color: var(--accent-red);
  font-size: 13px;
  z-index: 200;
  box-shadow: 0 4px 20px rgba(239, 68, 68, 0.2);
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

/* 连接区 */
.connection-area {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 40px;
  position: relative;
}

.connection-dots {
  display: flex;
  flex-direction: column;
  gap: 12px;
  position: absolute;
}

.connection-dot {
  width: 6px;
  height: 6px;
  background: var(--accent-yellow);
  border-radius: 50%;
  opacity: 0.3;
}

/* 统计面板 */
.stats-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.stats-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  border: 1px solid var(--glass-border);
  border-radius: var(--radius-panel);
  padding: var(--spacing-md);
}

.ring-stats {
  display: flex;
  justify-content: center;
  gap: var(--spacing-lg);
  flex-wrap: wrap;
}

.ring-stat-item {
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  border: 1px solid var(--glass-border);
  border-radius: var(--radius-panel);
  padding: var(--spacing-lg);
  display: flex;
  align-items: center;
  justify-content: center;
}

.kdr-ring {
  border-color: rgba(96, 165, 250, 0.2);
}

/* 统计详情 */
.stats-details {
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  border: 1px solid var(--glass-border);
  border-radius: var(--radius-panel);
  padding: var(--spacing-md);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.detail-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-xs) 0;
}

.detail-label {
  flex: 1;
  font-size: 12px;
  color: var(--text-secondary);
}

.detail-value {
  font-size: 16px;
  font-weight: 600;
  font-family: var(--font-mono);
  color: var(--text-primary);
}

.kdr-value {
  color: var(--accent-blue);
}

/* 提示信息 */
.info-card {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-sm);
  padding: var(--spacing-md);
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  border: 1px solid var(--glass-border);
  border-radius: var(--radius-panel);
  font-size: 12px;
  color: var(--text-secondary);
  line-height: 1.6;
  margin-top: auto;
}

.info-decoration {
  margin-top: 2px;
}

/* 悬浮统计窗口按钮 */
.stats-window-btn {
  width: 100%;
  margin-top: var(--spacing-sm);
}

/* 对话框 */
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
  backdrop-filter: blur(4px);
  animation: fadeIn 0.2s ease;
}

.dialog {
  width: 400px;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
  animation: slideUp 0.25s ease;
}

.dialog-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.dialog-header h3 {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  margin-top: var(--spacing-sm);
}

/* 动画 */
@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
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
