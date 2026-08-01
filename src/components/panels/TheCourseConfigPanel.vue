<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import Input from "@/components/base/Input.vue";
import Toggle from "@/components/base/Toggle.vue";
import { commands, OptionsConfig } from "@/services/cmds.ts";

const options = ref<OptionsConfig>();

const speedValue = computed<number>({
  get() {
    return options.value?.speedValue ?? 1;
  },
  set(val: number) {
    if (options.value) {
      options.value.speedValue = val;
    }
  },
});

onMounted(async () => {
  try {
    const res = await commands.options();
    options.value = res;
  } catch (err) {
    console.error("获取配置失败:", err);
  }
});
</script>

<template>
  <div class="course-config-panel">
    <h2 class="title">Course</h2>
    <div v-if="options" class="settings-container">
      <Toggle v-model="options.persistSession" label="Perisist Session" />
      <Toggle v-model="options.muteWebview" label="Mute Course Webview" />
      <Toggle v-model="options.speedLock" label="Lock Playing Speed" />
      <Input
        id="playing-speed-input"
        v-model.number="speedValue"
        placeholder="input number here"
        label="Playing Speed"
        aria-label="Playing Speed"
        @change="commands.setOptions(options!)"
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
