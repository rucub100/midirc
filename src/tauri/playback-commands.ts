import { invoke } from "@tauri-apps/api/core";
import { Playback } from "../types/playback";

const COMMAND = {
  GET_MIDI_PLAYBACK: "get_midi_playback",
  PLAY_MIDI_RECORDING: "play_midi_recording",
  PAUSE_MIDI_PLAYBACK: "pause_midi_playback",
  RESUME_MIDI_PLAYBACK: "resume_midi_playback",
  STOP_MIDI_PLAYBACK: "stop_midi_playback",
  LOAD_MIDI_TRACK: "load_midi_track",
  PLAY_MIDI_TRACK: "play_midi_track",
  EJECT_MIDI_TRACK: "eject_midi_track",
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

export async function loadMidiTrack() {
  return invoke<Playback>(COMMAND.LOAD_MIDI_TRACK);
}

export async function playMidiTrack(index: number) {
  return invoke<Playback>(COMMAND.PLAY_MIDI_TRACK, { index });
}

export async function ejectMidiTrack(index: number) {
  return invoke<Playback>(COMMAND.EJECT_MIDI_TRACK, { index });
}
