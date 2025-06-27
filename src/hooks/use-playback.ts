import { ref } from "vue";
import {
  getMidiPlayback,
  pauseMidiPlayback,
  playMidiRecording,
  resumeMidiPlayback,
  stopMidiPlayback,
} from "../tauri/playback-commands";
import { Playback } from "../types/playback";

const defaultPlayback: Playback = {
  state: "stopped",
  positionMilliseconds: 0,
};

const globalPlayback = ref<Playback>(defaultPlayback);

getMidiPlayback()
  .then((playback) => {
    globalPlayback.value = playback;
  })
  .catch((error) => {
    console.error("Error fetching initial playback state:", error);
  });

function updatePlayback() {
  getMidiPlayback()
    .then((playback) => {
      globalPlayback.value = playback;
    })
    .catch((error) => {
      console.error("Error fetching initial playback state:", error);
    });
}

function playRecording(index: number) {
  playMidiRecording(index)
    .then((playback) => {
      globalPlayback.value = playback;
    })
    .catch((error) => {
      console.error("Error playing MIDI recording:", error);
    });
}

function pausePlayback() {
  pauseMidiPlayback()
    .then((playback) => {
      globalPlayback.value = playback;
    })
    .catch((error) => {
      console.error("Error pausing MIDI playback:", error);
    });
}

function resumePlayback() {
  resumeMidiPlayback()
    .then((playback) => {
      globalPlayback.value = playback;
    })
    .catch((error) => {
      console.error("Error resuming MIDI playback:", error);
    });
}

function stopPlayback() {
  stopMidiPlayback()
    .then((playback) => {
      globalPlayback.value = playback;
    })
    .catch((error) => {
      console.error("Error stopping MIDI playback:", error);
    });
}

export function usePlayback() {
  return {
    playback: globalPlayback,
    updatePlayback,
    playRecording,
    pausePlayback,
    resumePlayback,
    stopPlayback,
  };
}
