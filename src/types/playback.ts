export type PlaybackIdentifier =
  | {
      type: "recording";
      index: number;
    }
  | {
      type: "midiFile";
      path: string;
    };

export type Track = {
  index: number;
  durationMilliseconds: number;
};

export type Playback = (
  | {
      state: "stopped";
    }
  | {
      state: "playing";
      durationMilliseconds: number;
      identifier: PlaybackIdentifier;
    }
  | {
      state: "paused";
      durationMilliseconds: number;
      identifier: PlaybackIdentifier;
    }
) & {
  positionMilliseconds: number;
  tracks: Track[];
};
