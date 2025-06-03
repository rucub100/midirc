import { invoke } from "@tauri-apps/api/core";
import { Midi } from "../types/midi";

const COMMAND = {
  GET_MIDI: "get_midi",
  SCAN_MIDI_INPUT: "scan_midi_input",
  SCAN_MIDI_OUTPUT: "scan_midi_output",
  CONNECT_MIDI_INPUT: "connect_midi_input",
  CONNECT_MIDI_OUTPUT: "connect_midi_output",
  DISCONNECT_MIDI_INPUT: "disconnect_midi_input",
  DISCONNECT_MIDI_OUTPUT: "disconnect_midi_output",
  PLAY_MIDI_DEMO: "play_midi_demo",
} as const;

export async function getMidi() {
  return invoke<Midi>(COMMAND.GET_MIDI);
}

export async function scanMidiInput() {
  return invoke<Midi>(COMMAND.SCAN_MIDI_INPUT);
}

export async function scanMidiOutput() {
  return invoke<Midi>(COMMAND.SCAN_MIDI_OUTPUT);
}

export async function connectMidiInput(inputId: string) {
  return invoke<Midi>(COMMAND.CONNECT_MIDI_INPUT, { inputId });
}

export async function connectMidiOutput(outputId: string) {
  return invoke<Midi>(COMMAND.CONNECT_MIDI_OUTPUT, { outputId });
}

export async function disconnectMidiInput() {
  return invoke<Midi>(COMMAND.DISCONNECT_MIDI_INPUT);
}

export async function disconnectMidiOutput() {
  return invoke<Midi>(COMMAND.DISCONNECT_MIDI_OUTPUT);
}

export async function playMidiDemo() {
  return invoke<void>(COMMAND.PLAY_MIDI_DEMO);
}
