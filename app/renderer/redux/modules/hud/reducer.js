import { HUD_VISIBLE, HUD_HIDDEN } from './actions';

const initialState = {
  visible: true,
};

export default function hudReducer(state = initialState, action) {
  switch (action.type) {
    case HUD_VISIBLE:
      return { ...state, visible: true };
    case HUD_HIDDEN:
      return { ...state, visible: false };
    default:
      return state;
  }
}
