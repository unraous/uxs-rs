<script setup lang="ts">
import { computed, useId } from "vue";
import TextBox from "./TextBox.vue";
import Label from "./Label.vue";

const props = defineProps<{
  modelValue: string | number;
  label: string;
  placeholder?: string;
  pattern?: string;
  id?: string;
}>();

const emit = defineEmits(["update:modelValue", "change"]);
const inputId = computed(() => props.id || useId());
</script>

<template>
  <div class="base-config-input">
    <!-- 仅保留文本标签，设定固定宽度以确保右侧输入框对齐 -->
    <Label :label="label" :aria-label="label" :for="inputId" />
    <TextBox
      :id="inputId"
      :modelValue="modelValue"
      :placeholder="placeholder"
      :pattern="pattern"
      class="input-section"
      @update:modelValue="emit('update:modelValue', $event)"
      @change="emit('change', $event)"
    />
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
