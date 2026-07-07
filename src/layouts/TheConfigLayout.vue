
<script setup lang="ts">
import BaseButton from '@/components/common/BaseButton.vue';
import TheAPIPanel from '@/components/TheAPIPanel.vue';

</script>

<template>
  <div class="setting-panel">
    <h1 class="title">Configuration</h1>
    <div class="info-panel">
      <TheAPIPanel />
      <div class="course-settings-panel"></div>
    </div>
    <div class="save-button">
      <BaseButton label="Save" style="width: 25%;" />
    </div>
  </div>
</template>

<style scoped>

.setting-panel {
  flex: 1;
  flex-direction: column;
}

.title {
  height: 10%;
  width: 100%;
  font-size: 2rem;
  margin-bottom: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.info-panel {
  height: 55%;
  flex: 1;
  flex-direction: row;
}

.save-button {
  height: 15%;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
