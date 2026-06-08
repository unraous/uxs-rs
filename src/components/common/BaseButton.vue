<script setup lang="ts">
import { ref } from 'vue';

const ripples = ref<{ id: number; x: number; y: number; size: number }[]>([]);
let rippleCount = 0;

const createRipple = (event: MouseEvent) => {
  const button = event.currentTarget as HTMLElement;
  const rect = button.getBoundingClientRect();
  
  const size = Math.max(rect.width, rect.height) * 2.5;
  
  const x = event.clientX - rect.left - size / 2;
  const y = event.clientY - rect.top - size / 2;
  
  const id = rippleCount++;
  ripples.value.push({ id, x, y, size });

  // 3. 动画完成后自动清理 DOM (同步 CSS 的 0.6s)
  setTimeout(() => {
    ripples.value = ripples.value.filter(r => r.id !== id);
  }, 600);
};

</script>

<template>
  <button class="base-button" @mousedown="createRipple">
    <div class="content">
      <slot><span class="text">Explore Coffee</span></slot>
    </div>
    
    <span 
      v-for="ripple in ripples" 
      :key="ripple.id" 
      class="ripple"
      :style="{
        left: ripple.x + 'px',
        top: ripple.y + 'px',
        width: ripple.size + 'px',
        height: ripple.size + 'px'
      }"
    ></span>
  </button>
</template>

<style scoped>
.base-button {
  --height: 54px;
    
  --brand-color: #0d58a4; 
  --color-mid: var(--brand-color);
  --color-dark: color-mix(in srgb, var(--brand-color), black 30%);
  --color-light: color-mix(in srgb, var(--brand-color), white 60%);

  --text-color: #e8dcc4;

  height: var(--height);
  padding: 0 calc(var(--height));
  border-radius: calc(var(--height) / 2);
  
  color: var(--text-color);
  background: conic-gradient(
    from 145deg at 50% 0%, 
    var(--color-dark) 0deg, 
    var(--color-mid) 160deg, 
    var(--color-light) 180deg, 
    var(--color-mid) 200deg, 
    var(--color-dark) 360deg
  );
  
  border: none;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
  box-shadow: 0 10px 20px rgba(0, 0, 0, 0.2);
  position: relative;
  overflow: hidden;
}

/* 涟漪圆样式 */
.ripple {
  position: absolute;
  background: rgba(255, 255, 255, 0.4); /* 白色半透明 */
  border-radius: 50%;
  pointer-events: none; /* 必须：确保不阻挡后续的 click 事件 */
  transform: scale(0);
  animation: ripple-animation 0.6s ease-out;
}

@keyframes ripple-animation {
  from {
    transform: scale(0);
    opacity: 1;
  }
  to {
    transform: scale(1);
    opacity: 0;
  }
}

.content {
  position: relative;
  z-index: 1; /* 确保文字始终在白圆上方 */
}

/* 交互效果 */
.base-button:hover {
  filter: brightness(1.15);
  transform: scale(1.05);
  box-shadow: 0 12px 25px rgba(0, 0, 0, 0.3);
}

.base-button:active {
  transform: scale(0.95);
}

</style>