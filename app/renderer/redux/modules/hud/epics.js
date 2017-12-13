import { Observable } from 'rxjs';
import { HUD_HOVER, hudVisible, hudHidden } from './actions';

import { STREAM_START_PROGRESS } from '../stream/actions';
const hudHoverEpic = action$ => {
  const hoverAction$ = Observable.merge(
    action$.ofType(HUD_HOVER),
    action$.ofType(STREAM_START_PROGRESS)
  );
  const hudVisible$ = hoverAction$.mapTo(hudVisible());
  const hidHidden$ = hoverAction$.debounceTime(5000).mapTo(hudHidden());

  return Observable.merge(hudVisible$, hidHidden$);
};
export default [hudHoverEpic];
