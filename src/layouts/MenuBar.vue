<script setup lang="ts">
import xIconRaw from '@/assets/x.svg?raw';
import mIconRaw from '@/assets/-.svg?raw';

interface Props {
    onMinimize: () => void;
    onClose: () => Promise<void>;
}

defineProps<Props>();
</script>

<template>
  <div class="menu-bar">
    <button @click="onMinimize">
        <span v-html="mIconRaw" class="icon"></span>
    </button>
    <button @click="onClose">
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
}

button {
  aspect-ratio: 1;
  border: none;
  border-radius: 0%;
  color: #5c3014;
  height: 100%;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* 平常状态 */
.icon {
  display: inline-flex;
  transition: filter 0.3s ease;
}

.icon svg {
  width: 100%;
  height: 100%;
  fill: #5c3014;
}

button:hover {
  background-color: #000000;
}

/* 悬停状态 */
button:hover .icon svg {
  fill: #8b4513;
}
</style>