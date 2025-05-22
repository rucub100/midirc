export type MidiInputPort = {
  id: string;
  name: string;
};

export type MidiOutputPort = {
  id: string;
  name: string;
};

export type Midi = {
  input: MidiInputPort[];
  output: MidiOutputPort[];
};
