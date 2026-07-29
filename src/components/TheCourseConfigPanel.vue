<script setup lang="ts">
import { ref, watchEffect, onMounted } from "vue";
import BaseInput from "./common/BaseInput.vue";
import BaseToggle from "./common/BaseToggle.vue";
import { commands, OptionsConfig } from "@/services/cmds.ts";

const options = ref<OptionsConfig>();

onMounted(async () => {
  try {
    const res = await commands.options();
    options.value = res;
  } catch (err) {
    console.error("获取配置失败:", err);
  }
});

watchEffect((onCleanup) => {
  const timer = setTimeout(async () => {
    try {
      await commands.setOptions(options.value!);
      console.log("保存配置成功:", options.value);
    } catch (err) {
      console.error("保存配置失败:", err);
    }
  }, 300);

  onCleanup(() => {
    clearTimeout(timer);
  });
});
</script>

<template>
  <div class="course-config-panel">
    <h2 class="title">Course</h2>
    <div v-if="options" class="settings-container">
      <BaseToggle v-model="options.persistSession" label="Perisist Session" />
      <BaseToggle v-model="options.muteWebview" label="Mute Course Webview" />
      <BaseToggle v-model="options.speedLock" label="Lock Playing Speed" />
      <BaseInput
        v-model.number="options.speedValue"
        label="Playing Speed"
        pattern="\d+(?:\.\d*)?"
        class="speed-input"
      />
    </div>
  </div>
</template>

<style scoped>
.course-config-panel {
  height: 100%;
  flex: 1;
  flex-direction: column;
  display: flex;
}

.settings-container {
  flex: 1;
  display: flex;
  gap: 16px;
  flex-direction: column;
}

.title {
  display: flex;
  height: 48px;
  font-size: 1.5rem;
  align-items: center;
  justify-content: center;
}

:deep(.base-config-select:first-child .select-dropdown-wrapper) {
  z-index: 20;
}

/* Model 下拉菜单 z-index 低一点 */
:deep(.base-config-select:nth-child(2) .select-dropdown-wrapper) {
  z-index: 19;
}

.speed-input :deep(.input-field) {
  text-align: center;
}
</style>
