import {
  STREAM_STARTED,
  STREAM_START_PROGRESS,
  LISTEN_FOR_STREAM,
} from './actions';
const defaultState = {
  loading: false,
  progressMessage: 'Loading',
};

export default function streamReducer(state = defaultState, action) {
  switch (action.type) {
    case LISTEN_FOR_STREAM:
      return {
        ...state,
        loading: true,
      };
    case STREAM_START_PROGRESS:
      return {
        ...state,
        progressMessage: action.payload.message.replace(/-/g, ' '),
      };
    case STREAM_STARTED:
      return { ...state, loading: false, progressMessage: 'Loading' };
    default:
      return state;
  }
}
