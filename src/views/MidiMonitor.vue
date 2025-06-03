<script setup lang="ts">
import IconButton from '../components/common/IconButton.vue';
import MidiInputSelector from '../components/MidiInputSelector.vue';
import MidiOutputSelector from '../components/MidiOutputSelector.vue';
import { useMidi } from '../hooks/use-midi';

const { playDemo } = useMidi();

function handlePlayDemo() {
    console.log('Playing demo...');
    playDemo();
}
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
        <IconButton icon="play_circle" title="Play Example" @click="handlePlayDemo"></IconButton>
        <h1 class="text-[var(--color-text-muted)]">Play some notes on your instrument to verify that the MIDI events are
            displayed below</h1>
        <p>TODO</p>
    </div>
</template>