// @flow
import React from 'react';
import { connect } from 'react-redux';
import styled from 'styled-components';
import { switchToCastDevice } from 'redux/modules/casts/actions';
import {
  panelBackground,
  primaryColor,
  primaryActiveColor,
  primaryHoverColor,
  gutterSmall,
  gutterMini,
} from 'theme';

const MenuContainer = styled.div`
  position: absolute;
  bottom: 100%;
  -webkit-app-region: no-drag;
  right: -0.5rem;
  background-color: ${panelBackground};
`;

const MenuItem = styled.div`
  z-index: 30;
  cursor: pointer;
  user-select: none;
  &:active {
    background-color: ${primaryActiveColor} !important;
  }
  &:hover {
    background-color: ${primaryHoverColor};
  }
  padding: ${gutterMini} ${gutterSmall};
`;

const ChromecastMenu = ({ chromecasts, ...otherProps }) => {
  return (
    <MenuContainer {...otherProps}>
      {chromecasts.map(chromecast => (
        <MenuItem
          onClick={() => switchToCastDevice(chromecast)}
          key={chromecast.host}
        >
          {chromecast.name}
        </MenuItem>
      ))}
    </MenuContainer>
  );
};

export default ChromecastMenu;
