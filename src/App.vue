<script setup lang="ts">
import { onMounted } from "vue";
import RootLayout from "./components/RootLayout.vue";
import { showWindow } from "./tauri/window";

onMounted(() => {
  showWindow();
});
</script>

<template>
  <RootLayout></RootLayout>
</template>
