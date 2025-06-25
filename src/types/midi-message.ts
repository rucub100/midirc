export type MidiChannel =
  | "channel1"
  | "channel2"
  | "channel3"
  | "channel4"
  | "channel5"
  | "channel6"
  | "channel7"
  | "channel8"
  | "channel9"
  | "channel10"
  | "channel11"
  | "channel12"
  | "channel13"
  | "channel14"
  | "channel15"
  | "channel16";

export type ChannelVoiceMessage =
  | { noteOff: { note: number; velocity: number } }
  | { noteOn: { note: number; velocity: number } }
  | { polyphonicKeyPressure: { note: number; pressure: number } }
  | { controlChange: { controller: number; value: number } }
  | { programChange: number }
  | { channelPressure: number }
  | { pitchBendChange: number };

export type ChannelModeMessage =
  | "allSoundOff"
  | "resetAllControllers"
  | "localControlOff"
  | "localControlOn"
  | "allNotesOff"
  | "omniModeOff"
  | "omniModeOn"
  | { monoMode: { numberOfVoices: number } }
  | "polyMode";

export type ChannelMessage =
  | { voice: ChannelVoiceMessage }
  | { mode: ChannelModeMessage };

export type SystemCommonMessage =
  | "midiTimeCodeQuarterFrame"
  | { songPositionPointer: number }
  | { songSelect: number }
  | "tuneRequest"
  | "endOfSystemExclusive";

export type SystemRealTimeMessage =
  | "timingClock"
  | "start"
  | "continue"
  | "stop"
  | "activeSensing"
  | "systemReset";

export type SystemExclusiveSubId =
  | { manufacturerIdentification: number[] }
  | "nonCommercial"
  | "nonRealTime"
  | "realTime";

export interface SystemExclusiveMessage {
  subId: SystemExclusiveSubId;
  data: number[];
}

export type SystemMessage =
  | { common: SystemCommonMessage }
  | { realTime: SystemRealTimeMessage }
  | { exclusive: SystemExclusiveMessage };

export type MidiChannelMessage = {
  channel: { channel: MidiChannel; message: ChannelMessage };
};

export type MidiSystemMessage = { system: SystemMessage };

export type MidiMessage = MidiChannelMessage | MidiSystemMessage;

export function isMidiChannelMessage(
  message: MidiMessage
): message is MidiChannelMessage {
  return "channel" in message;
}
