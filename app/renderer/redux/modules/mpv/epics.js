import {
  SEND_COMMAND,
  CHANGE_PROPERTY,
  UNOBSERVE_PROPERTY,
  OBSERVE_PROPERTY,
  MPVCommandSent,
  MPVPropertyChanged,
  observedMPVPropertyUpdated,
} from './actions';
import mpvMediator from 'utils/mpv-mediator';

const sendCommandEpic = action$ =>
  action$
    .ofType(SEND_COMMAND)
    .map(action => action.payload)
    .switchMap(command => mpvMediator.sendCommand(command))
    .map(MPVCommandSent);

const changePropertyEpic = action$ =>
  action$
    .ofType(CHANGE_PROPERTY)
    .map(action => action.payload)
    .switchMap(payload => mpvMediator.changeProperty(payload))
    .map(MPVPropertyChanged);

const delayedBufferObserver = ({ name, bufferTime }) => {
  const observedProperty$ = mpvMediator.observeProperty(name);
  const observedValuePayload = (name, value) => ({ name, value });
  const lastValue = values => values[values.length - 1];

  if (bufferTime) {
    const delayed$ = observedProperty$.delay(bufferTime);
    return observedProperty$
      .bufferWhen(() => delayed$)
      .map(values => observedValuePayload(name, lastValue(values)));
  } else {
    return observedProperty$.map(value => observedValuePayload(name, value));
  }
};

const observePropertyEpic = action$ => {
  const observePropertyAction$ = action$
    .ofType(OBSERVE_PROPERTY)
    .map(action => action.payload);
  const stopObservePropertyAction$ = action$
    .ofType(UNOBSERVE_PROPERTY)
    .map(action => action.payload);
  return observePropertyAction$
    .flatMap(payload =>
      delayedBufferObserver(payload).takeUntil(
        stopObservePropertyAction$.filter(({ name }) => payload.name === name)
      )
    )
    .map(observedMPVPropertyUpdated);
};

export default [observePropertyEpic, changePropertyEpic, sendCommandEpic];
