import React, { Element, PureComponent } from 'react';
import { connect } from 'react-redux';
import styled from 'styled-components';
import PropTypes from 'prop-types';
import mpvMediator from 'utils/mpv-mediator';

const VideoContainer = styled.div`
  flex-direction: column;
  display: flex;
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  transition: opacity 0ms ease-out;
  opacity: ${p => (p.stopped ? 0 : 1)};
`; 

  const videoReduxConnection = connect(({ playback }) => playback);
const Player = styled(mpvMediator.getComponent())`flex: 1;`;

class Video extends PureComponent {
  render(): Element<*> {
    const controlProps = this.props;

    return (
      <VideoContainer stopped={this.props.stopped}>
        <Player />
      </VideoContainer>
    );
  }
}

export default videoReduxConnection(Video);
