import { ref } from "vue";
import {
  getRecorder,
  pauseMidiRecording,
  resumeMidiRecording,
  startMidiRecording,
  stopMidiRecording,
} from "../tauri/recorder-commands";
import { Recorder } from "../types/recorder";

const defaultRecorder = {
  state: "stopped",
} satisfies Recorder;

const globalRecorder = ref<Recorder>(defaultRecorder);

getRecorder()
  .then((recorder) => {
    globalRecorder.value = recorder;
  })
  .catch((error) => {
    console.error("Error fetching initial recorder state:", error);
  });

function startRecording() {
  console.log("Starting MIDI recording...");
  startMidiRecording()
    .then((recorder) => {
      console.log("MIDI recording started:", recorder);
      globalRecorder.value = recorder;
    })
    .catch((error) => {
      console.error("Error starting MIDI recording:", error);
      globalRecorder.value = defaultRecorder;
    });
}

function pauseRecording() {
  pauseMidiRecording()
    .then((recorder) => {
      globalRecorder.value = recorder;
    })
    .catch((error) => {
      console.error("Error pausing MIDI recording:", error);
      globalRecorder.value = defaultRecorder;
    });
}

function resumeRecording() {
  resumeMidiRecording()
    .then((recorder) => {
      globalRecorder.value = recorder;
    })
    .catch((error) => {
      console.error("Error resuming MIDI recording:", error);
      globalRecorder.value = defaultRecorder;
    });
}

function stopRecording() {
  console.log("Stopping MIDI recording...");
  stopMidiRecording()
    .then((recorder) => {
      console.log("MIDI recording stopped:", recorder);
      globalRecorder.value = recorder;
    })
    .catch((error) => {
      console.error("Error stopping MIDI recording:", error);
      globalRecorder.value = defaultRecorder;
    });
}

export function useRecorder() {
  return {
    recorder: globalRecorder,
    startRecording,
    pauseRecording,
    resumeRecording,
    stopRecording,
  };
}
