<script setup lang="ts">
import MenuBar from '@/layouts/MenuBar.vue';
import ConfigPanel from '@/layouts/ConfigPanel.vue';

</script>

<template>
  <main class="container">
    <MenuBar/>
    <div class="main-layout">
      <div class="left-panel">
      </div>
      <div class="right-panel">
        <ConfigPanel/>
        <div class="chaoxing-webview">
        </div>
        <div class="version-info">
          <p>by unraous v2.0.0</p>
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
  color: #0d58a4;
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

.glitch-mask {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: 9999;
  pointer-events: none;
  mix-blend-mode: screen;
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