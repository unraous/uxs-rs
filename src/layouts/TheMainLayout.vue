<script setup lang="ts">
import MenuBar from '@/components/common/TheMenuBar.vue';
import TheLeftView from '@/views/TheLeftView.vue';
import TheRightView from '@/views/TheRightView.vue';

</script>

<template>
  <main class="container">
    <MenuBar />
    <div class="main-layout">
      <TheLeftView />
      <TheRightView />
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
