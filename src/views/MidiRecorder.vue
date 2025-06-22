<script setup lang="ts">
import IconButton from '../components/common/IconButton.vue';
import RecorderControls from '../components/recorder/RecorderControls.vue';
import { usePlayback } from '../hooks/use-playback';
import { useRecorder } from '../hooks/use-recorder';

const { recorder } = useRecorder();
const { playRecording } = usePlayback();
</script>

<template>
    <div class="w-full flex flex-col p-4 gap-4 relative">
        <RecorderControls></RecorderControls>
        <div class="flex flex-col">
            <h1 class="mb-2">Recordings</h1>
            <div class="flex flex-col gap-2">
                <div v-for="(_recording, index) in recorder.recordings" :key="index"
                    class="flex flex-row items-center border border-[var(--color-outline)] rounded p-2 ">
                    <IconButton icon="play_arrow" class="p-2" @click="playRecording(index)"></IconButton>
                    <h2>Recording {{ index }}</h2>
                </div>
            </div>
        </div>
    </div>
</template>