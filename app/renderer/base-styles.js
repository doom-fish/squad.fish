import { injectGlobal } from 'styled-components';
import 'fonts';
import { fontColor, fontSize } from 'theme';

injectGlobal`
  * {
    font-family: 'Proxima Nova';
    box-sizing: border-box;
  }
	html, body, #root {
    font-size: ${fontSize};
    color: ${fontColor};
		margin: 0;
    height: 100%;
	}
  #root {
    display: flex;
  }
`;
