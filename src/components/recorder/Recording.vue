<script setup lang="ts">
import { computed, PropType } from 'vue';
import IconButton from '../common/IconButton.vue';
import { Recording } from '../../types/recorder';
import { formatDuration } from '../../helpers/duration-helper';

const props = defineProps({
    recording: {
        type: Object as PropType<Recording>,
        required: true,
    },
});

const duration = computed(() => { return formatDuration(props.recording.durationMilliseconds); });

const emit = defineEmits<{
    (e: 'play'): void
}>();
</script>

<template>
    <div class="flex flex-row w-max items-center border border-[var(--color-outline)] rounded">
        <IconButton icon="play_arrow" @click="emit('play')" class="p-2"></IconButton>
        <h2>Recording #{{ recording.index + 1 }}</h2>
        <span class="px-4">{{ duration }}</span>
        <IconButton icon="file_save"></IconButton>
        <IconButton icon="delete" class="px-2"></IconButton>
    </div>
</template>