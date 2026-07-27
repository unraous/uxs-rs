<script setup lang="ts">
import { ref, watchEffect, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import BaseInput from './common/BaseInput.vue';
import BaseToggle from './common/BaseToggle.vue';

interface OptionConfig {
  persistSession: boolean,
  muteWebview: boolean,
  speedLock: boolean,
  speedValue: number,
}

const DEFAULT_OPTION: OptionConfig = {
  persistSession: true,
  muteWebview: true,
  speedLock: false,
  speedValue: 2.0,
}

const options = ref<OptionConfig>(DEFAULT_OPTION);

onMounted(async () => {
  try {
    const res = await invoke<OptionConfig>('options');
    options.value = res;
  } catch (err) {
    console.error('获取配置失败:', err);
  }
});

watchEffect((onCleanup) => {
  const configPayload = { ...options.value };

  const timer = setTimeout(async () => {
    try {
      await invoke('set_options', { options: configPayload });
      console.log('保存配置成功:', configPayload);
    } catch (err) {
      console.error('保存配置失败:', err);
    }
  }, 300);

  onCleanup(() => {
    clearTimeout(timer);
  });
});

</script>

<template>
  <div class="course-config-panel">
    <h2 class="title">Course</h2>
    <div class="settings-container">
      <BaseToggle v-model="options.persistSession" label="Perisist Session" />
      <BaseToggle v-model="options.muteWebview" label="Mute Course Webview" />
      <BaseToggle v-model="options.speedLock" label="Lock Playing Speed" />
      <BaseInput v-model.number="options.speedValue" label="Playing Speed" pattern="\d+(?:\.\d*)?" class="speed-input" />
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