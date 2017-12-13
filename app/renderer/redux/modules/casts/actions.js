export const LOOK_FOR_CHROMECASTS = 'spectrum/casts/LOOK_FOR_CHROMECASTS';
export const DISCOVERED_CHROMECASTS = 'spectrum/casts/DISCOVERED_CHROMECASTS';
export const TOGGLE_MENU = 'spectrum/casts/TOGGLE_MENU';
export const SWITCH_TO_CAST_DEVICE = 'spectrum/casts/SWITCH_TO_CAST_DEVICE';

export const discoveredChromecasts = chromecasts => ({
  type: DISCOVERED_CHROMECASTS,
  chromecasts,
});

export const toggleMenu = () => ({ type: TOGGLE_MENU });

export const switchToCastDevice = device => ({
  type: SWITCH_TO_CAST_DEVICE,
  device,
});

export const lookForChromecasts = () => ({
  type: LOOK_FOR_CHROMECASTS,
});
