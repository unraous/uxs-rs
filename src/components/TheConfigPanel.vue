<script setup lang="ts">
import { commands } from '@/services/cmds';

</script>


<template>
  <div class="body">
    <div class="config">
      <h2>信息面板（占位）</h2>
      <p>这里是信息面板，由Vue组件实现。</p>
    </div>
    <div class="tips">
      <h2>功能设置（占位）</h2>
      <p>
        <button @click="commands.goBack">后退</button>
        <button @click="commands.goForward">前进</button>
        <button @click="commands.setZoom(0.5)">缩小</button>
        <button @click="commands.setZoom(1.5)">放大</button>
      </p>
    </div>
  </div>
</template>

<style scoped>
.body {
  padding: 20px;
  width: 100%;
  height: 43.75%;
  background-color: transparent;
  display: flex;
  align-items: center;
  flex-direction: column;
}

.config {
  padding: 20px;
  width: 100%;
  height: 80%;
  background-color: #1d784d;
}

.tips {
  padding: 20px;
  width: 100%;
  height: 20%;
  background-color: #f6f8fa;
}
</style>
