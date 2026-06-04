<script setup lang="ts">

</script>

<template>
  <div class="left-container">
    <div class="info-panel">this is in the info panel</div>
    <div class="setting-panel">this is in the setting panel</div>
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

.setting-panel {
  flex: 1;
  background-color: lightgray;
}
</style>
