<script setup lang="ts">
import MainLayout from "@/layouts/TheMainLayout.vue";
</script>

<template>
  <MainLayout />
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

@font-face {
  font-family: "DefaultFont";
  src: url("@/assets/fonts/Mixture.ttf") format("truetype");
  font-weight: 400;
  font-style: normal;
}

:root {
  user-select: none;
  font-family: "DefaultFont", Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  color: #0d58a4;
  background: linear-gradient(135deg, #e8dcc4 0%, #f0ebe0 100%);

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

button,
input,
select,
textarea {
  font-family: inherit;
  font-size: inherit;
}
</style>
