import React, { Component } from 'react';
import PropTypes from 'prop-types';
import styled, { css } from 'styled-components';
import * as theme from 'theme';

const styles = css`
  width: 3rem;
  height: 3rem;
  padding: 5px;
  -webkit-app-region: no-drag;
  max-height: 100%;
  cursor: ${p => (p.disabled ? 'normal' : 'pointer')};
  opacity: ${p => (p.disabled ? 0.5 : 1)};
  circle {
    display: none;
  }
  &:hover circle {
    fill: ${theme.primaryHoverColor};
  }
  &:active {
    transform: scale(0.95);
    transition: scale 100ms ease-out;
    fill: ${theme.primaryColor};
  }
`;

type ButtonPropType = {
  onClick: () => void,
};

export default (Icon: Component<*, *, *>) => {
  const StyledButton = styled(Icon)`
    ${styles};
  `;

  const WrappedIcon = (props: ButtonPropType) => <StyledButton {...props} />;
  WrappedIcon.propTypes = {
    onClick: PropTypes.func,
  };

  return WrappedIcon;
};
