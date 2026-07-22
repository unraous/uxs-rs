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
  if (providers.value.length === 0) {
    console.error('Provider 列表为空');
    return;
  }

  await invoke('switch_provider', { provider: providers.value[provider.value] });
  const [fetchedModels, currentModel, fetchedApiKey] = await Promise.all([
    invoke('models') as Promise<string[]>,
    invoke('current_model') as Promise<string>,
    invoke('api_key') as Promise<string>
  ]);
  models.value = fetchedModels;
  model.value = fetchedModels.includes(currentModel) ? fetchedModels.indexOf(currentModel) : 0;
  apiKey.value = fetchedApiKey;
};


onMounted(async () => {
  const [fetchedProviders, currentProvider] = await Promise.all([
    invoke('providers') as Promise<string[]>,
    invoke('current_provider') as Promise<string>
  ]);
  providers.value = fetchedProviders;
  
  const targetProviderIndex = fetchedProviders.includes(currentProvider) ? fetchedProviders.indexOf(currentProvider) : 0;
  if (provider.value === targetProviderIndex) {
    await loadProvider();
  } else {
    provider.value = targetProviderIndex;
  }
});

watch(provider, loadProvider);

watch(model, async () => {
  await invoke('switch_model', { model: models.value[model.value] });
});


</script>

<template>
  <div class="api-panel">
    <h2 class="title">API</h2>
    <div class="settings-container">
      <BaseSelecter v-model="provider" label="Provider" :options="providers" />
      <BaseSelecter v-model="model" label="Model" :options="models" />
      <BaseInput v-model="apiKey" label="API Key" @change="invoke('set_key', { key: apiKey })" />
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
</style>