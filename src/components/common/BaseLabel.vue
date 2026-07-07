<script setup lang="ts">
defineProps<{
  label: string;
}>();
</script>

<template>
  <span class="label-text">
    {{ label }}
  </span>
</template>

<style scoped>
.label-text {
  flex-shrink: 0;
  width: 35%; /* 固定的标签宽度 */
  text-align: center;
  
  font-size: 1.25rem;
  color: #0d58a4;
  letter-spacing: 1px;
}

</style>