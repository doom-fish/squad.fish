import { combineReducers } from 'redux';
import { combineEpics } from 'redux-observable';

import streamEpics from './stream/epics';
import mpvEpics from './mpv/epics';
import playbackEpics from './playback/epics';
import hudEpics from './hud/epics';
import castsEpics from './casts/epics';

import notifcationsReducer from './notifications/reducer';

import streamReducer from './stream/reducer';
import playbackReducer from './playback/reducer';
import hudReducer from './hud/reducer';
import castsReducer from './casts/reducer';
export const rootEpic = combineEpics(
  ...[...castsEpics, ...streamEpics, ...mpvEpics, ...playbackEpics, ...hudEpics]
);

export const rootReducer = combineReducers({
  notifcations: notifcationsReducer,

  stream: streamReducer,
  playback: playbackReducer,
  hud: hudReducer,
  casts: castsReducer,
});
