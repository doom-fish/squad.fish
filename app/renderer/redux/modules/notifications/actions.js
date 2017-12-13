export const ERROR_NOTIFICATION = 'spectrum/search/ERROR_NOTIFICATION';

export const errorNotification = error => ({
  type: ERROR_NOTIFICATION,
  payload: error,
});
