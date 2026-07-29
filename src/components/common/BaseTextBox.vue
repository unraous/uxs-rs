<script setup lang="ts">
import { computed, useId } from "vue";

const props = defineProps<{
  modelValue: any;
  placeholder?: string;
  pattern?: string;
  id?: string;
}>();

const emit = defineEmits(["update:modelValue", "change"]);
const inputId = computed(() => props.id || useId());

const onInput = (event: Event) => {
  const target = event.target as HTMLInputElement;
  const value = target.value;

  if (
    props.pattern &&
    value !== "" &&
    !new RegExp(`^(?:${props.pattern})$`).test(value)
  ) {
    target.value = props.modelValue || "";
    return;
  }

  emit("update:modelValue", value);
};
</script>

<template>
  <div class="base-text-box">
    <div class="input-container">
      <input
        :id="inputId"
        :value="modelValue"
        :placeholder="placeholder"
        :pattern="pattern"
        class="input-field"
        @input="onInput"
        @change="$emit('change', $event)"
      />
      <div class="shadow-shell"></div>
    </div>
  </div>
</template>

<style scoped>
.base-text-box {
  --brand-color: #0d58a4;
  --contrast-border: color-mix(in srgb, var(--brand-color), black 30%);
  --base-thickness: 2px;
  --lift-thickness: 5px;

  display: flex;
  flex-direction: column;
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

.input-field:focus {
  transform: translate(-3px, -3px);
  border-color: var(--contrast-border);
}

.shadow-shell {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: var(--brand-color);
  z-index: 1;
  pointer-events: none;

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

.input-field:focus ~ .shadow-shell {
  --t: var(--lift-thickness);
  background: var(--contrast-border);
}

.input-field:hover ~ .shadow-shell {
  background: var(--contrast-border);
}

.input-field::placeholder {
  color: color-mix(in srgb, var(--brand-color), transparent 60%);
}
</style>
