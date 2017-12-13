export const passValueTo = eventHandler => ({ target: { value } }) =>
  eventHandler(value);
