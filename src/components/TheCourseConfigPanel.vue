<script setup lang="ts">
import { ref } from 'vue';
import BaseInput from './common/BaseInput.vue';
import BaseToggle from './common/BaseToggle.vue';

const perisistSession = ref(true);
const muteCourseWebview = ref(true);
const lockPlayingSpeed = ref(false);
const playingSpeed = ref(1.0);




const saveConfig = async () => {
  
};




defineExpose({ saveConfig });
</script>

<template>
  <div class="course-config-panel">
    <h2 class="title">Course</h2>
    <div class="settings-container">
      <BaseToggle v-model="perisistSession" label="Perisist Session" />
      <BaseToggle v-model="muteCourseWebview" label="Mute Course Webview" />
      <BaseToggle v-model="lockPlayingSpeed" label="Lock Playing Speed" />
      <BaseInput v-model.number="playingSpeed" label="Playing Speed" pattern="\d+(?:\.\d*)?" class="speed-input" />
    </div>
  </div>
</template>

<style scoped>
.course-config-panel {
  height: 100%;
  flex: 1;
  flex-direction: column;
  display: flex;
}

.settings-container {
  flex: 1;
  display: flex;
  gap: 16px;
  flex-direction: column;
}
  
.title {
  display: flex;
  height: 48px;
  font-size: 1.5rem;
  align-items: center;
  justify-content: center;   
}


:deep(.base-config-select:first-child .select-dropdown-wrapper) {
  z-index: 20;
}

/* Model 下拉菜单 z-index 低一点 */
:deep(.base-config-select:nth-child(2) .select-dropdown-wrapper) {
  z-index: 19;
}

.speed-input :deep(.input-field) {
  text-align: center;
}
</style>