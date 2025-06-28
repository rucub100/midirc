export type RecordingDuration = { secs: number; nanos: number };

export type Recording = {
  index: number;
  durationMilliseconds: number;
};

export type Recorder = (
  | {
      state: "stopped";
    }
  | {
      state: "recording";
    }
) & { recordings: Recording[] };
