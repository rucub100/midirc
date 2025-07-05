import { invoke } from "@tauri-apps/api/core";
import { Recorder } from "../types/recorder";

const COMMAND = {
  GET_MIDI_RECORDER: "get_midi_recorder",
  START_MIDI_RECORDING: "start_midi_recording",
  STOP_MIDI_RECORDING: "stop_midi_recording",
  SAVE_MIDI_RECORDING: "save_midi_recording",
  DELETE_MIDI_RECORDING: "delete_midi_recording",
} as const;

export async function getRecorder() {
  return invoke<Recorder>(COMMAND.GET_MIDI_RECORDER);
}

export async function startMidiRecording() {
  return invoke<Recorder>(COMMAND.START_MIDI_RECORDING);
}

export async function stopMidiRecording() {
  return invoke<Recorder>(COMMAND.STOP_MIDI_RECORDING);
}

export async function deleteMidiRecording(index: number) {
  return invoke<Recorder>(COMMAND.DELETE_MIDI_RECORDING, { index });
}

export async function saveMidiRecording(index: number) {
  return invoke<void>(COMMAND.SAVE_MIDI_RECORDING, { index });
}
