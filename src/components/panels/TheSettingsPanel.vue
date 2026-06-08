<script setup lang="ts">
import { ref } from 'vue';
import BaseButton from '../common/BaseButton.vue';
import BaseTextBox from '../common/BaseTextBox.vue';

const userName = ref('');
</script>

<template>
  <div class="setting-panel">
    <h2>设置面板</h2>
    <p>这里是设置面板的内容。</p>
    <BaseButton>
    </BaseButton>
    <BaseTextBox v-model="userName" label="用户名" />
  </div>
</template>

<style scoped>
.setting-panel {
  flex: 1;
}

</style>
