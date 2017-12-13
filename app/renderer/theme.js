//@flow
import { lighten, darken, rgba } from 'polished';
export const fontColor = '#ffffff';
export const lesserFontColor = '#5b6779';

export const fontSize = '14px';
export const lineHeight = '22px';

export const h1FontSize = '32px';
export const h1LineHeight = '44px';
export const h1FontWeight = 'bold';

export const h2FontSize = '22px';
export const h2LineHeight = '32px';
export const h2FontWeight = 'normal';

export const h3FontSize = '20px';
export const h3LineHeight = '28px';
export const h3FontWeight = 'normal';

export const h4FontSize = '14px';
export const h4LineHeight = '22px';
export const h4FontWeight = 'normal';

export const miniHeaderFontSize = '12px';
export const miniHeaderLineHeight = '14px';
export const miniHeaderFontWeight = 'bold';

export const backgroundColor = '#1b2431';

export const borderColor = '#313d4f';

export const primaryColor = '#2980B9';
export const primaryGreen = '#26C281';
export const primaryRed = '#F64747';

export const primaryHoverColor = lighten(0.05, primaryColor);
export const primaryActiveColor = darken(0.05, primaryColor);

export const panelBackground = rgba('#012132', 0.6);
export const panelDarkBackground = darken(0.05, panelBackground);
export const panelLightBackground = lighten(0.02, panelBackground);

export const progressBackground = rgba('#313d4f', 0.5);
export const progressColor = primaryHoverColor;

export const gutterMini = '0.6rem';
export const gutterSmall = '1rem';
export const gutterMedium = '1.75rem';
export const gutterLarge = '3rem';

export const borderRadius = '4px';
