<script setup lang="ts">
import { computed, PropType, ref, watch } from 'vue';
import IconButton from '../common/IconButton.vue';
import { Playback } from '../../types/playback';

const props = defineProps({
    playback: {
        type: Object as PropType<Playback>,
        required: true,
    },
});

const progress = ref(0);
const progressTime = computed(() => {
    if (props.playback.state === 'playing' || props.playback.state === 'paused') {
        const absoluteProgress = progress.value * props.playback.durationMilliseconds / 100;
        const minutes = Math.floor(absoluteProgress / 60000);
        const seconds = Math.floor((absoluteProgress % 60000) / 1000);
        return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
    }

    return '00:00';
});
const durationTime = computed(() => {
    if (props.playback.state === 'playing' || props.playback.state === 'paused') {
        const minutes = Math.floor(props.playback.durationMilliseconds / 60000);
        const seconds = Math.floor((props.playback.durationMilliseconds % 60000) / 1000);
        return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
    }

    return '00:00';
});

watch(() => props.playback, (playback, _prevPlayback, onCleanup) => {
    switch (playback.state) {
        case 'playing':
            const startPositionMs = playback.positionMilliseconds;
            const durationMs = playback.durationMilliseconds;
            progress.value = startPositionMs / durationMs * 100;
            const start = performance.now();
            const timer = setInterval(() => {
                if (progress.value < 100) {
                    const newProgress = (startPositionMs + (performance.now() - start)) / durationMs * 100;
                    progress.value = newProgress > 100 ? 100 : newProgress;
                } else {
                    progress.value = 100;
                    setTimeout(() => {
                        emit('end');
                    }, 100);
                    clearInterval(timer);
                }
            }, 100);
            onCleanup(() => clearInterval(timer));
            break;
        case 'paused':
            progress.value = playback.positionMilliseconds / playback.durationMilliseconds * 100;
            break;
        case 'stopped':
            progress.value = 0;
            break;
    }
});

const emit = defineEmits<{
    (e: 'play'): void,
    (e: 'pause'): void,
    (e: 'resume'): void,
    (e: 'stop'): void,
    (e: 'eject'): void,
    (e: 'load'): void,
    (e: 'end'): void,
}>();
</script>

<template>
    <div class="flex flex-row items-center p-2 border border-[var(--color-outline)] w-max rounded">
        <IconButton v-if="playback.state === 'stopped'" icon="play_arrow" class="p-2" @click="emit('play')">
        </IconButton>
        <IconButton v-if="playback.state === 'playing'" icon="pause" class="p-2" @click="emit('pause')"></IconButton>
        <IconButton v-if="playback.state === 'paused'" icon="resume" class="p-2" @click="emit('resume')"></IconButton>
        <IconButton icon="stop" class="p-2" @click="emit('stop')"></IconButton>
        <span class="text-xs text-[var(--color-text-muted)]">{{ progressTime }}</span>
        <div class="w-[200px] flex flex-row items-center p-2 relative">
            <div class="h-[4px] w-full bg-[var(--color-text-muted)] rounded"></div>
            <div class="absolute left-2 h-[4px] bg-[var(--color-primary)] rounded"
                :style="[`width: ${1.8 * progress}px`]">
            </div>
            <span class="material-symbols-sharp absolute z-10 top-[-8px]"
                :class="{ 'animate-pulse': playback.state === 'paused' }"
                :style="[`left: ${1.8 * progress}px`, 'transition: left 100ms linear']">music_note</span>
        </div>
        <span class="text-xs text-[var(--color-text-muted)]">{{ durationTime }}</span>
        <IconButton icon="eject" class="p-2" @click="emit('eject')"></IconButton>
        <IconButton icon="file_open" class="p-2" @click="emit('load')"></IconButton>
    </div>
</template>