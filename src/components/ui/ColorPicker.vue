<script setup lang="ts">
import { ref, computed, onMounted, watch } from "vue";

const props = defineProps<{
  initialColor?: string;
}>();

const emit = defineEmits<{
  select: [color: string];
  close: [];
}>();

// HSV values
const hue = ref(0);
const saturation = ref(100);
const value = ref(100);

// UI state
const isDraggingSV = ref(false);
const isDraggingHue = ref(false);
const hexInput = ref("");

// Convert HSV to RGB
function hsvToRgb(h: number, s: number, v: number): [number, number, number] {
  s /= 100;
  v /= 100;
  const c = v * s;
  const x = c * (1 - Math.abs(((h / 60) % 2) - 1));
  const m = v - c;
  
  let r = 0, g = 0, b = 0;
  if (h < 60) { r = c; g = x; b = 0; }
  else if (h < 120) { r = x; g = c; b = 0; }
  else if (h < 180) { r = 0; g = c; b = x; }
  else if (h < 240) { r = 0; g = x; b = c; }
  else if (h < 300) { r = x; g = 0; b = c; }
  else { r = c; g = 0; b = x; }
  
  return [
    Math.round((r + m) * 255),
    Math.round((g + m) * 255),
    Math.round((b + m) * 255)
  ];
}

// Convert RGB to HEX
function rgbToHex(r: number, g: number, b: number): string {
  return "#" + [r, g, b].map(x => x.toString(16).padStart(2, "0")).join("");
}

// Convert HEX to HSV
function hexToHsv(hex: string): [number, number, number] | null {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  if (!result) return null;
  
  const r = parseInt(result[1], 16) / 255;
  const g = parseInt(result[2], 16) / 255;
  const b = parseInt(result[3], 16) / 255;
  
  const max = Math.max(r, g, b);
  const min = Math.min(r, g, b);
  const d = max - min;
  
  let h = 0;
  const s = max === 0 ? 0 : (d / max) * 100;
  const v = max * 100;
  
  if (d !== 0) {
    if (max === r) h = ((g - b) / d + (g < b ? 6 : 0)) * 60;
    else if (max === g) h = ((b - r) / d + 2) * 60;
    else h = ((r - g) / d + 4) * 60;
  }
  
  return [h, s, v];
}

// Current color values
const currentRgb = computed(() => hsvToRgb(hue.value, saturation.value, value.value));
const currentHex = computed(() => rgbToHex(...currentRgb.value));
const currentHsl = computed(() => {
  const [r, g, b] = currentRgb.value.map(x => x / 255);
  const max = Math.max(r, g, b);
  const min = Math.min(r, g, b);
  const l = (max + min) / 2;
  const d = max - min;
  let h = 0, s = 0;
  
  if (d !== 0) {
    s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
    if (max === r) h = ((g - b) / d + (g < b ? 6 : 0)) * 60;
    else if (max === g) h = ((b - r) / d + 2) * 60;
    else h = ((r - g) / d + 4) * 60;
  }
  
  return `hsl(${Math.round(h)}, ${Math.round(s * 100)}%, ${Math.round(l * 100)}%)`;
});

// Update hex input when color changes
watch(currentHex, (val) => {
  hexInput.value = val.toUpperCase();
}, { immediate: true });

// Parse hex input
function handleHexInput() {
  const hsv = hexToHsv(hexInput.value);
  if (hsv) {
    [hue.value, saturation.value, value.value] = hsv;
  }
}

// SV panel interaction
function handleSVMouseDown(e: MouseEvent) {
  isDraggingSV.value = true;
  updateSV(e);
}

function updateSV(e: MouseEvent) {
  const panel = (e.target as HTMLElement).closest(".sv-panel");
  if (!panel) return;
  
  const rect = panel.getBoundingClientRect();
  const x = Math.max(0, Math.min(e.clientX - rect.left, rect.width));
  const y = Math.max(0, Math.min(e.clientY - rect.top, rect.height));
  
  saturation.value = (x / rect.width) * 100;
  value.value = 100 - (y / rect.height) * 100;
}

// Hue slider interaction
function handleHueMouseDown(e: MouseEvent) {
  isDraggingHue.value = true;
  updateHue(e);
}

function updateHue(e: MouseEvent) {
  const slider = (e.target as HTMLElement).closest(".hue-slider");
  if (!slider) return;
  
  const rect = slider.getBoundingClientRect();
  const x = Math.max(0, Math.min(e.clientX - rect.left, rect.width));
  hue.value = (x / rect.width) * 360;
}

// Mouse move and up handlers
function handleMouseMove(e: MouseEvent) {
  if (isDraggingSV.value) updateSV(e);
  if (isDraggingHue.value) updateHue(e);
}

function handleMouseUp() {
  isDraggingSV.value = false;
  isDraggingHue.value = false;
}

// Preset colors
const presetColors = [
  "#F9D342", "#FF6B6B", "#4ECDC4", "#45B7D1",
  "#96CEB4", "#FFEAA7", "#DDA0DD", "#98D8C8",
  "#F7DC6F", "#BB8FCE", "#85C1E9", "#F1948A",
];

// Select color
function selectColor(hex: string) {
  const hsv = hexToHsv(hex);
  if (hsv) {
    [hue.value, saturation.value, value.value] = hsv;
  }
}

// Confirm selection
function confirmSelection() {
  emit("select", currentHex.value);
  emit("close");
}

