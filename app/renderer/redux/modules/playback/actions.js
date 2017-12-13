export const LOAD = 'spectrum/playback/LOAD';
export const UNLOAD = 'spectrum/playback/UNLOAD';
export const SEEK = 'spectrum/playback/SEEK';
export const LOADED = 'spectrum/playback/LOADED';
export const UNLOADED = 'spectrum/playback/UNLOADED';
export const PAUSE = 'spectrum/playback/PAUSE';
export const PAUSED = 'spectrum/playback/PAUSED';
export const PLAY = 'spectrum/playback/PLAY';
export const STOP = 'spectrum/playback/STOP';
export const STOPPED = 'spectrum/playback/STOPPED';
export const PLAYING = 'spectrum/playback/PLAYING';
export const UPDATE_PROGRESS = 'spectrum/playback/UPDATE_PROGRESS';
export const load = fileURI => ({
  type: LOAD,
  payload: {
    fileURI,
  },
});

export const loaded = fileURI => ({
  type: LOADED,
  payload: {
    fileURI,
  },
});
export const seek = percentPos => ({
  type: SEEK,
  payload: {
    percentPos,
  },
});
export const stop = () => ({ type: STOP });
export const pause = () => ({ type: PAUSE });
export const play = () => ({ type: PLAY });
export const stopped = () => ({ type: STOPPED });
export const paused = () => ({ type: PAUSED });
export const playing = () => ({ type: PLAYING });
export const unload = () => ({ type: UNLOAD });
export const unloaded = () => ({ type: UNLOADED });

export const updateProgress = progress => ({
  type: UPDATE_PROGRESS,
  payload: { progress },
});
