<script setup lang="ts">
import { Ref, ref } from 'vue';
import MidiInputSelector from '../components/MidiInputSelector.vue';
import MidiOutputSelector from '../components/MidiOutputSelector.vue';
import VirtualKeyboard from '../components/virtual-keyboard/VirtualKeyboard.vue';
import { useMidi } from '../hooks/use-midi';
import { isMidiChannelMessage, MidiMessage } from '../types/midi-message';

const { onMessage } = useMidi();

const eventBuffer: Ref<Array<MidiMessage>> = ref([]);
onMessage((event) => {
    console.log('Received MIDI event:', event);
    eventBuffer.value.unshift(event);
    if (eventBuffer.value.length > 100) {
        eventBuffer.value.pop;
    }
});

</script>

<template>
    <div class="w-full flex flex-col p-4 gap-4 relative">
        <h1>First, connect to your digital instrument</h1>
        <div class="flex flex-row items-center gap-4">
            <MidiInputSelector></MidiInputSelector>
            <MidiOutputSelector></MidiOutputSelector>
        </div>
        <h1 class="text-[var(--color-text-muted)]">Play some notes on the virtual keyboard to check that the output is
            coming
            from
            the desired instrument
        </h1>
        <VirtualKeyboard class="grow shrink-0 max-h-max"></VirtualKeyboard>
        <h1 class="text-[var(--color-text-muted)]">Play some notes on your instrument to verify that the MIDI
            events are
            displayed below</h1>
        <div class="grow flex flex-col rounded border border-[var(--color-outline)] p-2 overflow-auto">
            <div v-for="(event, index) in eventBuffer" :key="index"
                class="flex flex-row text-[var(--color-text-muted)]">
                <div v-if="isMidiChannelMessage(event)">
                    {{ event.channel.channel.toUpperCase() }}: {{ event.channel.message }}
                </div>
            </div>
        </div>
    </div>
</template>