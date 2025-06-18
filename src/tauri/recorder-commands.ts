import { invoke } from "@tauri-apps/api/core";
import { Recorder } from "../types/recorder";
// import { Recorder } from "../types/recorder";

const COMMAND = {
  GET_MIDI_RECORDER: "get_midi_recorder",
  START_MIDI_RECORDING: "start_midi_recording",
  PAUSE_MIDI_RECORDING: "pause_midi_recording",
  RESUME_MIDI_RECORDING: "resume_midi_recording",
  STOP_MIDI_RECORDING: "stop_midi_recording",
} as const;

export async function getRecorder() {
  return invoke<Recorder>(COMMAND.GET_MIDI_RECORDER);
}

export async function startMidiRecording() {
  return invoke<Recorder>(COMMAND.START_MIDI_RECORDING);
}

export async function pauseMidiRecording() {
  return invoke<Recorder>(COMMAND.PAUSE_MIDI_RECORDING);
}

export async function resumeMidiRecording() {
  return invoke<Recorder>(COMMAND.RESUME_MIDI_RECORDING);
}

export async function stopMidiRecording() {
  return invoke<Recorder>(COMMAND.STOP_MIDI_RECORDING);
}
