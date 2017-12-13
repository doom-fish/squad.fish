import {
  UPDATE_PROGRESS,
  UNLOADED,
  STOPPED,
  LOADED,
  PLAYING,
  PAUSED,
} from './actions';
import { LISTEN_FOR_STREAM } from '../stream/actions';
const initialState = {
  currentDeivce: 'mpv',
  playing: false,
  stopped: true,
  disabled: true,
  progress: 0,
};
export default function playbackReducer(state = initialState, action) {
  switch (action.type) {
    case LISTEN_FOR_STREAM:
      return { ...state, disabled: true };
    case LOADED:
      return { ...state, disabled: false };
    case PLAYING:
      return { ...state, playing: true, stopped: false };
    case STOPPED:
      return { ...state, playing: false, progress: 0, stopped: true };
    case UNLOADED:
      return { ...state, playing: false };
    case PAUSED:
      return { ...state, playing: false };
    case UPDATE_PROGRESS:
      return { ...state, progress: action.payload.progress };
  }
  return state;
}
