<script setup lang="ts">
import TheSettingsPanel from '@/components/panels/TheSettingsPanel.vue';
</script>

<template>
  <div class="left-container">
    <div class="info-panel">this is in the info panel</div>
    <TheSettingsPanel />
  </div>
</template>

<style scoped>
.left-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  background-color: yellow;
}

.info-panel {
  height: 40%;
  background-color: cyan;
}


</style>
