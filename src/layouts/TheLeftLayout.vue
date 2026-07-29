<script setup lang="ts">
import TheConfigLayout from "./TheConfigLayout.vue";
</script>

<template>
  <div class="left-container">
    <div class="info-panel">this is in the info panel</div>
    <TheConfigLayout />
  </div>
</template>

<style scoped>
.left-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  background-color: transparent;
}

.info-panel {
  height: 50%;
  background-color: transparent;
}
</style>
