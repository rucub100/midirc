<script setup lang="ts">
import { confirm } from '@tauri-apps/plugin-dialog';
import RecorderControls from '../components/recorder/RecorderControls.vue';
import PlaybackControls from '../components/recorder/PlaybackControls.vue';
import { usePlayback } from '../hooks/use-playback';
import { useRecorder } from '../hooks/use-recorder';
import RecorderView from '../components/recorder/RecorderView.vue';
import RecordingList from '../components/recorder/RecordingList.vue';
import TrackList from '../components/recorder/TrackList.vue';

const { recorder, startRecording, stopRecording, saveRecording, deleteRecording } = useRecorder();
const { playback, playRecording, pausePlayback, resumePlayback, stopPlayback, updatePlayback, loadTrack, playTrack } = usePlayback();

function handleSaveRecording(index: number) {
    // FIXME: https://vuejs.org/examples/#modal
    // we need to be able to change some properties for the export
    // e.g. tempo, time signature, etc.
    saveRecording(index);
}

function handleDeleteRecording(index: number) {
    confirm(
        'Are you sure you want to delete this recording?',
        { title: 'Delete recording?' }
    ).then((confirmed) => {
        if (confirmed) {
            deleteRecording(index);
        }
    });
}
</script>

<template>
    <div class="w-full flex flex-col p-4 gap-4 relative">
        <div class="flex flex-row gap-4">
            <RecorderControls :state="recorder.state" @start-recording="startRecording" @stop-recording="stopRecording">
            </RecorderControls>
            <PlaybackControls :playback="playback" @pause="pausePlayback" @resume="resumePlayback" @stop="stopPlayback"
                @load="loadTrack" @end="updatePlayback">
            </PlaybackControls>
        </div>
        <RecorderView></RecorderView>
        <div class="flex flex-row gap-4">
            <RecordingList class="flex-1/2" :recordings="recorder.recordings" @play="playRecording"
                @save="handleSaveRecording" @delete="handleDeleteRecording"></RecordingList>
            <TrackList class="flex-1/2" :tracks="playback.tracks" @play="playTrack"></TrackList>
        </div>
    </div>
</template>