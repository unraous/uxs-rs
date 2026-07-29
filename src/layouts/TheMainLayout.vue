<script setup lang="ts">
import MenuBar from '@/components/TheMenuBar.vue';
import TheLeftLayout from './TheLeftLayout.vue';
import TheRightLayout from './TheRightLayout.vue';
import { onMounted, ref } from 'vue';
import { commands, MetadataConfig } from '@/services/cmds.ts';

const metadata = ref<MetadataConfig>();

onMounted(async () => {
  metadata.value = await commands.metadata();
});
</script>

<template>
  <main class="container">
    <MenuBar :app-title="metadata?.title ?? 'backend error'" />
    <div class="main-layout">
      <TheLeftLayout />
      <TheRightLayout 
        :author="metadata?.author ?? 'backend error'"
        :version="metadata?.version ?? '0.0.0'"
      />
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
