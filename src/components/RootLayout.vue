<script setup lang="ts">
import { computed, ref } from 'vue';
import MidiMonitor from '../views/MidiMonitor.vue';
import ViewNavigation from './ViewNavigation.vue';

const views = [MidiMonitor];

const currentViewIndex = ref(0);
const currentView = computed(() => (views[currentViewIndex.value]));

</script>

<template>
    <div class="flex flex-row h-full w-full">
        <ViewNavigation></ViewNavigation>
        <div class="h-full w-[1px] bg-[var(--color-hover)]"></div>
        <component :is="currentView" />
    </div>
</template>
