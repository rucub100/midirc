import { invoke } from "@tauri-apps/api/core";
import { Midi } from "../types/midi";

const COMMAND = {
  SCAN_MIDI: "scan_midi",
  CONNECT_MIDI: "connect_midi",
  DISCONNECT_MIDI: "disconnect_midi",
} as const;

export async function scanMidi() {
  return invoke<Midi>(COMMAND.SCAN_MIDI);
}

export async function connectMidi(inputId: string, outputId: string) {
  return invoke<Midi>(COMMAND.CONNECT_MIDI, { inputId, outputId });
}

export async function disconnectMidi() {
  return invoke<Midi>(COMMAND.DISCONNECT_MIDI);
}
