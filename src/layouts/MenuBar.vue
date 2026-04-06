<script setup lang="ts">
import xIconRaw from '@/assets/x.svg?raw';
import mIconRaw from '@/assets/-.svg?raw';
import { invoke } from '@tauri-apps/api/core';

const appTitle = 'uxuescript'; 

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
  <div class="menu-bar">
    <div class="title">
      {{ appTitle }}
    </div>
    <button @click="minimizeApp">
        <span v-html="mIconRaw" class="icon"></span>
    </button>
    <button @click="closeApp">
        <span v-html="xIconRaw" class="icon"></span>
    </button>
  </div>
</template>

<style scoped>
.menu-bar {
  height: 4%;
  background-color: transparent;
  display: flex;
  flex-direction: row;
  justify-content: flex-end;
  position: relative;
}

.title {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  color: #0d58a4;
  font-size: 30px;
}

button {
  aspect-ratio: 1;
  border: none;
  border-radius: 0%;
  background-color: transparent;
  color: #0d58a4;
  height: 100%;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.25s ease-out;
}

/* 悬停状态 */
button:hover {
  background-color: #0d58a4;
  color: #ede5d5;
}
</style>