import React from 'react';
import PropTypes from 'prop-types';
import Fade from 'utils/fade';
import styled, { keyframes, css } from 'styled-components';
import * as theme from 'theme';
import { connect } from 'react-redux';

const LoadingMessage = styled.div`
  color: ${theme.fontColor};
  font-size: ${theme.h4FontSize};
  display: block;
  background-color: rgba(0, 0, 0, 0.8);
  outline: none;
  opacity: ${p => (p.loading ? '1' : '0')};
  transition: opacity 250ms ease-out;
  display: flex;
  align-items: center;
  justify-content: center;
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  text-transform: uppercase;
  overflow: visible;
  user-select: none;
  cursor: default;
  margin: auto;
`;

const square = 16;
const duration = 10;

const squareAnimation = keyframes`
  0% {left: 0; top: 0}

  10.5% {left: 0; top: 0;}
  12.5% {left: ${square}px; top: 0;}

  23% {left: ${square}px; top: 0;}
  25% {left: ${square * 2}px; top: 0;}

  35.5% {left: ${square * 2}px; top: 0;}
  37.5% {left: ${square * 2}px; top: ${square}px;}

  48% {left: ${square * 2}px; top: ${square}px;}
  50% {left: ${square}px; top: ${square}px;}

  60.5% {left: ${square}px; top: ${square}px;}
  62.5% {left: ${square}px; top: ${square * 2}px;}

  73% {left: ${square}px; top: ${square * 2}px;}
  75% {left: 0; top: ${square * 2}px;}

  85.5% {left: 0; top: ${square * 2}px;}
  87.5% {left: 0; top: ${square}px;}

  98% {left: 0; top: ${square}px;}
  100% {left: 0; top: 0;}
`;
const hueRotateAnimation = keyframes`
  0% {filter: hue-rotate(0deg)}
  100% {filter: hue-rotate(360deg)}
`;

const SquaresWrapper = styled.div`
  position: relative;
  width: ${square * 3}px;
  height: ${square * 3}px;
  transform: rotate(45deg);
  animation: ${hueRotateAnimation} ${duration}s linear infinite both;
`;

const Square = styled.div`
  position: absolute;
  top: 0;
  left: 0;
  width: ${square - 3}px;
  height: ${square - 3}px;
  margin: 2px;
  border-radius: 2px;
  background: #07a;
  background-image: linear-gradient(45deg, #fa0 40%, #0c9 60%);
  background-size: cover;
  background-position: center;
  background-attachment: fixed;
  animation: ${squareAnimation} ${duration}s ease-in-out infinite both;
  &:nth-of-type(1) {
    animation-delay: -${(duration / 7).toString().substring(0, 5)}s;
  }
  &:nth-of-type(2) {
    animation-delay: -${(duration / 7 * 2).toString().substring(0, 5)}s;
  }
  &:nth-of-type(3) {
    animation-delay: -${(duration / 7 * 3).toString().substring(0, 5)}s;
  }
  &:nth-of-type(4) {
    animation-delay: -${(duration / 7 * 4).toString().substring(0, 5)}s;
  }
  &:nth-of-type(5) {
    animation-delay: -${(duration / 7 * 5).toString().substring(0, 5)}s;
  }
  &:nth-of-type(6) {
    animation-delay: -${(duration / 7 * 6).toString().substring(0, 5)}s;
  }
  &:nth-of-type(7) {
    animation-delay: -${(duration / 7 * 7).toString().substring(0, 5)}s;
  }
`;

const Message = styled.div`
  position: absolute;
  padding-top: 70px;
`;

const reduxConnection = connect(({ stream }) => stream);
const AnimatedLoadingMessage = ({ loading, progressMessage }) => (
  <Fade in={loading}>
    <LoadingMessage loading={loading}>
      <SquaresWrapper>
        <Square />
        <Square />
        <Square />
        <Square />
        <Square />
        <Square />
        <Square />
        <Square />
      </SquaresWrapper>
      <Message>{progressMessage}</Message>
    </LoadingMessage>
  </Fade>
);

AnimatedLoadingMessage.defaultProps = {
  progressMessage: 'Loading',
};

export default reduxConnection(AnimatedLoadingMessage);
