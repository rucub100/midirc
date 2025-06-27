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
      durationMilliseconds: number;
    }
) & { title?: string; positionMilliseconds: number };
