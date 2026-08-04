<script setup lang="ts">
import { computed, ref, type Component } from "vue";
import { useRipple } from "@/composables/useRipple";
import { useMagnetic } from "@/composables/useMagnetic";

const buttonRef = ref<HTMLElement | null>(null);
const contentRef = ref<HTMLElement | null>(null);

const {
  label = "",
  icon = undefined,
  background = "",
  color = "",
  size = "32px",
  shape = "pill",
  variant = "brand",
  disabled = false,
  type = "button",
} = defineProps<{
  /** 文本标签 */
  label?: string;
  /** SVG 组件图标 */
  icon?: Component;
  /** 按钮形状：'pill' (胶囊全圆角) | 'circle' (正圆形) | 'round' (标准圆角) */
  shape?: "pill" | "circle" | "round";
  /** 预设风格：'brand' (主色极光) | 'translucent' (黑半透明) | 'custom' (自定义) */
  variant?: "brand" | "translucent" | "custom";
  /** 自定义背景色 / 渐变色 */
  background?: string;
  /** 自定义文字或图标颜色 */
  color?: string;
  /** 按钮尺寸 (如 '36px' 或 '80%') */
  size?: string;
  /** 是否禁用 */
  disabled?: boolean;
  /** 按钮类型：'button' | 'submit' | 'reset' */
  type?: "button" | "submit" | "reset";
}>();

const emit = defineEmits<{
  (e: "click", event: MouseEvent): void;
}>();

const { createRipple } = useRipple({
  scale: shape === "circle" ? 2.0 : 2.5,
  duration: 0.8,
});

/** 使用拆离组合式设计的精致微幅双层视差磁吸 */
const {
  handleMouseEnter,
  handleMouseMove,
  handleMouseDown,
  handleMouseUp,
  handleMouseLeave,
} = useMagnetic({
  outerRef: buttonRef,
  innerRef: contentRef,
  outerFactor: 0.12,
  innerFactor: 0.36,
  maxDistance: 25,
  disabled: () => disabled,
  onMouseDown: (event) => createRipple(event, disabled),
});

/** 动态计算尺寸与背景样式 */
const buttonStyle = computed(() => {
  const styles: Record<string, string> = {};

  if (size) {
    styles.height = size;
    if (shape === "circle") {
      styles.width = size;
    }
  }

  if (background) {
    styles.background = background;
  }

  if (color) {
    styles.color = color;
  }

  return styles;
});

const handleClick = (event: MouseEvent) => {
  if (!disabled) {
    emit("click", event);
  }
};
</script>

<template>
  <button
    ref="buttonRef"
    :type="type"
    class="base-button"
    :class="[`shape-${shape}`, `variant-${variant}`, { disabled }]"
    :style="buttonStyle"
    @mouseenter="handleMouseEnter"
    @mousemove="handleMouseMove"
    @mouseleave="handleMouseLeave"
    @mousedown="handleMouseDown"
    @mouseup="handleMouseUp"
    @click="handleClick"
  >
    <!-- 内容层 (支持插槽/文字/SVG，绑定 contentRef 实现微幅视差) -->
    <div
      ref="contentRef"
      class="content"
    >
      <slot>
        <component
          :is="icon"
          v-if="icon"
          class="icon-svg"
        />
        <span
          v-if="label"
          class="label-text"
        >
          {{ label }}
        </span>
      </slot>
    </div>
  </button>
</template>

<style scoped>
.base-button {
  border: none;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  overflow: hidden;
  outline: none;
  user-select: none;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  will-change: transform, opacity, filter;
  transition:
    opacity 0.3s cubic-bezier(0.4, 0, 0.2, 1),
    filter 0.25s ease,
    box-shadow 0.3s ease;
}

.base-button:not(.disabled):hover {
  filter: brightness(1.15);
}

/* 形状配置 */
.shape-pill {
  border-radius: 999px;
  padding: 0 1.2rem;
}

.shape-circle {
  border-radius: 50%;
  aspect-ratio: 1 / 1;
  padding: 0;
}

.shape-round {
  border-radius: 8px;
  padding: 0 1rem;
}

/* 预设风格 */
.variant-brand {
  --brand-color: #0d58a4;
  --color-mid: var(--brand-color);
  --color-dark: color-mix(in srgb, var(--brand-color), black 25%);
  --color-light: color-mix(in srgb, var(--brand-color), white 75%);
  color: #e8dcc4;
  background: conic-gradient(
    from 145deg at 50% 0%,
    var(--color-dark) 0deg,
    var(--color-mid) 160deg,
    var(--color-light) 180deg,
    var(--color-mid) 200deg,
    var(--color-dark) 360deg
  );
}

.variant-translucent {
  background-color: rgba(0, 0, 0, 0);
  backdrop-filter: blur(4px);
  color: #ffffff;
}

/* 内部内容与布局 */
.content {
  position: relative;
  z-index: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  pointer-events: none;
  will-change: transform;
}

.icon-svg {
  width: 1.25em;
  height: 1.25em;
  fill: currentColor;
}

.base-button.disabled {
  opacity: 0.4;
  cursor: not-allowed;
  box-shadow: none;
  transform: scale(0.95);
  filter: grayscale(0.5);
}
</style>
