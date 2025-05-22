import { invoke } from "@tauri-apps/api/core";
import { Midi } from "../types/midi";

const COMMAND = {
  SCAN_MIDI: "scan_midi",
} as const;

export async function scanMidi() {
  return invoke<Midi>(COMMAND.SCAN_MIDI);
}
