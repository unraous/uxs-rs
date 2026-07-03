<script setup lang="ts">
import { ref } from 'vue';
import BaseInput from './common/BaseInput.vue';
import BaseSelecter from './common/BaseSelecter.vue';

const provider = ref(0); // 默认选择 OpenAI
const model = ref('');
const apiKey = ref('');
const providers = [
  "bigmodel",
  "deepseek",
  "google",
  "moonshot",
  "ollama",
  "openai",
  "openrouter",
]
</script>

<template>
  <div class="api-panel">
    <h2 class="title">API</h2>
    <div class="settings-container">
      <BaseSelecter v-model="provider" label="Provider" :options="providers" />
      <BaseInput v-model="model" label="Model" />
      <BaseInput v-model="apiKey" label="API Key" />
    </div>
  </div>
</template>

<style scoped>
.api-panel {
  padding: 20px;
  width: 50%;
  height: 100%;
  flex: 1;
  flex-direction: column;
  display: flex;
}

.settings-container {
  flex: 1;
  display: flex;
  gap: 10px;
  flex-direction: column;
  justify-content: center;
}
  
.title {
    font-size: 1.5rem;
    margin-bottom: 10px;
    text-align: center;
}

</style>