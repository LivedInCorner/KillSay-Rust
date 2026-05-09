import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { Config } from "@/types";

export const useAppStore = defineStore("app", () => {
  const chatKey = ref("t");
  const isDarkTheme = ref(true);
  
  // Load chat key from config
  async function loadChatKey() {
    try {
      const config = await invoke<Config | null>("get_current_config");
      if (config?.chat_key) {
        chatKey.value = config.chat_key;
      }
    } catch (e) {
      console.error("Failed to load chat key:", e);
    }
  }
  
  // Update chat key
  function setChatKey(key: string) {
    chatKey.value = key || "t";
  }
  
  // Toggle theme
  function toggleTheme() {
    isDarkTheme.value = !isDarkTheme.value;
    applyTheme();
  }
  
  // Apply theme
  function applyTheme() {
    const root = document.documentElement;
    if (isDarkTheme.value) {
      root.style.setProperty("--bg-primary", "#1A1D22");
      root.style.setProperty("--bg-panel", "#272B33");
      root.style.setProperty("--text-primary", "#E5E7EB");
      root.style.setProperty("--text-secondary", "#9CA3AF");
      root.style.setProperty("--border-light", "#3A3F48");
    } else {
      root.style.setProperty("--bg-primary", "#F5F5F5");
      root.style.setProperty("--bg-panel", "#FFFFFF");
      root.style.setProperty("--text-primary", "#1F2937");
      root.style.setProperty("--text-secondary", "#6B7280");
      root.style.setProperty("--border-light", "#E5E7EB");
    }
    localStorage.setItem("killsay-theme", isDarkTheme.value ? "dark" : "light");
  }
  
  // Load theme
  function loadTheme() {
    const saved = localStorage.getItem("killsay-theme");
    isDarkTheme.value = saved !== "light";
    applyTheme();
  }
  
  return {
    chatKey,
    isDarkTheme,
    loadChatKey,
    setChatKey,
    toggleTheme,
    loadTheme,
  };
});
