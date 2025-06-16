<script setup lang="ts">
import ViewNavigation from './ViewNavigation.vue';
import MidiMonitor from "../views/MidiMonitor.vue";
import MidiRecorder from "../views/MidiRecorder.vue";
import { useNavigation, View } from '../hooks/use-navigation';

const { currentView, navigateTo } = useNavigation();
const views: Record<View, any> = {
    'setup': MidiMonitor,
    'recorder': MidiRecorder,
};
</script>

<template>
    <div class="flex flex-row h-full w-full">
        <ViewNavigation @navigate="navigateTo"></ViewNavigation>
        <div class="h-full w-[1px] bg-[var(--color-hover)]"></div>
        <component :is="views[currentView]" class="overflow-hidden relative" />
    </div>
</template>
