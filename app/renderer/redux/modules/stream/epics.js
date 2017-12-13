import { Observable } from 'rxjs';
import {
  LISTEN_FOR_STREAM,
  streamStarted,
  streamStartProgress,
} from './actions';
import { play, load, unload } from '../playback/actions';

import { ipcObservable } from 'utils/ipc';
import { errorNotification } from '../notifications/actions';

const listenForStreamEpic = action$ =>
  action$
    .ofType(LISTEN_FOR_STREAM)
    .switchMap(() => ipcObservable('listen-for-stream'))
    .flatMap(
      response =>
        response.type === 'progress'
          ? Observable.of(streamStartProgress(response.message))
          : Observable.of(
              streamStarted(response.data),
              load(response.data),
              play(response.data)
            )
    );

export default [listenForStreamEpic];
