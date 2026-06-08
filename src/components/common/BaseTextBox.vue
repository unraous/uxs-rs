<template>
  <div class="base-text-box">
    <label v-if="label" :for="inputId" class="label">{{ label }}</label>
    <div class="input-container">
      <input
        :id="inputId" 
        :value="modelValue"
        :type="type || 'text'"
        :placeholder="placeholder"
        class="input-field"
        @input="onInput"
      />
      <!-- L 型 3D 阴影外壳 -->
      <div class="shadow-shell"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  modelValue: string;
  label?: string;
  placeholder?: string;
  type?: string;
  id?: string;
}>();

const emit = defineEmits(['update:modelValue']);
const inputId = computed(() => props.id || `input-${Math.random().toString(36).slice(2, 9)}`);

const onInput = (event: Event) => {
  const target = event.target as HTMLInputElement;
  emit('update:modelValue', target.value);
};
</script>

<style scoped>
.base-text-box {
  --brand-color: #0d58a4;
  --contrast-border: color-mix(in srgb, var(--brand-color), black 30%);
  --base-thickness: 2px;    /* 初始 L 边框厚度 */
  --lift-thickness: 8px;    /* 抬升后的阴影厚度 */
  
  display: flex;
  flex-direction: column;
  gap: 8px;
  width: 100%;
}

.label {
  color: var(--brand-color);
  font-weight: 800;
  font-size: 0.85rem;
  letter-spacing: 1.5px;
  text-transform: uppercase;
}

.input-container {
  position: relative;
  width: 100%;
  /* 为右侧和下方的阴影预留空间，防止容器尺寸抖动 */
  padding-right: var(--lift-thickness);
  padding-bottom: var(--lift-thickness);
}

.input-field {
  width: 100%;
  height: 48px;
  padding: 0 12px;
  background-color: var(--bg-color); /* 给输入框一个底色，增强实体感 */
  color: var(--brand-color);
  border: 0px;
  border-radius: 0;
  outline: none;
  font-size: 1.1rem;
  font-weight: 600;
  position: relative;
  z-index: 5;
  
  transition: all 0.2s cubic-bezier(0.2, 0, 0, 1);
}

/* 焦点状态：输入框向左上微移，模拟“离地” */
.input-field:focus {
  transform: translate(-3px, -3px);
  border-color: var(--contrast-border);
}

/* L 型 3D 外壳 */
.shadow-shell {
  position: absolute;
  /* 初始紧贴输入框 */
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: var(--brand-color);
  z-index: 1;
  pointer-events: none;

  /* 
    初始厚度变量 
    使用 clip-path 绘制 45 度斜角的 L 形
  */
  --t: var(--base-thickness);
  clip-path: polygon(
    calc(100% - var(--t)) 0,     /* 右侧内起点 (右上) */
    100% var(--t),               /* 右侧外顶点 (斜切起点) */
    100% 100%,                   /* 右下总外角 */
    var(--t) 100%,               /* 下侧外顶点 (斜切终点) */
    0 calc(100% - var(--t)),     /* 下侧内起点 (左下) */
    calc(100% - var(--t)) calc(100% - var(--t)) /* 内拐角 */
  );

  transition: 
    clip-path 0.2s cubic-bezier(0.2, 0, 0, 1),
    background 0.2s ease,
    transform 0.2s cubic-bezier(0.2, 0, 0, 1);
}

/* 选中或悬停时的变化 */
.input-field:focus ~ .shadow-shell {
  --t: var(--lift-thickness);
  background: var(--contrast-border);
  /* 阴影外壳稍微向右下张开，配合输入框的左上移，形成完美抬升感 */
  transform: translate(2px, 2px); 
}

.input-field:hover ~ .shadow-shell {
  background: var(--contrast-border);
}

.input-field::placeholder {
  color: color-mix(in srgb, var(--brand-color), transparent 60%);
}
</style>