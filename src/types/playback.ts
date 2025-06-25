export type Playback = (
  | {
      state: "stopped";
    }
  | {
      state: "playing";
      durationMilliseconds: number;
    }
  | {
      state: "paused";
    }
) & { title?: string; positionMilliseconds: number };
