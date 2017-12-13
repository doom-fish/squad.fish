// @flow
import React from 'react';
import PropTypes from 'prop-types';
import { connect } from 'react-redux';
import styled from 'styled-components';
import backgroundVideo from './video.mp4';

import Logo from './logo';
const Video = styled.video`
  width: 100%;
  height: 100%;
  position: absolute;
  top: 0;
  left: 0;
  opacity: 0.3;
`;
const Wrapper = styled.div`
  position: absolute;
  top: 0;
  left: 0;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
  display: flex;
`;

const reduxConnection = connect(({ stream, playback }) => ({
  stream,
  playback,
}));
const Background = ({ stream, playback }) => {
  return (
    <Wrapper>
      {!stream.loading && !playback.playing ? (
        <Video autoPlay loop>
          <source src={backgroundVideo} type="video/webm" />
        </Video>
      ) : null}
      <Logo />
    </Wrapper>
  );
};

export default reduxConnection(Background);
