<script setup lang="ts">
// import { ref } from "vue";
// import { invoke } from "@tauri-apps/api/core";
import RootLayout from "./components/RootLayout.vue";

// const greetMsg = ref("");
// const name = ref("");

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsg.value = await invoke("greet", { name: name.value });
// }
</script>

<template>
  <RootLayout></RootLayout>
</template>

<style scoped>
/* local styles */
</style>

<style>
@import "tailwindcss";

html, body, #app {
  height: 100vh;
  min-height: 100vh;
  max-height: 100vh;
  width: 100%;
  min-width: 100%;
  max-width: 100%;
  overflow: hidden;
}

:root {
  /* https://developer.mozilla.org/en-US/docs/Web/CSS/color-scheme */
  /* color-scheme: light dark; */
}

@font-face {
  font-family: 'Material Symbols Sharp';
  font-style: normal;
  src: url(assets/MaterialSymbolsSharp[FILL,GRAD,opsz,wght].woff2) format('woff2');
}

.material-symbols-sharp {
  font-family: 'Material Symbols Sharp';
  font-variation-settings:
  'FILL' 1,
  'wght' 400,
  'GRAD' 0,
  'opsz' 24;
  font-weight: normal;
  font-style: normal;
  font-size: 24px;
  display: inline-block;
  line-height: 1;
  text-transform: none;
  letter-spacing: normal;
  word-wrap: normal;
  white-space: nowrap;
  direction: ltr;
}
</style>
