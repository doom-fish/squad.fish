import { DISCOVERED_CHROMECASTS, TOGGLE_MENU } from './actions';
const initialState = {
  menuOpen: false,
  chromecasts: [],
  canCast: false,
};

export default function castsReducer(state = initialState, action) {
  switch (action.type) {
    case TOGGLE_MENU: {
      return { ...state, menuOpen: !state.menuOpen };
    }
    case DISCOVERED_CHROMECASTS:
      const { chromecasts } = action;
      return { ...state, chromecasts, canCast: chromecasts.length > 0 };
  }
  return state;
}
