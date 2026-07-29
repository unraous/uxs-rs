<script setup lang="ts">
import ConfigPanel from '@/components/TheConfigPanel.vue';

defineProps<{
  version: string,
  author: string,
}>();

</script>

<template>
  <div class="right-container">
        <ConfigPanel />
        <!-- 
          Placeholder element for the Chaoxing WebView container.
          Note: This element remains empty because Tauri's native child WebView window 
          is dynamically positioned and overlayed according to the layout bounds and 
          geometry of this placeholder (styled by .chaoxing-webview).
          Therefore, no child nodes or Vue components are rendered within this element.
        -->
        <div class="chaoxing-webview"></div>
        <div class="version-info">
          <p>by {{ author }} v{{ version }}</p>
        </div>
    </div>
</template>

<style scoped>

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