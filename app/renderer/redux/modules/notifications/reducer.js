import { ERROR_NOTIFICATION } from './actions';

const initialState = {
  errors: [],
};

export default function notifcationsReducer(state = initialState, action) {
  switch (action.type) {
    case ERROR_NOTIFICATION:
      return { errors: [...state.errors, action.payload] };
    default:
  }
  return state;
}
