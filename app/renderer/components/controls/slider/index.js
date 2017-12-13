//@flow
import React, { Component } from 'react';
import type { Element } from 'react';
import PropTypes from 'prop-types';
import styled from 'styled-components';
import * as theme from 'theme';
import Cursor from './cursor';
import Progress from './progress';

const Container = styled.div`
  position: relative;
  display: flex;
  margin: 0.5rem;
  -webkit-app-region: no-drag;

  padding: 10px;
  flex: 1;
  flex-direction: column;
`;

const Layer = styled.div`
  display: flex;
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  align-items: center;
`;

const Track = styled.div`
  height: 0.5rem;
  flex: 1;
  background-color: ${theme.progressBackground};
  box-shadow: inset 1px 1px rgba(0,0,0,.5)
  border-radius: 5px;
`;
const ClickableTrack = styled.div`
  height: 1rem;
  flex: 1;
  cursor: ${p => (p.disabled ? 'normal' : 'pointer')};
  opacity: ${p => (p.disabled ? 0.5 : 1)};
`;

type SliderPropTypes = {
  className: string,
  position: number,
  onChange: () => void,
  value: number,
};
type StateType = { value: number, dragging: boolean };
export default class Slider extends Component<*, SliderPropTypes, StateType> {
  static propTypes = {
    className: PropTypes.string,
    position: PropTypes.number,
    onChange: PropTypes.func,
    value: PropTypes.number,
  };
  static defaultProps = {
    value: 0,
  };
  track: Object;
  state = { value: 0, dragging: false };
  constructor(props) {
    super(props);
  }
  componentWillReceiveProps({ value }: SliderPropTypes) {
    if (!this.state.dragging) {
      this.setState({ value });
    }
  }
  triggerChange(value: number) {
    const { onChange } = this.props;
    if (onChange) {
      onChange(value);
    }
  }
  calculateNewXOffset(clientX: number): number {
    const trackBounding = this.track.getBoundingClientRect();
    return clientX - trackBounding.left;
  }
  isCursorWithinTrack(cursorXOffset: number, tackWidth: number): boolean {
    return cursorXOffset > 0 && cursorXOffset <= tackWidth;
  }
  onTrackClick({ clientX }: SyntheticMouseEvent) {
    if (this.props.disabled) {
      return;
    }
    window.requestAnimationFrame(() => {
      const cursorXOffset = this.calculateNewXOffset(clientX);
      const trackWidth = this.track.clientWidth;
      if (this.isCursorWithinTrack(cursorXOffset, trackWidth)) {
        const value = cursorXOffset / trackWidth * 100;

        this.triggerChange(value);
        this.setState({ value });
      }
    });
  }
  onDrag({ clientX }: SyntheticMouseEvent) {
    if (this.props.disabled) {
      return;
    }
    window.requestAnimationFrame(() => {
      const cursorXOffset = this.calculateNewXOffset(clientX);
      const trackWidth = this.track.clientWidth;
      if (this.isCursorWithinTrack(cursorXOffset, trackWidth)) {
        const value = cursorXOffset / trackWidth * 100;
        this.setState({ value });
      }
    });
  }
  onDragStart() {
    if (this.props.disabled) {
      return;
    }
    this.setState({ dragging: true });
  }
  onDragEnd() {
    const { onChange, disabled } = this.props;
    if (disabled) {
      return;
    }
    if (onChange) {
      onChange(this.state.value);
    }
    this.setState({ dragging: false });
  }
  render(): Element<*> {
    const { className, disabled } = this.props;
    const { value, dragging } = this.state;

    return (
      <Container className={className}>
        <Layer>
          <Track innerRef={track => (this.track = track)} />
        </Layer>
        <Layer>
          <Progress dragging={dragging} position={value} />
        </Layer>
        <Layer>
          <Cursor
            disabled={disabled}
            dragging={dragging}
            onDrag={e => this.onDrag(e)}
            onDragStart={() => this.onDragStart()}
            onDragEnd={() => this.onDragEnd()}
            position={value}
          />
        </Layer>
        <Layer>
          <ClickableTrack
            disabled={disabled}
            onClick={e => this.onTrackClick(e)}
          />
        </Layer>
      </Container>
    );
  }
}
