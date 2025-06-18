import { ref } from "vue";
import {
  getRecorder,
  startMidiRecording,
  stopMidiRecording,
} from "../tauri/recorder-commands";
import { Recorder } from "../types/recorder";

const defaultRecorder = {
  state: "stopped",
  recordings: [],
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
    stopRecording,
  };
}
