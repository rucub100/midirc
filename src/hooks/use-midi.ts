import { ref } from "vue";
import { Midi } from "../types/midi";
import { scanMidi } from "../tauri/midi-commands";

const defaultMidi = { input: [], output: [] } satisfies Midi;
const globalMidi = ref<Midi>(defaultMidi);

function scan() {
  scanMidi()
    .then((midi) => {
      console.log("MIDI devices scanned:", midi);
      globalMidi.value = midi;
    })
    .catch((error) => {
      // TODO: proper error handling (e.g. show a notification)
      console.error("Error scanning MIDI devices:", error);
      globalMidi.value = defaultMidi;
    });
}

export function useMidi() {
  return { midi: globalMidi, scan };
}
