import React from 'react';
import Transition from 'react-transition-group/Transition';

const defaultStyle = duration => ({
  transition: `opacity ${duration}ms ease-in`,
  opacity: 0,
});

const transitionStyles = {
  entering: { opacity: 1 },
  entered: { opacity: 1 },
  exiting: { opacity: 0 },
};

const Fade = ({ in: inProp, children, duration }) => (
  <Transition in={inProp} timeout={duration}>
    {state => (
      <div
        style={{
          ...defaultStyle(duration),
          ...transitionStyles[state],
        }}
      >
        {state === 'exited' ? null : children}
      </div>
    )}
  </Transition>
);

Fade.defaultProps = {
  duration: 250,
};
export default Fade;
