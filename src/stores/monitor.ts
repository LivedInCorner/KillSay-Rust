import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { MonitorParams, MonitorStatus, MonitorEvent } from "@/types";

export const useMonitorStore = defineStore("monitor", () => {
  // 状态
  const status = ref<MonitorStatus>({
    is_running: false,
    kills: 0,
    deaths: 0,
  });
  const error = ref<string | null>(null);
  const lastEvent = ref<MonitorEvent | null>(null);
  const antiSnipePlayer = ref<string | null>(null);

  // 计算属性
  const isRunning = computed(() => status.value.is_running);
  const kills = computed(() => status.value.kills);
  const deaths = computed(() => status.value.deaths);
  const kdr = computed(() => {
    if (status.value.deaths === 0) return status.value.kills;
    return (status.value.kills / status.value.deaths).toFixed(2);
  });

  // 监听事件
  let unlisten: (() => void) | null = null;

  function setupEventListener() {
    if (unlisten) return;
    
    listen<MonitorEvent>("monitor-event", (event) => {
      const monitorEvent = event.payload;
      lastEvent.value = monitorEvent;

      switch (monitorEvent.type) {
        case "Kill":
          status.value.kills = monitorEvent.kills;
          status.value.deaths = monitorEvent.deaths;
          break;
        case "Death":
          status.value.kills = monitorEvent.kills;
          status.value.deaths = monitorEvent.deaths;
          break;
        case "AntiSnipe":
          antiSnipePlayer.value = monitorEvent.player;
          // 3秒后清除
          setTimeout(() => {
            antiSnipePlayer.value = null;
          }, 3000);
          break;
        case "Error":
          error.value = monitorEvent.message;
          break;
        case "StatusChanged":
          status.value.is_running = monitorEvent.is_running;
          break;
      }
    }).then((fn) => {
      unlisten = fn;
    });
  }

  // 开始监控
  async function startMonitoring(params: MonitorParams) {
    try {
      error.value = null;
      await invoke("start_monitoring", { params });
      status.value.is_running = true;
      status.value.kills = 0;
      status.value.deaths = 0;
      setupEventListener();
    } catch (e) {
      error.value = String(e);
      throw e;
    }
  }

  // 停止监控
  async function stopMonitoring() {
    try {
      error.value = null;
      await invoke("stop_monitoring");
      status.value.is_running = false;
    } catch (e) {
      error.value = String(e);
      throw e;
    }
  }

  // 获取状态
  async function fetchStatus() {
    try {
      const s = await invoke<MonitorStatus>("get_monitor_status");
      status.value = s;
    } catch (e) {
      error.value = String(e);
    }
  }

  // 清理事件监听
  function cleanup() {
    if (unlisten) {
      unlisten();
      unlisten = null;
    }
  }

  return {
    status,
    error,
    lastEvent,
    antiSnipePlayer,
    isRunning,
    kills,
    deaths,
    kdr,
    startMonitoring,
    stopMonitoring,
    fetchStatus,
    setupEventListener,
    cleanup,
  };
});
