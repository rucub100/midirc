<script setup lang="ts">
import MidiInputSelector from '../components/MidiInputSelector.vue';
import MidiOutputSelector from '../components/MidiOutputSelector.vue';
</script>

<template>
    <div class="flex flex-col">
        <div class="flex flex-row items-center p-4 gap-4">
            <MidiInputSelector></MidiInputSelector>
            <MidiOutputSelector></MidiOutputSelector>
        </div>
    </div>
</template>