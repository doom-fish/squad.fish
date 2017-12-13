import { LOOK_FOR_CHROMECASTS, discoveredChromecasts } from './actions';
import { ipcObservable } from 'utils/ipc';

const discoverCastsEpic = action$ => {
  const lookForCastAction$ = action$.ofType(LOOK_FOR_CHROMECASTS);
  return lookForCastAction$.flatMap(() =>
    ipcObservable('discovered-chromecasts').map(casts =>
      discoveredChromecasts(casts)
    )
  );
};

export default [discoverCastsEpic];
