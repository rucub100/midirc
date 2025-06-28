<script setup lang="ts">
import RecorderControls from '../components/recorder/RecorderControls.vue';
import PlaybackControls from '../components/recorder/PlaybackControls.vue';
import Recording from '../components/recorder/Recording.vue';
import { usePlayback } from '../hooks/use-playback';
import { useRecorder } from '../hooks/use-recorder';
import RecorderView from '../components/recorder/RecorderView.vue';

const { recorder, startRecording, stopRecording } = useRecorder();
const { playback, playRecording, pausePlayback, resumePlayback, stopPlayback, updatePlayback } = usePlayback();
</script>

<template>
    <div class="w-full flex flex-col p-4 gap-4 relative">
        <div class="flex flex-row gap-4">
            <RecorderControls :state="recorder.state" @start-recording="startRecording" @stop-recording="stopRecording">
            </RecorderControls>
            <PlaybackControls :playback="playback" @pause="pausePlayback" @resume="resumePlayback" @stop="stopPlayback"
                @end="updatePlayback">
            </PlaybackControls>
        </div>
        <RecorderView></RecorderView>
        <span v-if="recorder.recordings.length === 0">No recordings yet.</span>
        <div v-else class="flex flex-col">
            <h1 class="mb-2">Recordings</h1>
            <div class="flex flex-col gap-2">
                <template v-for="recording in recorder.recordings" :key="index">
                    <Recording :recording="recording" @play="playRecording(recording.index)"></Recording>
                </template>
            </div>
        </div>
    </div>
</template>