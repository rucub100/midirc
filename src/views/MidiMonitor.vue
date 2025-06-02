<script setup lang="ts">
import MidiInputSelector from '../components/MidiInputSelector.vue';
import MidiOutputSelector from '../components/MidiOutputSelector.vue';
</script>

<template>
    <div class="flex flex-col p-4 gap-4">
        <h1>First, connect to your digital instrument</h1>
        <div class="flex flex-row items-center gap-4">
            <MidiInputSelector></MidiInputSelector>
            <MidiOutputSelector></MidiOutputSelector>
        </div>
        <h1 class="text-[var(--color-text-muted)]">Play the following example to check that the output is coming
            from
            the desired instrument
        </h1>
        <h1 class="text-[var(--color-text-muted)]">Play some notes on your instrument to verify that the MIDI events are
            displayed below</h1>
        <p>TODO</p>
    </div>
</template>