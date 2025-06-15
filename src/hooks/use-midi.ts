import { onUnmounted, ref } from "vue";
import { Midi } from "../types/midi";
import {
  connectMidiInput,
  connectMidiOutput,
  disconnectMidiInput,
  disconnectMidiOutput,
  getMidi,
  registerMidiChannel,
  scanMidiInput,
  scanMidiOutput,
  sendMidiMessage,
} from "../tauri/midi-commands";
import { MidiMessage } from "../types/midi-message";

const defaultMidi = {
  availableInputPorts: [],
  availableOutputPorts: [],
} satisfies Midi;

const globalMidi = ref<Midi>(defaultMidi);

const onMessageCallbacks: ((msg: MidiMessage) => void)[] = [];

registerMidiChannel()
  .then((channel) => {
    channel.onmessage = (msg) => {
      onMessageCallbacks.forEach((callback) => callback(msg));
    };
  })
  .catch((error) => {
    console.error("Error registering MIDI channel:", error);
  });

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

function sendMessage(midiMessage: MidiMessage) {
  sendMidiMessage(midiMessage)
    .then(() => {
      console.log("MIDI message sent:", midiMessage);
    })
    .catch((error) => {
      console.error("Error sending MIDI message:", error);
    });
}

export function useMidi() {
  const onMessageCallback = ref<((msg: MidiMessage) => void) | null>(null);

  function _remoteOnMessageCallback() {
    if (onMessageCallback.value) {
      const index = onMessageCallbacks.indexOf(onMessageCallback.value);
      if (index !== -1) {
        onMessageCallbacks.splice(index, 1);
      }
      onMessageCallback.value = null;
    }
  }

  function onMessage(callback: (msg: MidiMessage) => void) {
    _remoteOnMessageCallback();
    onMessageCallback.value = callback;
    onMessageCallbacks.push(callback);
  }

  onUnmounted(() => {
    _remoteOnMessageCallback();
  });

  return {
    midi: globalMidi,
    scanInput,
    scanOutput,
    connectInput,
    connectOutput,
    disconnectInput,
    disconnectOutput,
    sendMessage,
    onMessage,
  };
}
