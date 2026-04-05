<script setup lang="ts">
import xIcon from '@/assets/x.svg';
import mIcon from '@/assets/-.svg';

interface Props {
    onMinimize: () => void;
    onClose: () => Promise<void>;
}

defineProps<Props>();
</script>

<template>
  <div class="menu-bar">
    <button @click="onMinimize">
        <img :src="mIcon" alt="Minimize"/>
    </button>
    <button @click="onClose">
        <img :src="xIcon" alt="Close"/>
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
}

button {
  aspect-ratio: 1;
  border: none;
  border-radius: 0%;
  height: 100%;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>