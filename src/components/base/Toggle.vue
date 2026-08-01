<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, computed, useId } from "vue";
import Label from "./Label.vue";

const props = withDefaults(
  defineProps<{
    label: string;
    modelValue?: boolean;
    id?: string;
  }>(),
  {
    modelValue: false,
  },
);

const emit = defineEmits(["update:modelValue"]);
const toggleId = computed(() => props.id || useId());

// 初始旋转角度
const rotationAngle = ref(180);
const ballVisible = ref(false);
let timer: ReturnType<typeof setTimeout> | null = null;

const toggle = () => {
  emit("update:modelValue", !props.modelValue);
};

const handleStateUpdate = (isON: boolean, isInitial = false) => {
  if (isInitial) {
    rotationAngle.value = isON ? 0 : 180;
  } else {
    rotationAngle.value -= 180;
  }

  if (timer) clearTimeout(timer);

  // 延迟时间与 CSS 变量 --duration-ring 保持一致 (400ms)
  const delay = isInitial ? 250 : 300;

  timer = setTimeout(() => {
    ballVisible.value = props.modelValue;
  }, delay);
};

watch(
  () => props.modelValue,
  (newVal) => handleStateUpdate(newVal),
);

onMounted(() => handleStateUpdate(props.modelValue, true));
onUnmounted(() => timer && clearTimeout(timer));
</script>

<template>
  <div class="base-config-input">
    <Label :label="label" :for="toggleId" :aria-label="label" />

    <div class="input-section">
      <div
        :id="toggleId"
        class="bowl-toggle"
        role="switch"
        :aria-checked="modelValue"
        tabindex="0"
        @click="toggle"
        @keydown.space.prevent="toggle"
        @keydown.enter.prevent="toggle"
      >
        <!-- 变量控制中心 -->
        <div class="canvas-area">
          <div
            class="bowl-ring"
            :style="{ transform: `rotate(${rotationAngle}deg)` }"
          >
            <svg viewBox="0 0 40 40" class="bowl-svg">
              <path
                d="M 4,20 A 16,16 0 0 0 36,20"
                fill="none"
                stroke="currentColor"
                stroke-width="5"
                stroke-linecap="round"
              />
            </svg>
          </div>

          <div class="ball-clip-box">
            <transition name="ball-physics">
              <div v-if="ballVisible" class="ball" />
            </transition>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.base-config-input {
  display: flex;
  align-items: center;
  width: 100%;
  height: 48px;
}

.input-section {
  flex: 1;
  display: flex;
  justify-content: center;
}

.bowl-toggle {
  width: 80px;
  height: 60px;
  cursor: pointer;
  display: flex;
  justify-content: center;
  align-items: center;
  user-select: none;
}

/* --- 变量控制中心 --- */
.canvas-area {
  position: relative;

  /* 基础尺寸与颜色 */
  --size: 40px;
  --color: #0d58a4;
  --ball-size: 24px;

  /* 动画时间 */
  --duration-ring: 0.3s;
  --duration-ball-in: 0.25s;
  --duration-ball-out: 0.25s;

  /* 物理数值 */
  --drop-height: -50px;
  --bounce-height: -5px;

  width: var(--size);
  height: var(--size);
  color: var(--color); /* 传导给 SVG 的 scaleColor */
}

.bowl-ring {
  position: absolute;
  inset: 0;
  transition:
    transform var(--duration-ring) cubic-bezier(0.3, 1.4, 0.6, 1),
    filter 0.25s ease;
  transform-origin: center;
  z-index: 2;
}

.ball-clip-box {
  position: absolute;
  inset: 0;
  overflow: hidden;
  pointer-events: none;
  z-index: 1;
}

.ball {
  position: absolute;
  width: var(--ball-size);
  height: var(--ball-size);
  background-color: var(--color);
  border-radius: 50%;
  transition: filter 0.25s ease;

  left: calc((var(--size) - var(--ball-size)) / 2);
  top: calc((var(--size) - var(--ball-size)) / 2);
}

.ball-physics-enter-active {
  animation: ball-drop-in var(--duration-ball-in) ease-out both;
}

@keyframes ball-drop-in {
  0% {
    transform: translateY(var(--drop-height));
  }
  60% {
    transform: translateY(0);
  }
  80% {
    transform: translateY(var(--bounce-height));
  }
  100% {
    transform: translateY(0);
  }
}

.ball-physics-leave-active {
  transition: transform var(--duration-ball-out) ease-in;
}

.ball-physics-leave-to {
  transform: translateY(calc(var(--size) + 10px));
}

.bowl-toggle:hover .bowl-ring,
.bowl-toggle:hover .ball {
  filter: brightness(1.25);
}
</style>
