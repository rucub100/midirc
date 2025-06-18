<script setup lang="ts">
import IconButton from '../common/IconButton.vue';
import { useRecorder } from '../../hooks/use-recorder';
import { computed } from 'vue';

const { recorder, startRecording, stopRecording } = useRecorder();
const state = computed(() => recorder.value.state);
</script>

<template>
    <div class="flex flex-row p-2 border border-[var(--color-outline)] w-max rounded">
        <IconButton v-if="state !== 'stopped'" icon="stop" class="p-2" @click="stopRecording">
        </IconButton>
        <IconButton v-if="state === 'stopped'" icon="fiber_manual_record" class="p-2" @click="startRecording">
        </IconButton>
        <!-- <IconButton v-else-if="state === 'recording'" icon="pause" class="p-2" @click="pauseRecording">
        </IconButton>
        <IconButton v-else-if="state === 'paused'" icon="resume" class="p-2" @click="resumeRecording">
        </IconButton> -->
        <!-- <IconButton icon="play_arrow" class="p-2"></IconButton>
        <IconButton icon="eject" class="p-2"></IconButton>
        <IconButton icon="file_open" class="p-2"></IconButton>
        <IconButton icon="file_save" class="p-2"></IconButton> -->
    </div>
</template>