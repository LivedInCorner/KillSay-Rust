import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { Config, ConfigInfo, AppSettings } from "@/types";

export const useConfigStore = defineStore("config", () => {
  // 状态
  const configList = ref<ConfigInfo[]>([]);
  const currentConfig = ref<Config | null>(null);
  const currentConfigName = ref<string>("");
  const currentConfigPath = ref<string>("");
  const settings = ref<AppSettings | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // 计算属性
  const hasConfigs = computed(() => configList.value.length > 0);
  const userHistory = computed(() => settings.value?.user_history.items || []);
  const pathHistory = computed(() => settings.value?.path_history.items || []);
  const defaultUser = computed(() => 
    userHistory.value.find(item => item.is_default)?.value || ""
  );
  const defaultPath = computed(() => 
    pathHistory.value.find(item => item.is_default)?.value || ""
  );

  // 获取配置列表
  async function fetchConfigList() {
    try {
      loading.value = true;
      error.value = null;
      configList.value = await invoke<ConfigInfo[]>("get_config_list");
    } catch (e) {
      error.value = String(e);
    } finally {
      loading.value = false;
    }
  }

  // 加载配置
  async function loadConfig(path: string) {
    try {
      loading.value = true;
      error.value = null;
      const config = await invoke<Config>("load_config", { path });
      currentConfig.value = config;
      currentConfigPath.value = path;
      
      // 从路径中提取配置名称
      const parts = path.replace(/\\/g, "/").split("/");
      const fileName = parts[parts.length - 1];
      currentConfigName.value = fileName.replace(".json", "");
      
      return config;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  // 保存配置
  async function saveConfig(path: string, config: Config) {
    try {
      loading.value = true;
      error.value = null;
      await invoke("save_config", { path, config });
      
      // 如果是当前配置，更新本地状态
      if (currentConfig.value) {
        currentConfig.value = config;
      }
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  // 创建配置
  async function createConfig(name: string) {
    try {
      loading.value = true;
      error.value = null;
      const config = await invoke<Config>("create_config", { name });
      await fetchConfigList();
      return config;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  // 删除配置
  async function deleteConfig(path: string) {
    try {
      loading.value = true;
      error.value = null;
      await invoke("delete_config", { path });
      await fetchConfigList();
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  // 设置默认配置
  async function setDefaultConfig(path: string) {
    try {
      error.value = null;
      await invoke("set_default_config", { path });
      await fetchConfigList();
    } catch (e) {
      error.value = String(e);
      throw e;
    }
  }

  // 获取当前配置
  async function fetchCurrentConfig() {
    try {
      const config = await invoke<Config | null>("get_current_config");
      if (config) {
        currentConfig.value = config;
        
        // Find the config name from the list
        const configPath = await invoke<string | null>("get_current_config_path");
        if (configPath) {
          currentConfigPath.value = configPath;
          const parts = configPath.replace(/\\/g, "/").split("/");
          const fileName = parts[parts.length - 1];
          currentConfigName.value = fileName.replace(".json", "");
        }
      }
    } catch (e) {
      error.value = String(e);
    }
  }

  // 获取设置
  async function fetchSettings() {
    try {
      settings.value = await invoke<AppSettings>("get_settings");
    } catch (e) {
      error.value = String(e);
    }
  }

  // 添加用户历史
  async function addUserHistory(user: string) {
    try {
      await invoke("add_user_history", { user });
      await fetchSettings();
    } catch (e) {
      error.value = String(e);
    }
  }

  // 设置默认用户
  async function setDefaultUser(user: string) {
    try {
      await invoke("set_default_user", { user });
      await fetchSettings();
    } catch (e) {
      error.value = String(e);
    }
  }

  // 删除用户历史
  async function removeUserHistory(user: string) {
    try {
      await invoke("remove_user_history", { user });
      await fetchSettings();
    } catch (e) {
      error.value = String(e);
    }
  }

  // 添加路径历史
  async function addPathHistory(path: string) {
    try {
      await invoke("add_path_history", { path });
      await fetchSettings();
    } catch (e) {
      error.value = String(e);
    }
  }

  // 设置默认路径
  async function setDefaultPath(path: string) {
    try {
      await invoke("set_default_path", { path });
      await fetchSettings();
    } catch (e) {
      error.value = String(e);
    }
  }

  // 删除路径历史
  async function removePathHistory(path: string) {
    try {
      await invoke("remove_path_history", { path });
      await fetchSettings();
    } catch (e) {
      error.value = String(e);
    }
  }

  return {
    configList,
    currentConfig,
    currentConfigName,
    currentConfigPath,
    settings,
    loading,
    error,
    hasConfigs,
    userHistory,
    pathHistory,
    defaultUser,
    defaultPath,
    fetchConfigList,
    loadConfig,
    saveConfig,
    createConfig,
    deleteConfig,
    setDefaultConfig,
    fetchCurrentConfig,
    fetchSettings,
    addUserHistory,
    setDefaultUser,
    removeUserHistory,
    addPathHistory,
    setDefaultPath,
    removePathHistory,
  };
});
