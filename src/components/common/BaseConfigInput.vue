<script setup lang="ts">
import BaseTextBox from './BaseTextBox.vue';
import BaseConfigLabel from './BaseConfigLabel.vue';

defineProps<{
  modelValue: string;
  label: string;
  placeholder?: string;
}>();

const emit = defineEmits(['update:modelValue']);
</script>

<template>
  <div class="base-config-input">
    <!-- 仅保留文本标签，设定固定宽度以确保右侧输入框对齐 -->
    <BaseConfigLabel :label="label" />
    <div class="input-section">
      <BaseTextBox 
        :modelValue="modelValue" 
        :placeholder="placeholder"
        @update:modelValue="emit('update:modelValue', $event)" 
      />
    </div>
  </div>
</template>

<style scoped>
.base-config-input {
  display: flex;
  align-items: center;
  gap: 16px;
  width: 100%;
  margin: 12px 0;
}

.label-text {
  flex-shrink: 0;
  width: 30%; /* 固定的标签宽度 */
  text-align: center;
  
  font-size: 1.25rem;
  color: #0d58a4;
  letter-spacing: 1px;
}

.input-section {
  flex-grow: 1;
}
</style>