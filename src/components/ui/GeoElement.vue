<script setup lang="ts">
import { computed } from "vue";

const props = withDefaults(defineProps<{
  type?: "hexagon" | "triangle" | "circle" | "diamond" | "cross";
  size?: number;
  color?: string;
  opacity?: number;
}>(), {
  type: "hexagon",
  size: 24,
  color: "var(--accent-yellow)",
  opacity: 0.3,
});

const hexagonPoints = computed(() => {
  const s = props.size;
  const cx = s / 2;
  const cy = s / 2;
  const r = s / 2 - 2;
  return Array.from({ length: 6 }, (_, i) => {
    const angle = (Math.PI / 3) * i - Math.PI / 2;
    return `${cx + r * Math.cos(angle)},${cy + r * Math.sin(angle)}`;
  }).join(" ");
});

const trianglePoints = computed(() => {
  const s = props.size;
  return `${s / 2},2 ${s - 2},${s - 2} 2,${s - 2}`;
});

const diamondPoints = computed(() => {
  const s = props.size;
  return `${s / 2},2 ${s - 2},${s / 2} ${s / 2},${s - 2} 2,${s / 2}`;
});

const crossPath = computed(() => {
  const s = props.size;
  const w = s / 4;
  return `M${s / 2 - w},2 H${s / 2 + w} V${s / 2 - w} H${s - 2} V${s / 2 + w} H${s / 2 + w} V${s - 2} H${s / 2 - w} V${s / 2 + w} H2 V${s / 2 - w} H${s / 2 - w} Z`;
});
</script>

<template>
  <div class="geo-element" :class="[`geo-${type}`]">
    <svg
      :width="size"
      :height="size"
      :viewBox="`0 0 ${size} ${size}`"
      fill="none"
    >
      <!-- 六边形 -->
      <polygon
        v-if="type === 'hexagon'"
        :points="hexagonPoints"
        :fill="color"
        :opacity="opacity"
      />
      <!-- 三角形 -->
      <polygon
        v-if="type === 'triangle'"
        :points="trianglePoints"
        :fill="color"
        :opacity="opacity"
      />
      <!-- 圆形 -->
      <circle
        v-if="type === 'circle'"
        :cx="size / 2"
        :cy="size / 2"
        :r="size / 2 - 2"
        :fill="color"
        :opacity="opacity"
      />
      <!-- 菱形 -->
      <polygon
        v-if="type === 'diamond'"
        :points="diamondPoints"
        :fill="color"
        :opacity="opacity"
      />
      <!-- 十字 -->
      <path
        v-if="type === 'cross'"
        :d="crossPath"
        :fill="color"
        :opacity="opacity"
      />
    </svg>
  </div>
</template>

<style scoped>
.geo-element {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}
</style>
