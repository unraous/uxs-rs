
<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  modelValue: string;
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

<template>
  <div class="base-text-box">
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

<style scoped>
.base-text-box {
  --brand-color: #0d58a4;
  --contrast-border: color-mix(in srgb, var(--brand-color), black 30%);
  --base-thickness: 2px;    /* 初始 L 边框厚度 */
  --lift-thickness: 5px;    /* 抬升后的阴影厚度 */
  
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.input-container {
  position: relative;
  width: 100%;
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
    calc(100% - var(--t)) 0,     
    100% var(--t),               
    100% 100%,                   
    var(--t) 100%,               
    0 calc(100% - var(--t)),     
    calc(100% - var(--t)) calc(100% - var(--t)) 
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
}

.input-field:hover ~ .shadow-shell {
  background: var(--contrast-border);
}

.input-field::placeholder {
  color: color-mix(in srgb, var(--brand-color), transparent 60%);
}
</style>