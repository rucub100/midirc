import { ref } from "vue";
import {
  getMidiPlayback,
  pauseMidiPlayback,
  playMidiRecording,
  resumeMidiPlayback,
  stopMidiPlayback,
  loadMidiTrack,
  playMidiTrack,
  ejectMidiTrack,
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

function loadTrack() {
  loadMidiTrack()
    .then((playback) => {
      console.log("Track loaded:", playback);
      globalPlayback.value = playback;
    })
    .catch((error) => {
      console.error("Error loading MIDI track:", error);
    });
}

function ejectTrack(index: number) {
  ejectMidiTrack(index)
    .then((playback) => {
      console.log("Track ejected:", playback);
      globalPlayback.value = playback;
    })
    .catch((error) => {
      console.error("Error ejecting MIDI track:", error);
    });
}

function playRecording(index: number) {
  playMidiRecording(index)
    .then((playback) => {
      console.log("Playback started:", playback);
      globalPlayback.value = playback;
    })
    .catch((error) => {
      console.error("Error playing MIDI recording:", error);
    });
}

function playTrack(index: number) {
  playMidiTrack(index)
    .then((playback) => {
      console.log("Track playback started:", playback);
      globalPlayback.value = playback;
    })
    .catch((error) => {
      console.error("Error playing MIDI track:", error);
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
    loadTrack,
    playTrack,
    ejectTrack,
  };
}
