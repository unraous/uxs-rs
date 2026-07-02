<script setup lang="ts">
import { ref } from 'vue';
import BaseConfigInput from './common/BaseConfigInput.vue';

const provider = ref('');
const model = ref('');
const apiKey = ref('');
</script>
<template>
  <div class="api-panel">
    <h2 class="title">API</h2>
    <div class="settings-container">
      <BaseConfigInput v-model="provider" label="Provider" />
      <BaseConfigInput v-model="model" label="Model" />
      <BaseConfigInput v-model="apiKey" label="API Key" />
    </div>
  </div>
</template>

<style scoped>
.api-panel {
  padding: 20px;
  width: 50%;
  height: 100%;
  background-color: #f0f0f0;
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
  
.bottom-space {
  width: 100%;
  height: 20%;
  background-color: tomato;
}

.title {
    font-size: 1.5rem;
    margin-bottom: 10px;
    text-align: center;
}

</style>