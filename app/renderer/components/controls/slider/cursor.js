//@flow
import React from 'react';
import styled from 'styled-components';
import * as theme from 'theme';
import Draggable from 'utils/draggable';
import CursorIcon from '../../icons/circle-record';

type CursorPropTypes = {
  position: number,
  dragging: boolean,
};

const DraggableCursorIcon = Draggable(
  ({ dragging, ...props }: CursorPropTypes) => <CursorIcon {...props} />
);

const Cursor = styled(DraggableCursorIcon)`
  height: 1rem;
  width: 1rem;
  flex-shrink: 0;
  cursor: ${p => (p.disabled ? 'normal' : 'pointer')};
  opacity: ${p => (p.disabled ? 0.5 : 1)};
  position: absolute;
  circle {
    fill: ${theme.fontColor};
  }
  transition: transform 1s ease-out,
    left ${({ dragging }) => (dragging ? 0 : '100ms')} linear;
  &:active {
    cursor: ${p => (p.disabled ? 'normal' : 'pointer')};
    opacity: ${p => (p.disabled ? 0.5 : 1)};
    fill: ${p => !p.disabled && theme.primaryActiveColor};
    transform: ${p => !p.disabled && 'scale(1.1, 1.1) translate3d(-50%, 0, 0)'};
  }
  left: 0;
  z-index: 1000;
  transform: translate3d(-50%, 0, 0);
`;

export default ({ position, dragging, ...otherProps }: CursorPropTypes) => (
  <Cursor
    dragging={dragging}
    {...otherProps}
    style={{ left: `${position}%` }}
  />
);
