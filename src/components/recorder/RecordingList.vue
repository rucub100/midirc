<script setup lang="ts">
import { PropType } from 'vue';
import { Recording as RecordingObj } from '../../types/recorder';
import Recording from './Recording.vue';

defineProps({
    recordings: {
        type: Object as PropType<Array<RecordingObj>>,
        required: true,
    },
});

const emit = defineEmits<{
    (e: 'play', index: number): void,
    (e: 'save', index: number): void,
    (e: 'delete', index: number): void,
}>();
</script>

<template>
    <span v-if="recordings.length === 0">No recordings yet.</span>
    <div v-else class="flex flex-col">
        <h1 class="mb-2">Recordings</h1>
        <div class="flex flex-col gap-2">
            <template v-for="recording in recordings" :key="recording.index">
                <Recording :recording="recording" @play="emit('play', recording.index)"
                    @save="emit('save', recording.index)" @delete="emit('delete', recording.index)">
                </Recording>
            </template>
        </div>
    </div>
</template>