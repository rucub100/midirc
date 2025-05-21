<script setup lang="ts">
import { ref } from 'vue';

const selectedInstrument = ref<string>("");
</script>

<template>
    <div class="flex flex-col">
        <div class="flex flex-row p-4">
            <select v-model="selectedInstrument">
                <option value="">--Please select your instrument--</option>
            </select>
            <button class="flex items-center justify-center cursor-pointer p-1 hover:bg-[rgba(127,127,127,0.1)]">
                <span class="material-symbols-sharp">refresh</span>
            </button>
        </div>
    </div>
</template>