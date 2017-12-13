//@flow
import { injectGlobal } from 'styled-components';

injectGlobal`
  @font-face {
    font-family: 'Proxima Nova';
    src: url(${require('./proximanova-bold-webfont.woff')}) format('woff');
    font-weight: 800;
    font-style: normal;
  }
  @font-face {
    font-family: 'Proxima Nova';
    src: url(${require('./proximanova-light-webfont.woff')}) format('woff');
    font-weight: 300;
    font-style: normal;
  }
  @font-face {
    font-family: 'Proxima Nova';
    src: url(${require('./proximanova-thin-webfont.woff')}) format('woff');
    font-weight: 200;
    font-style: normal;
  }
  @font-face {
    font-family: 'Proxima Nova';
    src: url(${require('./proximanova-regular-webfont.woff')}) format('woff');
    font-weight: 400;
    font-style: normal;
  }
  @font-face {
    font-family: 'Proxima Nova';
    src: url(${require('./proximanova-regularitalic-webfont.woff')}) format('woff');
    font-weight: 400;
    font-style: italic;
  }
  @font-face {
    font-family: 'Proxima Nova';
    src: url(${require('./proximanova-semibold-webfont.woff')}) format('woff');
    font-weight: 600;
    font-style: normal;
  }
`;
