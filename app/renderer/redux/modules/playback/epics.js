import { Observable } from 'rxjs';
import { PLAY, PAUSE, STOP, LOAD, UNLOAD, SEEK } from './actions';
import MPVDevice from './mpv-device';

const DEVICES = {
  mpv: new MPVDevice(),
};

const resolveDevice = store => DEVICES[store.getState().playback.currentDeivce];

const playbackProgressEpic = action$ => {
  return Observable.merge(DEVICES.mpv.reportProgress(action$));
};

const seekEpic = (action$, store) =>
  action$.ofType(SEEK).flatMap(action => resolveDevice(store).seek(action));

const unloadEpic = (action$, store) =>
  action$.ofType(UNLOAD).flatMap(action => resolveDevice(store).unload(action));

const loadEpic = (action$, store) =>
  action$.ofType(LOAD).flatMap(action => resolveDevice(store).load(action));

const playEpic = (action$, store) =>
  action$.ofType(PLAY).flatMap(action => resolveDevice(store).play(action));

const pauseEpic = (action$, store) =>
  action$.ofType(PAUSE).flatMap(action => resolveDevice(store).pause());

const stopEpic = (action$, store) =>
  action$.ofType(STOP).flatMap(action => resolveDevice(store).stop(action));

export default [
  playEpic,
  seekEpic,
  pauseEpic,
  loadEpic,
  stopEpic,
  unloadEpic,
  playbackProgressEpic,
];
