<script setup lang="ts">
import { ref } from 'vue';
import BaseButton from '../common/BaseButton.vue';
import BaseConfigInput from '../common/BaseConfigInput.vue';
import BaseConfigRadio from '../common/BaseConfigRadio.vue';

const userName = ref('');
const isActivated = ref(false);

</script>

<template>
  <div class="setting-panel">
    <h2>设置面板</h2>
    <p>这里是设置面板的内容。</p>
    <BaseButton>
    </BaseButton>
    <BaseConfigInput v-model="userName" label="用户名" />
    <BaseConfigRadio v-model="isActivated" label="激活状态" />
  </div>
</template>

<style scoped>
.setting-panel {
  flex: 1;
}

</style>
