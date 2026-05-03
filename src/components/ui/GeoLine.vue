<script setup lang="ts">
defineProps<{
  direction?: "horizontal" | "vertical" | "diagonal";
  length?: number;
  animated?: boolean;
}>();
</script>

<template>
  <div
    class="geo-line"
    :class="[`line-${direction || 'horizontal'}`, { animated }]"
    :style="{ [direction === 'vertical' ? 'height' : 'width']: `${length || 100}%` }"
  >
    <div class="line-inner"></div>
    <div class="line-dot line-dot-start"></div>
    <div class="line-dot line-dot-end"></div>
  </div>
</template>

<style scoped>
.geo-line {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

.line-horizontal {
  height: 1px;
  width: 100%;
}

.line-vertical {
  width: 1px;
  height: 100%;
}

.line-diagonal {
  height: 1px;
  width: 100%;
  transform: rotate(-45deg);
}

.line-inner {
  position: absolute;
  background: linear-gradient(
    90deg,
    transparent,
    var(--border-light) 20%,
    var(--accent-yellow) 50%,
    var(--border-light) 80%,
    transparent
  );
  opacity: 0.5;
}

.line-horizontal .line-inner {
  width: 100%;
  height: 1px;
}

.line-vertical .line-inner {
  height: 100%;
  width: 1px;
  background: linear-gradient(
    180deg,
    transparent,
    var(--border-light) 20%,
    var(--accent-yellow) 50%,
    var(--border-light) 80%,
    transparent
  );
}

.line-dot {
  position: absolute;
  width: 4px;
  height: 4px;
  background: var(--accent-yellow);
  border-radius: 50%;
  opacity: 0.6;
}

.line-horizontal .line-dot-start {
  left: 0;
}

.line-horizontal .line-dot-end {
  right: 0;
}

.line-vertical .line-dot-start {
  top: 0;
}

.line-vertical .line-dot-end {
  bottom: 0;
}

.animated .line-inner {
  animation: line-pulse 3s ease-in-out infinite;
}

@keyframes line-pulse {
  0%, 100% {
    opacity: 0.3;
  }
  50% {
    opacity: 0.7;
  }
}
</style>
