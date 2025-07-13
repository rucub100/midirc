<script setup lang="ts">
import { PropType } from 'vue';
import { Track as TrackObj } from '../../types/playback';
import Track from './Track.vue';

defineProps({
    tracks: {
        type: Object as PropType<Array<TrackObj>>,
        required: true,
    },
});

const emit = defineEmits<{
    (e: 'play', index: number): void,
}>();
</script>

<template>
    <span v-if="tracks.length === 0">No MIDI tracks yet.</span>
    <div v-else class="flex flex-col">
        <h1 class="mb-2">MIDI Tracks</h1>
        <div class="flex flex-col gap-2">
            <template v-for="track in tracks" :key="track.index">
                <Track :track="track" @play="emit('play', track.index)">
                </Track>
            </template>
        </div>
    </div>
</template>