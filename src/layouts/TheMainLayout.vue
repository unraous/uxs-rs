<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref, onMounted } from 'vue';
import MenuBar from '../components/common/TheMenuBar.vue';
import ConfigPanel from '../components/panels/TheConfigPanel.vue';

const version = ref('2.0.0');
const author = ref('unraous');

onMounted(async () => {
  try {
    version.value = await invoke<string>('metadata', { key: 'version' });
    author.value = await invoke<string>('metadata', { key: 'author' });
  } catch (e) {
    console.error("加载元数据失败，使用默认值", e);
  }
});
</script>

<template>
  <main class="container">
    <MenuBar />
    <div class="main-layout">
      <div class="left-container">
      </div>
      <div class="right-container">
        <ConfigPanel />
        <div class="chaoxing-webview"></div>
        <div class="version-info">
          <p>by {{ author }} v{{ version }}</p>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
.container {
  display: flex;
  height: 100vh;
  width: 100vw;
  flex-direction: column;
  position: relative;
}

.main-layout {
  flex: 1;
  display: flex;
  flex-direction: row;
}

.left-container {
  flex: 0 0 50%;
  background-color: yellow;
}

.right-container {
  flex: 1;
  display: flex;
  align-items: center;
  flex-direction: column;
  background-color: transparent;
}

.chaoxing-webview {
  padding: 20px;
  width: 96%;
  height: 50%;
  background-color: #F6F8FA;
  outline: 6px solid #0d58a4;
}

.version-info {
  padding-right: 40px;
  height: 6.25%;
  width: 100%;
  background-color: transparent;
  display: flex;
  align-items: center;
  justify-content: flex-end;
}
</style>
