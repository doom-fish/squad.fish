import React from 'react';
import PropTypes from 'prop-types';
import { connect } from 'react-redux';

import styled from 'styled-components';

import Slider from '../controls/slider';
import Chromecasts from '../casts/chromecasts';
import PlayToggleButton from './play-toggle-button';
import IconButton from '../buttons/icon-button';
import StopIcon from '../icons/circle-stop';

import { panelBackground, gutterMini } from 'theme';
import filterRender from 'utils/filtered-render';

import { play, stop, pause, seek } from 'redux/modules/playback/actions';

const Container = styled.div`
  position: absolute;
  bottom: 0;
  left: ${gutterMini};
  right: ${gutterMini};
  margin: auto;
  display: flex;
  align-items: center;
`;
const Buttons = styled.div`
  margin-left: 5px;
  margin-right: 10px;
  display: flex;
  > * {
    flex: 1;
  }
`;

const StopButton = IconButton(StopIcon);

const FullWidthSlider = styled(Slider)`
  flex: 1
  margin-right: 15px;
`;

const reduxConnection = connect(({ playback }) => ({ playback }), {
  play,
  stop,
  pause,
  seek,
});

const Controls = ({ play, pause, stop, seek, playback }) => (
  <Container>
    <Buttons>
      <PlayToggleButton
        disabled={playback.disabled}
        onPauseClick={pause}
        onPlayClick={play}
        playing={playback.playing}
      />
      <StopButton disabled={playback.disabled} onClick={stop} />
    </Buttons>
    <FullWidthSlider
      value={playback.progress}
      disabled={playback.disabled}
      onChange={seek}
    />
    <Buttons>
      <Chromecasts />
    </Buttons>
  </Container>
);

export default reduxConnection(Controls);
