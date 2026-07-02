<script setup lang="ts">
import MenuBar from '@/components/TheMenuBar.vue';
import TheLeftLayout from './TheLeftLayout.vue';
import TheRightLayout from './TheRightLayout.vue';

</script>

<template>
  <main class="container">
    <MenuBar />
    <div class="main-layout">
      <TheLeftLayout />
      <TheRightLayout />
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

</style>
