// @flow
import React from 'react';
import { connect } from 'react-redux';
import { hudHover } from 'redux/modules/hud/actions';
import debounce from 'utils/debounce';
import styled from 'styled-components';

const HudWrapper = styled.div`
  flex: 1;
  width: 100%;
  height: 100%;

  transition: opacity 250ms ease-out;
  cursor: ${({ visible }) => (visible ? 'default' : 'none')};
  opacity: ${({ visible }) => (visible ? '1' : '0')};
`;

const hudConnection = connect(({ hud }) => ({ visible: hud.visible }), {
  hudHover,
});

const Hud = ({ visible, hudHover, children }) => {
  return (
    <HudWrapper
      onKeyDown={hudHover}
      onMouseDown={hudHover}
      onMouseMove={hudHover}
      visible={visible}
    >
      {children}
    </HudWrapper>
  );
};

export default hudConnection(Hud);
