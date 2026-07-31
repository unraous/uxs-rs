<script setup lang="ts">
import { computed, ref } from "vue";
import { gsap } from "gsap";
import { useRipple } from "@/effects/useRipple";

const buttonRef = ref<HTMLElement | null>(null);

const props = withDefaults(
  defineProps<{
    /** 文本标签 */
    label?: string;
    /** SVG Raw 字符串图标 */
    iconRaw?: string;
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
  }>(),
  {
    shape: "pill",
    variant: "brand",
    disabled: false,
  },
);

const emit = defineEmits<{
  (e: "click", event: MouseEvent): void;
}>();

const { createRipple } = useRipple({
  scale: props.shape === "circle" ? 2.0 : 2.5,
  duration: 0.6,
});

/** 动态计算尺寸与背景样式 */
const buttonStyle = computed(() => {
  const styles: Record<string, string> = {};

  if (props.size) {
    styles.height = props.size;
    if (props.shape === "circle") {
      styles.width = props.size;
    }
  }

  if (props.background) {
    styles.background = props.background;
  }

  if (props.color) {
    styles.color = props.color;
  }

  return styles;
});

const handleClick = (event: MouseEvent) => {
  if (!props.disabled) {
    emit("click", event);
  }
};

/** GSAP 物理交互动画 */
const handleMouseEnter = () => {
  if (props.disabled || !buttonRef.value) return;
  gsap.to(buttonRef.value, {
    scale: 1.025,
    filter: "brightness(1.15)",
    duration: 0.25,
    ease: "power2.out",
  });
};

const handleMouseDown = (event: MouseEvent) => {
  createRipple(event, props.disabled);
  if (props.disabled || !buttonRef.value) return;
  gsap.to(buttonRef.value, {
    scale: 0.95,
    duration: 0.1,
    ease: "power1.out",
  });
};

const handleMouseRelease = () => {
  if (props.disabled || !buttonRef.value) return;
  gsap.to(buttonRef.value, {
    scale: 1,
    filter: "brightness(1)",
    duration: 0.45,
    ease: "back.out(2.5)",
  });
};
</script>

<template>
  <button
    ref="buttonRef"
    class="base-button"
    :class="[
      `shape-${props.shape}`,
      `variant-${props.variant}`,
      { disabled: props.disabled },
    ]"
    :style="buttonStyle"
    @mouseenter="handleMouseEnter"
    @mouseleave="handleMouseRelease"
    @mousedown="handleMouseDown"
    @mouseup="handleMouseRelease"
    @click="handleClick"
  >
    <!-- 内容层 (支持插槽/文字/SVG) -->
    <div class="content">
      <slot>
        <span
          v-if="props.iconRaw"
          class="icon-raw"
          v-html="props.iconRaw"
        ></span>
        <span v-if="props.label" class="label-text">{{ props.label }}</span>
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
}

.icon-raw {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 1em;
  height: 1em;
}

.icon-raw :deep(svg) {
  width: 100%;
  height: 100%;
  fill: currentColor;
}

.base-button.disabled {
  opacity: 0.4;
  cursor: not-allowed;
  box-shadow: none;
}
</style>
