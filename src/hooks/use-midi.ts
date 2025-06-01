import { ref } from "vue";
import { Midi } from "../types/midi";
import { connectMidi, disconnectMidi, scanMidi } from "../tauri/midi-commands";

const defaultMidi = {
  availableInputPorts: [],
  availableOutputPorts: [],
} satisfies Midi;
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

function connect(inputId: string, outputId: string) {
  connectMidi(inputId, outputId)
    .then((midi) => {
      console.log("MIDI devices connected:", midi);
      globalMidi.value = midi;
    })
    .catch((error) => {
      console.error("Error connecting MIDI devices:", error);
      globalMidi.value = defaultMidi;
    });
}

function disconnect() {
  disconnectMidi()
    .then((midi) => {
      console.log("MIDI devices disconnected!");
      globalMidi.value = midi;
    })
    .catch((error) => {
      console.error("Error disconnecting MIDI devices:", error);
      globalMidi.value = defaultMidi;
    });
}

export function useMidi() {
  return { midi: globalMidi, scan, connect, disconnect };
}
