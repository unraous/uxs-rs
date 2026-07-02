<script setup lang="ts">
import { ref } from 'vue';
import BaseButton from '../common/BaseButton.vue';
import BaseConfigInput from '../common/BaseConfigInput.vue';
import BaseConfigToggle from '../common/BaseConfigToggle.vue';

const userName = ref('');
const isActivated = ref(false);

</script>

<template>
  <div class="setting-panel">
    <h1 class="title">Configuration</h1>
    <BaseConfigInput v-model="userName" label="用户名" />
    <BaseConfigToggle v-model="isActivated" label="激活状态" />
  </div>
</template>

<style scoped>
.title {
  width: 100%;
  font-size: 2rem;
  margin-bottom: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.setting-panel {
  flex: 1;
  flex-direction: column;
}
</style>