onMounted(() => {
  document.addEventListener("mousemove", handleMouseMove);
  document.addEventListener("mouseup", handleMouseUp);
  
  // Parse initial color
  if (props.initialColor) {
    const hsv = hexToHsv(props.initialColor);
    if (hsv) {
      [hue.value, saturation.value, value.value] = hsv;
    }
  }
});
</script>

<template>
  <div class="color-picker-overlay" @mousedown.self="emit('close')">
    <div class="color-picker-panel dg-panel">
      <div class="color-picker-header">
        <h3>选择主题色</h3>
        <button class="dg-btn dg-btn--secondary dg-btn--sm" @click="emit('close')">关闭</button>
      </div>
      
      <div class="color-picker-content">
        <!-- SV Panel -->
        <div
          class="sv-panel"
          :style="{ backgroundColor: `hsl(${hue}, 100%, 50%)` }"
          @mousedown="handleSVMouseDown"
        >
          <div class="sv-white"></div>
          <div class="sv-black"></div>
          <div
            class="sv-cursor"
            :style="{
              left: `${saturation}%`,
              top: `${100 - value}%`,
            }"
          ></div>
        </div>
        
        <!-- Hue Slider -->
        <div class="hue-slider" @mousedown="handleHueMouseDown">
          <div
            class="hue-cursor"
            :style="{ left: `${(hue / 360) * 100}%` }"
          ></div>
        </div>
        
        <!-- Preview -->
        <div class="color-preview-area">
          <div
            class="color-preview"
            :style="{ backgroundColor: currentHex }"
          ></div>
          <div class="color-values">
            <div class="color-value-row">
              <label>HEX</label>
              <input
                v-model="hexInput"
                class="dg-input dg-input--sm"
                @change="handleHexInput"
              />
            </div>
            <div class="color-value-row">
              <label>RGB</label>
              <span>{{ currentRgb.join(", ") }}</span>
            </div>
          </div>
        </div>
        
        <!-- Preset Colors -->
        <div class="preset-colors">
          <span class="preset-label">预设颜色</span>
          <div class="preset-grid">
            <div
              v-for="color in presetColors"
              :key="color"
              class="preset-color"
              :style="{ backgroundColor: color }"
              @click="selectColor(color)"
            ></div>
          </div>
        </div>
      </div>
      
      <div class="color-picker-actions">
        <button class="dg-btn dg-btn--primary" @click="confirmSelection">
          确认
        </button>
        <button class="dg-btn dg-btn--secondary" @click="emit('close')">取消</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.color-picker-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 200;
  backdrop-filter: blur(4px);
  animation: fadeIn 0.2s ease;
}

.color-picker-panel {
  width: 360px;
  animation: slideUp 0.25s ease;
}

.color-picker-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-md);
}

.color-picker-header h3 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.color-picker-content {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

/* SV Panel */
.sv-panel {
  width: 100%;
  height: 200px;
  position: relative;
  cursor: crosshair;
  border-radius: var(--radius-small);
  overflow: hidden;
}

.sv-white {
  position: absolute;
  inset: 0;
  background: linear-gradient(to right, white, transparent);
}

.sv-black {
  position: absolute;
  inset: 0;
  background: linear-gradient(to bottom, transparent, black);
}

.sv-cursor {
  position: absolute;
  width: 14px;
  height: 14px;
  border: 2px solid white;
  border-radius: 50%;
  transform: translate(-50%, -50%);
  box-shadow: 0 0 4px rgba(0, 0, 0, 0.5);
  pointer-events: none;
}

/* Hue Slider */
.hue-slider {
  width: 100%;
  height: 16px;
  position: relative;
  cursor: pointer;
  border-radius: 8px;
  background: linear-gradient(
    to right,
    hsl(0, 100%, 50%),
    hsl(60, 100%, 50%),
    hsl(120, 100%, 50%),
    hsl(180, 100%, 50%),
    hsl(240, 100%, 50%),
    hsl(300, 100%, 50%),
    hsl(360, 100%, 50%)
  );
}

.hue-cursor {
  position: absolute;
  top: 50%;
  width: 20px;
  height: 20px;
  background: white;
  border: 2px solid var(--bg-primary);
  border-radius: 50%;
  transform: translate(-50%, -50%);
  box-shadow: 0 0 4px rgba(0, 0, 0, 0.3);
  pointer-events: none;
}

/* Preview */
.color-preview-area {
  display: flex;
  gap: var(--spacing-md);
  align-items: flex-start;
}

.color-preview {
  width: 60px;
  height: 60px;
  border-radius: var(--radius-small);
  border: 2px solid var(--border-light);
  flex-shrink: 0;
}

.color-values {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.color-value-row {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
}

.color-value-row label {
  width: 32px;
  color: var(--text-secondary);
  font-weight: 500;
}

.color-value-row span {
  color: var(--text-primary);
  font-family: var(--font-mono);
  font-size: 11px;
}

.dg-input--sm {
  padding: 4px 8px;
  font-size: 11px;
  font-family: var(--font-mono);
}

/* Preset Colors */
.preset-colors {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.preset-label {
  font-size: 11px;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 1px;
}

.preset-grid {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  gap: 6px;
}

.preset-color {
  width: 100%;
  aspect-ratio: 1;
  border-radius: 4px;
  cursor: pointer;
  border: 2px solid transparent;
  transition: all 0.15s ease;
}

.preset-color:hover {
  transform: scale(1.1);
  border-color: var(--text-primary);
}

/* Actions */
.color-picker-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  margin-top: var(--spacing-md);
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
