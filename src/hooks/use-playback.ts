import { playMidiRecording } from "../tauri/playback-commands";

function playRecording(index: number) {
  console.log("Playing MIDI recording...");
  playMidiRecording(index)
    .then((_todo) => {
      console.log("MIDI recording playing started");
      //   globalRecorder.value = recorder;
    })
    .catch((error) => {
      console.error("Error stopping MIDI recording:", error);
      //   globalRecorder.value = defaultRecorder;
    });
}

export function usePlayback() {
  return {
    playRecording,
  };
}
