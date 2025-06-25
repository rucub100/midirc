<script setup lang="ts">
import { PropType } from 'vue';
import IconButton from '../common/IconButton.vue';
import { Playback } from '../../types/playback';

defineProps({
    state: {
        type: String as PropType<Playback['state']>,
        required: true,
    },
});

const emit = defineEmits<{
    (e: 'play'): void,
    (e: 'pause'): void,
    (e: 'resume'): void,
    (e: 'stop'): void,
    (e: 'eject'): void,
    (e: 'load'): void,
}>();
</script>

<template>
    <div class="flex flex-row p-2 border border-[var(--color-outline)] w-max rounded">
        <IconButton v-if="state === 'stopped'" icon="play_arrow" class="p-2" @click="emit('play')"></IconButton>
        <IconButton v-if="state === 'playing'" icon="pause" class="p-2" @click="emit('pause')"></IconButton>
        <IconButton v-if="state === 'paused'" icon="resume" class="p-2" @click="emit('resume')"></IconButton>
        <IconButton v-if="state !== 'stopped'" icon="stop" class="p-2" @click="emit('stop')"></IconButton>
        <IconButton icon="eject" class="p-2" @click="emit('eject')"></IconButton>
        <IconButton icon="file_open" class="p-2" @click="emit('load')"></IconButton>
    </div>
</template>