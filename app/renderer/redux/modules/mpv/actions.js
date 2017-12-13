export const SEND_COMMAND = 'spectrum/mpv/SEND_COMMAND';
export const COMMAND_SENT = 'spectrum/mpv/COMMAND_SENT';

export const CHANGE_PROPERTY = 'spectrum/mpv/CHANGE_PROPERTY';
export const PROPERTY_CHANGED = 'spectrum/mpv/PROPERTY_CHANGED';

export const OBSERVE_PROPERTY = 'spectrum/mpv/OBSERVE_PROPERTY';
export const UNOBSERVE_PROPERTY = 'spectrum/mpv/UNOBSERVE_PROPERTY';

export const OBSERVED_PROPERTY_UPDATED =
  'spectrum/mpv/OBSERVED_PROPERTY_UPDATED';

export const observeMPVProperty = (name, bufferTime) => ({
  type: OBSERVE_PROPERTY,
  payload: { name, bufferTime },
});

export const unobserveMPVProperty = (name, bufferTime) => ({
  type: UNOBSERVE_PROPERTY,
  payload: { name },
});

export const observedMPVPropertyUpdated = ({ name, value }) => ({
  type: OBSERVED_PROPERTY_UPDATED,
  payload: { name, value },
});

export const changeMPVProperty = (name, value) => ({
  type: CHANGE_PROPERTY,
  payload: {
    name,
    value,
  },
});

export const MPVPropertyChanged = payload => ({
  type: PROPERTY_CHANGED,
  payload: payload,
});

export const MPVCommandSent = command => ({
  type: COMMAND_SENT,
  payload: command,
});
export const sendMPVCommand = (name, ...args) => ({
  type: SEND_COMMAND,
  payload: {
    name,
    args,
  },
});
