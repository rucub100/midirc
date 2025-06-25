<script setup lang="ts">
import { PropType } from 'vue';

import IconButton from '../common/IconButton.vue';
import { Recorder } from '../../types/recorder';

defineProps({
    state: {
        type: String as PropType<Recorder['state']>,
        required: true,
    },
});

const emit = defineEmits<{
    (e: 'startRecording'): void,
    (e: 'stopRecording'): void,
}>();
</script>

<template>
    <div class="flex flex-row p-2 border border-[var(--color-outline)] w-max rounded">
        <IconButton v-if="state !== 'stopped'" icon="stop" class="p-2" @click="emit('stopRecording')">
        </IconButton>
        <IconButton v-if="state === 'stopped'" icon="fiber_manual_record" class="p-2" @click="emit('startRecording')">
        </IconButton>
    </div>
</template>