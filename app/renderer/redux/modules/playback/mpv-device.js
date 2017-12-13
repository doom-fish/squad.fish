import { Observable } from 'rxjs';
import {
  load,
  updateProgress,
  loaded,
  unloaded,
  play,
  playing,
  stopped,
  paused,
} from './actions';

import {
  OBSERVED_PROPERTY_UPDATED,
  sendMPVCommand,
  observeMPVProperty,
  unobserveMPVProperty,
  changeMPVProperty,
} from '../mpv/actions';

export default class MPVDevice {
  reportProgress(action$) {
    return action$
      .ofType(OBSERVED_PROPERTY_UPDATED)
      .filter(action => action.payload.name === 'percent-pos')
      .map(action => updateProgress(action.payload.value));
  }
  seek(action) {
    return Observable.of(
      unobserveMPVProperty('percent-pos'),
      changeMPVProperty('percent-pos', action.payload.percentPos),
      observeMPVProperty('percent-pos', 100)
    );
  }
  unload() {
    return Observable.of(
      unobserveMPVProperty('percent-pos'),
      changeMPVProperty('pause', true),
      unloaded()
    );
  }

  load(action) {
    return Observable.of(
      sendMPVCommand('loadfile', action.payload.fileURI),
      loaded(action.payload.fileURI)
    );
  }

  play() {
    return Observable.of(
      unobserveMPVProperty('percent-pos'),
      observeMPVProperty('percent-pos', 100),
      changeMPVProperty('pause', false),
      playing()
    );
  }

  pause() {
    return Observable.of(changeMPVProperty('pause', true), paused());
  }
  stop() {
    return Observable.of(
      changeMPVProperty('percent-pos', 0),
      changeMPVProperty('pause', true),
      unobserveMPVProperty('percent-pos'),
      stopped()
    );
  }
}
