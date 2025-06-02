import { ref } from "vue";
import { Midi } from "../types/midi";
import {
  connectMidiInput,
  connectMidiOutput,
  disconnectMidiInput,
  disconnectMidiOutput,
  getMidi,
  scanMidiInput,
  scanMidiOutput,
} from "../tauri/midi-commands";

const defaultMidi = {
  availableInputPorts: [],
  availableOutputPorts: [],
} satisfies Midi;
const globalMidi = ref<Midi>(defaultMidi);

getMidi()
  .then((midi) => {
    console.log("Initial MIDI state:", midi);
    globalMidi.value = midi;
  })
  .catch((error) => {
    console.error("Error fetching initial MIDI state:", error);
  });

function scanInput() {
  scanMidiInput()
    .then((midi) => {
      console.log("MIDI input scanned:", midi);
      globalMidi.value = midi;
    })
    .catch((error) => {
      console.error("Error scanning MIDI input:", error);
      globalMidi.value = defaultMidi;
    });
}

function scanOutput() {
  scanMidiOutput()
    .then((midi) => {
      console.log("MIDI output scanned:", midi);
      globalMidi.value = midi;
    })
    .catch((error) => {
      console.error("Error scanning MIDI output:", error);
      globalMidi.value = defaultMidi;
    });
}

function connectInput(inputId: string) {
  connectMidiInput(inputId)
    .then((midi) => {
      console.log("MIDI input connected:", midi);
      globalMidi.value = midi;
    })
    .catch((error) => {
      console.error("Error connecting MIDI input:", error);
      globalMidi.value = defaultMidi;
    });
}

function connectOutput(outputId: string) {
  connectMidiOutput(outputId)
    .then((midi) => {
      console.log("MIDI output connected:", midi);
      globalMidi.value = midi;
    })
    .catch((error) => {
      console.error("Error connecting MIDI output:", error);
      globalMidi.value = defaultMidi;
    });
}

function disconnectInput() {
  disconnectMidiInput()
    .then((midi) => {
      console.log("MIDI input disconnected!");
      globalMidi.value = midi;
    })
    .catch((error) => {
      console.error("Error disconnecting MIDI input:", error);
      globalMidi.value = defaultMidi;
    });
}

function disconnectOutput() {
  disconnectMidiOutput()
    .then((midi) => {
      console.log("MIDI output disconnected!");
      globalMidi.value = midi;
    })
    .catch((error) => {
      console.error("Error disconnecting MIDI output:", error);
      globalMidi.value = defaultMidi;
    });
}

export function useMidi() {
  return {
    midi: globalMidi,
    scanInput,
    scanOutput,
    connectInput,
    connectOutput,
    disconnectInput,
    disconnectOutput,
  };
}
