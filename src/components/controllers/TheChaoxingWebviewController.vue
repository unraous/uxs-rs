<script setup lang="ts">
import zoomInIconRaw from "@/assets/zoom_in.svg?raw";
import zoomOutIconRaw from "@/assets/zoom_out.svg?raw";
import arrowIconRaw from "@/assets/arrow_forward.svg?raw";
import refreshIconRaw from "@/assets/refresh.svg?raw";
import { ref, computed, onMounted, watch } from "vue";
import { listen } from "@tauri-apps/api/event";
import Button from "@/components/base/Button.vue";
import { commands } from "@/services/cmds";

// Webview 缩放控制阶梯表
const ZOOM_LEVELS = [
  0.25, 0.33, 0.5, 0.67, 0.75, 0.8, 0.9, 1.0, 1.1, 1.25, 1.5, 1.75, 2.0, 2.5,
  3.0, 4.0, 5.0,
];

const zoomIndex = ref(7); // index to 100%

/** 当前响应式缩放倍率数值 */
const currentScale = computed(() => ZOOM_LEVELS[zoomIndex.value] ?? 1.0);

/** 当前响应式缩放百分比文本 (如 '100%') */
const currentZoomText = computed(
  () => `${Math.round(currentScale.value * 100)}%`,
);

const canZoomIn = computed(() => zoomIndex.value < ZOOM_LEVELS.length - 1);
const canZoomOut = computed(() => zoomIndex.value > 0);

watch(zoomIndex, async (newIndex) => {
  await commands.setZoom(ZOOM_LEVELS[newIndex]).catch((e) => {
    console.error("设置Webview缩放失败", e);
  });
});

const zoomIn = () => {
  zoomIndex.value++;
};
const zoomOut = () => {
  zoomIndex.value--;
};

const currentUrl = ref("等待页面加载...");
const canGoBack = ref(false);
const canGoForward = ref(false);

const updateNavState = async () => {
  try {
    const [back, forward] = await Promise.all([
      commands.canGoBack(),
      commands.canGoForward(),
    ]);
    canGoBack.value = back;
    canGoForward.value = forward;
  } catch (err) {
    console.error("获取导航状态失败:", err);
  }
};

onMounted(async () => {
  await Promise.all([updateNavState(), commands.reload()]);
  await listen("url-update", async (event) => {
    currentUrl.value = event.payload as string;
    await updateNavState();
  });
});
</script>

<template>
  <div class="body">
    <div class="navigation">
      <Button
        :icon-raw="arrowIconRaw"
        @click="commands.goBack"
        :disabled="!canGoBack"
        color="#ebe2cf"
        shape="circle"
        size="2rem"
        variant="translucent"
        class="back-btn"
      />
      <Button
        :icon-raw="arrowIconRaw"
        @click="commands.goForward"
        :disabled="!canGoForward"
        color="#ebe2cf"
        shape="circle"
        size="2rem"
        variant="translucent"
      />
      <Button
        :icon-raw="refreshIconRaw"
        @click="commands.reload"
        color="#ebe2cf"
        shape="circle"
        size="2rem"
        variant="translucent"
      />
    </div>
    <div class="capsule-slot">
      <p class="url-text">{{ currentUrl }}</p>
      <Button
        :icon-raw="zoomOutIconRaw"
        @click="zoomOut"
        :disabled="!canZoomOut"
        color="#0d58a4"
        shape="circle"
        size="1.75rem"
        variant="translucent"
      />
      <span class="zoom-value">{{ currentZoomText }}</span>
      <Button
        :icon-raw="zoomInIconRaw"
        @click="zoomIn"
        :disabled="!canZoomIn"
        color="#0d58a4"
        shape="circle"
        size="1.75rem"
        variant="translucent"
      />
    </div>
    <div class="title">
      <p>课程页面</p>
    </div>
  </div>
</template>
<style scoped>
.body {
  width: 100%;
  height: 9.09%;
  background-color: #0d58a4;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: flex-end;
  padding: 2%;
}

.navigation {
  width: 12%;
  height: 90%;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  margin-right: auto;
}

:deep(.navigation .icon-raw) {
  transform: scale(1.25);
}

:deep(.back-btn .icon-raw) {
  transform: rotate(180deg) scale(1.25);
}

.url-text {
  width: 70%;
  overflow: hidden;
  text-overflow: ellipsis; /* 超出部分显示省略号 ... */
  white-space: nowrap; /* 强制单行，禁止换行 */
  font-size: 1rem;
  color: #0b4c8d;
  margin-right: auto;
}

.zoom-value {
  width: 7.5%;
  text-align: center;
  font-size: 0.85rem;
}

.capsule-slot {
  height: 75%; /* 垂直方向贴合顶栏高 */
  width: 70%; /* 胶囊最小宽度 */
  background: linear-gradient(
    135deg,
    #e8dcc4 0%,
    #f0ebe0 100%
  ); /* 镂空露出的浅底色（与主页面背景一致） */
  border-radius: 999px; /* 完美胶囊圆角 */
  display: flex;
  align-items: center;
  overflow: hidden;
  padding: 2%;
  gap: 8px;
  border: 2px solid #0b4c8d;
  box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.25); /* 核心：内阴影打造沉降镂空质感 */
}

.title {
  font-size: larger;
  width: 12.5%;
  color: #ebe2cf;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
