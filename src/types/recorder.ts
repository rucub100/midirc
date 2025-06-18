export type RecordingDuration = { secs: number; nanos: number };

export type Recorder = (
  | {
      state: "stopped";
    }
  | {
      state: "recording";
    }
) & { recordings: any[] };
