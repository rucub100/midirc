import { invoke } from "@tauri-apps/api/core";
import { Playback } from "../types/playback";

const COMMAND = {
  GET_MIDI_PLAYBACK: "get_midi_playback",
  PLAY_MIDI_RECORDING: "play_midi_recording",
  PAUSE_MIDI_PLAYBACK: "pause_midi_playback",
  RESUME_MIDI_PLAYBACK: "resume_midi_playback",
  STOP_MIDI_PLAYBACK: "stop_midi_playback",
} as const;

export async function getMidiPlayback() {
  return invoke<Playback>(COMMAND.GET_MIDI_PLAYBACK);
}

export async function playMidiRecording(index: number) {
  return invoke<Playback>(COMMAND.PLAY_MIDI_RECORDING, { index });
}

export async function pauseMidiPlayback() {
  return invoke<Playback>(COMMAND.PAUSE_MIDI_PLAYBACK);
}

export async function resumeMidiPlayback() {
  return invoke<Playback>(COMMAND.RESUME_MIDI_PLAYBACK);
}

export async function stopMidiPlayback() {
  return invoke<Playback>(COMMAND.STOP_MIDI_PLAYBACK);
}
