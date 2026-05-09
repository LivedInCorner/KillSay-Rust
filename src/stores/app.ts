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
      // Dark theme (default)
      root.style.setProperty("--bg-primary", "#1A1D22");
      root.style.setProperty("--bg-panel", "#272B33");
      root.style.setProperty("--bg-secondary", "#1E2128");
      root.style.setProperty("--text-primary", "#E5E7EB");
      root.style.setProperty("--text-secondary", "#9CA3AF");
      root.style.setProperty("--border-light", "#3A3F48");
      root.style.setProperty("--glass-bg", "rgba(39, 43, 51, 0.75)");
      root.style.setProperty("--glass-border", "rgba(249, 211, 66, 0.15)");
      root.style.setProperty("--glass-blur", "blur(16px)");
      root.style.setProperty("--shadow-sm", "0 2px 8px rgba(0, 0, 0, 0.3)");
      root.style.setProperty("--shadow-md", "0 4px 16px rgba(0, 0, 0, 0.4)");
      root.style.setProperty("--input-bg", "#1E2128");
      root.style.setProperty("--hover-bg", "rgba(255, 255, 255, 0.05)");
    } else {
      // Light theme
      root.style.setProperty("--bg-primary", "#F8F9FA");
      root.style.setProperty("--bg-panel", "#FFFFFF");
      root.style.setProperty("--bg-secondary", "#F0F1F3");
      root.style.setProperty("--text-primary", "#1F2937");
      root.style.setProperty("--text-secondary", "#6B7280");
      root.style.setProperty("--border-light", "#E5E7EB");
      root.style.setProperty("--glass-bg", "rgba(255, 255, 255, 0.85)");
      root.style.setProperty("--glass-border", "rgba(0, 0, 0, 0.08)");
      root.style.setProperty("--glass-blur", "blur(12px)");
      root.style.setProperty("--shadow-sm", "0 2px 8px rgba(0, 0, 0, 0.08)");
      root.style.setProperty("--shadow-md", "0 4px 16px rgba(0, 0, 0, 0.12)");
      root.style.setProperty("--input-bg", "#FFFFFF");
      root.style.setProperty("--hover-bg", "rgba(0, 0, 0, 0.04)");
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
