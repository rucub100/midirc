<script setup lang="ts">
import { computed, PropType } from 'vue';
import IconButton from '../common/IconButton.vue';
import { formatDuration } from '../../helpers/duration-helper';
import { Track } from '../../types/playback';

const props = defineProps({
    track: {
        type: Object as PropType<Track>,
        required: true,
    },
});

const duration = computed(() => { return formatDuration(props.track.durationMilliseconds); });

const emit = defineEmits<{
    (e: 'play'): void
}>();
</script>

<template>
    <div class="flex flex-row w-max items-center border border-[var(--color-outline)] rounded">
        <IconButton icon="play_arrow" @click="emit('play')" class="p-2"></IconButton>
        <h2>Track #{{ track.index + 1 }}</h2>
        <span class="px-4">{{ duration }}</span>
    </div>
</template>
