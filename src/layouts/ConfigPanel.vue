<script setup lang="ts">

</script>

<template>
  <div class="body">
    <div class="config">
      <h2>信息面板（占位）</h2>
      <p>这里是信息面板，由Vue组件实现。</p>
    </div>
    <div class="tips">
      <h2>功能设置（占位）</h2>
      <p>这里是功能设置区域，可以放置各种开关和选项。</p>
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
  background-color: #F6F8FA;
}
</style>