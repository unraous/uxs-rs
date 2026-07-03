<script setup lang="ts">
import BaseLabel from './BaseLabel.vue';

const props = withDefaults(
  defineProps<{
    modelValue: number; 
    label: string;
    options: string[];    
  }>(),
  {
    modelValue: () => 0,
    options: () => ["undefined", "option1", "option2", "option3"]
  }
);

const emit = defineEmits<{
  (e: 'update:modelValue', value: number): void;
}>();

/**
 * 处理下拉框发生改变的事件
 */
const handleSelectChange = (event: Event) => {
  const target = event.target as HTMLSelectElement;
  emit('update:modelValue', Number.parseInt(target.value));
};
</script>

<template>
  <div class="base-config-select">
    <BaseLabel :label="label" />
    
    <div class="input-section">
      <select
        :id = "label"
        :value="modelValue"
        @change="handleSelectChange"
        class="single-select"
      >
        <option value="" disabled hidden>请选择...</option>
        <option 
          v-for="opt, index in options" 
          :key="index" 
          :value="index"
        >
          {{ opt }}
        </option>
      </select>
    </div>
  </div>
</template>

<style scoped>
.base-config-select {
  display: flex;
  align-items: center;
  gap: 16px;
  width: 100%;
  margin: 12px 0;
}

.input-section {
  flex-grow: 1;
}

.single-select {
  width: 100%; /* 单选框占满容器宽度 */
  padding: 6px 12px;
  font-size: 1rem;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  background-color: #fff;
  color: #606266;
  outline: none;
  cursor: pointer;
  transition: border-color 0.2s;
}

.single-select:focus {
  border-color: #0d58a4;
}
</style>