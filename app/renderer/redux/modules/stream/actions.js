export const LISTEN_FOR_STREAM = 'spectrum/stream/LISTEN_FOR_STREAM';
export const STARTING_STREAM = 'spectrum/stream/STARTING_STREAM';
export const STREAM_STARTED = 'spectrum/stream/STREAM_STARTED';
export const STREAM_START_PROGRESS = 'spectrum/stream/STREAM_START_PROGRESS';

export const streamStartProgress = message => ({
  type: STREAM_START_PROGRESS,
  payload: { message },
});

export const listenForStream = () => ({
  type: LISTEN_FOR_STREAM,
});

export const streamStarted = (streamURL: string) => ({
  type: STREAM_STARTED,
  payload: { streamURL },
});
