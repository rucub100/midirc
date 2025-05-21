<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <main class="flex flex-col items-center justify-center h-screen">
    <h1>Welcome to Tauri + Vue</h1>

    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>
  </main>
</template>

<style scoped>
/* local styles */
</style>

<style>
@import "tailwindcss";

:root {
  /* https://developer.mozilla.org/en-US/docs/Web/CSS/color-scheme */
  color-scheme: light dark;
}
</style>