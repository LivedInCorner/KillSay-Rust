<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  value: number;
  max?: number;
  label: string;
  unit?: string;
  color?: string;
  size?: number;
}>();

const radius = computed(() => (props.size || 120) / 2 - 10);
const circumference = computed(() => 2 * Math.PI * radius.value);
const progress = computed(() => {
  const max = props.max || 100;
  return Math.min(props.value / max, 1);
});
const dashOffset = computed(() => circumference.value * (1 - progress.value));
const center = computed(() => (props.size || 120) / 2);
</script>

<template>
  <div class="ring-stat" :style="{ width: `${size || 120}px`, height: `${size || 120}px` }">
    <!-- 背景环 -->
    <svg class="ring-svg" :width="size || 120" :height="size || 120">
      <!-- 刻度线 -->
      <g class="scale-marks">
        <line
          v-for="i in 36"
          :key="i"
          :x1="center + (radius - 8) * Math.cos((i * 10 * Math.PI) / 180)"
          :y1="center + (radius - 8) * Math.sin((i * 10 * Math.PI) / 180)"
          :x2="center + radius * Math.cos((i * 10 * Math.PI) / 180)"
          :y2="center + radius * Math.sin((i * 10 * Math.PI) / 180)"
          stroke="var(--border-light)"
          stroke-width="1"
          :opacity="i % 9 === 0 ? 0.8 : 0.3"
        />
      </g>
      
      <!-- 背景轨道 -->
      <circle
        :cx="center"
        :cy="center"
        :r="radius"
        fill="none"
        stroke="var(--border-light)"
        stroke-width="3"
        opacity="0.3"
      />
      
      <!-- 进度环 -->
      <circle
        :cx="center"
        :cy="center"
        :r="radius"
        fill="none"
        :stroke="color || 'var(--accent-yellow)'"
        stroke-width="3"
        stroke-linecap="round"
        :stroke-dasharray="circumference"
        :stroke-dashoffset="dashOffset"
        transform-origin="center"
        transform="rotate(-90)"
        class="progress-ring"
      />
      
      <!-- 装饰点 -->
      <circle
        :cx="center + radius * Math.cos((-90 + progress * 360) * Math.PI / 180)"
        :cy="center + radius * Math.sin((-90 + progress * 360) * Math.PI / 180)"
        r="4"
        :fill="color || 'var(--accent-yellow)'"
        class="progress-dot"
      />
    </svg>
    
    <!-- 中心内容 -->
    <div class="ring-content">
      <div class="ring-value">{{ value }}</div>
      <div class="ring-label">{{ label }}</div>
    </div>
  </div>
</template>

<style scoped>
.ring-stat {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ring-svg {
  position: absolute;
  top: 0;
  left: 0;
}

.progress-ring {
  transition: stroke-dashoffset 0.5s ease;
}

.progress-dot {
  filter: drop-shadow(0 0 4px var(--accent-yellow));
}

.ring-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  z-index: 1;
}

.ring-value {
  font-size: 28px;
  font-weight: 700;
  font-family: var(--font-mono);
  color: var(--text-primary);
  line-height: 1;
}

.ring-label {
  font-size: 11px;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 1.5px;
  margin-top: 4px;
}
</style>
