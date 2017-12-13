//@flow
import React from 'react';
import PropTypes from 'prop-types';
import PlayIcon from '../icons/circle-play';
import PauseIcon from '../icons/circle-pause';
import IconButton from '../buttons/icon-button';

const PlayButton = IconButton(PlayIcon);
const PauseButton = IconButton(PauseIcon);

type PropsType = {
  onClick: () => void,
  playing: boolean,
};

const PlayToggleButton = ({
  playing,
  onPauseClick,
  onPlayClick,
  ...props
}: PropsType) =>
  playing ? (
    <PauseButton onClick={onPauseClick} {...props} />
  ) : (
    <PlayButton onClick={onPlayClick} {...props} />
  );

PlayToggleButton.propTypes = {
  onClick: PropTypes.func,
  playing: PropTypes.bool,
};

PlayToggleButton.defaultProps = {
  playing: false,
};

export default PlayToggleButton;
