<script setup lang="ts">
import Button from "@/components/base/Button.vue";
import TheAPIPanel from "@/components/panels/TheAPIPanel.vue";
import TheCourseConfigPanel from "@/components/panels/TheCourseConfigPanel.vue";
import { commands } from "@/services/cmds";
</script>

<template>
  <div class="setting-panel">
    <h1 class="title">Configuration</h1>
    <div class="info-panel">
      <TheAPIPanel ref="apiPanelRef" />
      <TheCourseConfigPanel ref="courseConfigPanelRef" />
    </div>
    <div class="save-button" @click="commands.saveConfig">
      <Button label="Save" style="width: 35%; height: 60%; font-size: 1.5rem" />
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
  display: flex;
  gap: 16px;
  flex-direction: row;
}

.save-button {
  height: 20%;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
