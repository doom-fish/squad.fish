// @flow
import React from 'react';
import { connect } from 'react-redux';
import { toggleMenu } from 'redux/modules/casts/actions';
import styled from 'styled-components';
import Button from './button';
import Menu from './menu';
import Fade from 'utils/fade';

const Container = styled.div`
  position: relative;
`;

const reduxConnection = connect(({ casts }) => casts, { toggleMenu });
const Chromecasts = ({ canCast, chromecasts, toggleMenu, menuOpen }) => {
  return (
    <Container>
      <Fade duration={100} in={canCast && menuOpen}>
        <Menu onMouseUp={toggleMenu} chromecasts={chromecasts} />
      </Fade>
      <Button onClick={toggleMenu} disabled={!canCast} />
    </Container>
  );
};

export default reduxConnection(Chromecasts);
