<script setup lang="ts">
import RecorderControls from '../components/recorder/RecorderControls.vue';
import PlaybackControls from '../components/recorder/PlaybackControls.vue';
import Recording from '../components/recorder/Recording.vue';
import { usePlayback } from '../hooks/use-playback';
import { useRecorder } from '../hooks/use-recorder';

const { recorder, startRecording, stopRecording } = useRecorder();
const { playback, playRecording, pausePlayback, resumePlayback, stopPlayback } = usePlayback();

// TODO: track the playback state with interval and make sure to cleanup (onMounted/onUnmounted)
</script>

<template>
    <div class="w-full flex flex-col p-4 gap-4 relative">
        <div class="flex flex-row gap-4">
            <RecorderControls :state="recorder.state" @start-recording="startRecording" @stop-recording="stopRecording">
            </RecorderControls>
            <PlaybackControls :state="playback.state" @pause="pausePlayback" @resume="resumePlayback"
                @stop="stopPlayback"></PlaybackControls>
        </div>
        <div class="flex flex-col">
            <h1 class="mb-2">Recordings</h1>
            <div class="flex flex-col gap-2">
                <template v-for="(_recording, index) in recorder.recordings" :key="index">
                    <Recording :index="index" @play="playRecording(index)"></Recording>
                </template>
            </div>
        </div>
    </div>
</template>