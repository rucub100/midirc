import { invoke } from "@tauri-apps/api/core";
import { Recorder } from "../types/recorder";

const COMMAND = {
  PLAY_MIDI_RECORDING: "play_midi_recording",
} as const;

export async function playMidiRecording(index: number) {
  return invoke<Recorder>(COMMAND.PLAY_MIDI_RECORDING, { index });
}
