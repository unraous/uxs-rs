<script setup lang="ts">
import BaseTextBox from './BaseTextBox.vue';
import BaseLabel from './BaseLabel.vue';

defineProps<{
  modelValue: any;
  label: string;
  placeholder?: string;
  pattern?: string;
}>();

const emit = defineEmits(['update:modelValue', 'change']);
</script>

<template>
  <div class="base-config-input">
    <!-- 仅保留文本标签，设定固定宽度以确保右侧输入框对齐 -->
    <BaseLabel :label="label" />
    <div class="input-section">
      <BaseTextBox 
        :modelValue="modelValue" 
        :placeholder="placeholder"
        :pattern="pattern"
        @update:modelValue="emit('update:modelValue', $event)" 
        @change="emit('change', $event)"
      />
    </div>
  </div>
</template>

<style scoped>
.base-config-input {
  display: flex;
  align-items: center;
  width: 100%;
}


.input-section {
  flex-grow: 1;
}
</style>