export type MidiInputPort = {
  id: string;
  name: string;
};

export type MidiOutputPort = {
  id: string;
  name: string;
};

export type Midi = {
  availableInputPorts: MidiInputPort[];
  availableOutputPorts: MidiOutputPort[];
};
