<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import BaseInput from './common/BaseInput.vue';
import BaseSelecter from './common/BaseSelecter.vue';
import { invoke } from '@tauri-apps/api/core';

const provider = ref(0);
const providers = ref<string[]>([]);
const model = ref(0);
const models = ref<string[]>([]);
const apiKey = ref('');

const loadProvider = async () => {
  await invoke('switch_provider', { provider: providers.value[provider.value] });
  models.value = await invoke('models') as string[];
  const currentModel = await invoke('current_model') as string;
  console.log('Current model:', currentModel);
  model.value = models.value.includes(currentModel) ? models.value.indexOf(currentModel) : 0;
  apiKey.value = await invoke('api_key') as string;
};

const saveConfig = async () => {
  await invoke('set_key', { key: apiKey.value });
};

onMounted(async () => {
  providers.value = await invoke('providers') as string[];
  const currentProvider = await invoke('current_provider') as string;
  provider.value = providers.value.includes(currentProvider) ? providers.value.indexOf(currentProvider) : 0;
  await loadProvider();
});

watch(provider, loadProvider);

watch(model, async () => {
  await invoke('switch_model', { model: models.value[model.value] });
});


defineExpose({ saveConfig });
</script>

<template>
  <div class="api-panel">
    <h2 class="title">API</h2>
    <div class="settings-container">
      <BaseSelecter v-model="provider" label="Provider" :options="providers" />
      <BaseSelecter v-model="model" label="Model" :options="models" />
      <BaseInput v-model="apiKey" label="API Key" />
    </div>
  </div>
</template>

<style scoped>
.api-panel {
  width: 50%;
  height: 100%;
  flex: 1;
  flex-direction: column;
  display: flex;
}

.settings-container {
  flex: 1;
  display: flex;
  gap: 48px;
  flex-direction: column;
  justify-content: center;
}
  
.title {
  display: flex;
  height: 48px;
  font-size: 1.5rem;
  align-items: center;      /* 垂直居中 */
  justify-content: center;   /* 水平居中 */
}


:deep(.base-config-select:first-child .select-dropdown-wrapper) {
  z-index: 20;
}

/* Model 下拉菜单 z-index 低一点 */
:deep(.base-config-select:nth-child(2) .select-dropdown-wrapper) {
  z-index: 19;
}
</style>