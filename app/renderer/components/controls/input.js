import styled from 'styled-components';
import * as theme from 'theme';

export default styled.input`
  padding: 0 ${theme.gutterSmall};
  height: 2.5rem;
  color: ${theme.fontColor};
  font-size: ${theme.h4FontSize};

  display: block;
  outline: none;
  &::placeholder {
    color: #9e9e9e;
  }
  -webkit-app-region: no-drag;
  background-color: ${theme.panelBackground};
`;
