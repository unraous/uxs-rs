<script setup lang="ts">
import MenuBar from './layouts/MenuBar.vue';
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';

const isClosing = ref(false);

const minimizeApp = () => {
    invoke('minimize_app').catch(e => {
        console.error('最小化失败:', e);
    });
};

const closeApp = async () => {
    invoke('close_app').catch(e => {
        console.error('关闭应用失败:', e);
    });
};
</script>

<template>
  <div class="mask"></div>
  <main class="container" :class="{ closing: isClosing }">
    <!-- Glitch 遮罩 - 覆盖整个视口（暂时停用） -->
    <div class="glitch-overlay"></div>

    <MenuBar :on-close="closeApp" :on-minimize="minimizeApp" />
    <div class="main-layout">
      <div class="left-panel">
      </div>
      <div class="right-panel">
          <div class="cx-control-panel">
            <h1>超星控制面板(占位)</h1>
            <p>这里是超星控制面板，由Vue组件实现。</p>
          </div>
          <div class="chaoxing-webview">
          </div>
      </div>
    </div>
  </main>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  background: linear-gradient(135deg, #e8dcc4 0%, #f0ebe0 100%);
  
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  display: flex;
  height: 100vh;
  width: 100vw;
  flex-direction: column;
  position: relative;
}

.mask {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: 9999;
  background-image: url('@/assets/noise_texture.png');
  mix-blend-mode: soft-light;
  pointer-events: none;
  opacity: 0.5;
}

@keyframes fadeOut {
  from {
    opacity: 1;
  }
  to {
    opacity: 0;
  }
}

/* Glitch 遮罩 */
.glitch-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: 9999;
  pointer-events: none;  /* 不阻止下方元素交互 */
  mix-blend-mode: screen;  /* 可选：改变混合模式，adjust为 multiply/overlay/screen 等 */
}

.main-layout {
  flex: 1;
  display: flex;
  flex-direction: row;
}

.left-panel {
  flex: 0 0 50%;
  background-color: transparent;
}

.right-panel {
  flex: 1;
  display: flex;
  align-items: center;
  flex-direction: column;
  background-color: transparent;
}

.cx-control-panel {
  padding: 40px;
  width: 100%;
  height: 43.75%;
  background-color: transparent;
  display: flex;
  align-items: center;
  flex-direction: column;
}

.chaoxing-webview {
  padding: 20px;
  width: 96%;
  height: 50%;
  background-color: #F6F8FA;
}
</style>