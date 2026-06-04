<script setup lang="ts">
const onClickSettingButton = () => {
  alert('这是一个设置按钮');
};


</script>

<template>
  <div class="setting-panel">
    <h2>设置面板</h2>
    <p>这里是设置面板的内容。</p>
    <button @click="onClickSettingButton">
      <span>点击设置</span>
    </button>
  </div>
</template>

<style scoped>
.setting-panel {
  flex: 1;
  background-color: lightgray;
}

</style>
