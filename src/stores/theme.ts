import { defineStore } from "pinia";
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";

// Default accent color
const DEFAULT_COLOR = "#F9D342";

// Convert hex to HSL components for CSS
function hexToHsl(hex: string): { h: number; s: number; l: number } {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  if (!result) return { h: 0, s: 0, l: 0 };
  
  let r = parseInt(result[1], 16) / 255;
  let g = parseInt(result[2], 16) / 255;
  let b = parseInt(result[3], 16) / 255;
  
  const max = Math.max(r, g, b);
  const min = Math.min(r, g, b);
  const d = max - min;
  const l = (max + min) / 2;
  
  let h = 0, s = 0;
  if (d !== 0) {
    s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
    if (max === r) h = ((g - b) / d + (g < b ? 6 : 0)) * 60;
    else if (max === g) h = ((b - r) / d + 2) * 60;
    else h = ((r - g) / d + 4) * 60;
  }
  
  return { h, s: s * 100, l: l * 100 };
}

// Generate darker/lighter variants
function generateVariants(hex: string) {
  const { h, s, l } = hexToHsl(hex);
  return {
    accent: hex,
    accentDark: `hsl(${h}, ${s}%, ${Math.max(l - 15, 10)}%)`,
    accentGlow: `hsla(${h}, ${s}%, ${l}%, 0.35)`,
    accentBorder: `hsla(${h}, ${s}%, ${l}%, 0.15)`,
    accentBg: `hsla(${h}, ${s}%, ${l}%, 0.1)`,
    accentBgHover: `hsla(${h}, ${s}%, ${l}%, 0.15)`,
  };
}

export const useThemeStore = defineStore("theme", () => {
  const accentColor = ref(DEFAULT_COLOR);
  const variants = ref(generateVariants(DEFAULT_COLOR));
  const isDarkTheme = ref(true);
  
  // Load saved color from config file
  async function loadSavedColor() {
    try {
      const saved = await invoke<string>("get_theme_color");
      if (saved) {
        accentColor.value = saved;
        variants.value = generateVariants(saved);
        applyTheme();
      }
    } catch (e) {
      console.error("Failed to load theme color:", e);
    }
  }
  
  // Apply theme to CSS variables
  function applyTheme() {
    const root = document.documentElement;
    const v = variants.value;
    
    root.style.setProperty("--accent-yellow", v.accent);
    root.style.setProperty("--accent-yellow-dark", v.accentDark);
    root.style.setProperty("--highlight-shadow", `0 0 12px ${v.accentGlow}`);
    root.style.setProperty("--glass-border", v.accentBorder);
  }
  
  // Set new accent color
  async function setAccentColor(color: string) {
    accentColor.value = color;
    variants.value = generateVariants(color);
    applyTheme();
    
    try {
      await invoke("set_theme_color", { color });
    } catch (e) {
      console.error("Failed to save theme color:", e);
    }
  }
  
  // Reset to default
  function resetColor() {
    setAccentColor(DEFAULT_COLOR);
  }
  
  // Watch for changes
  watch(accentColor, () => {
    applyTheme();
  });
  
  // Initialize
  loadSavedColor();
  
  return {
    accentColor,
    variants,
    setAccentColor,
    resetColor,
    loadSavedColor,
  };
});
