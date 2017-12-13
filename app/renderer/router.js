import React from 'react';
import { MemoryRouter as Router, Route } from 'react-router';

import Layout from 'components/layout';

import VideoControls from 'components/video-controls';
import Video from 'components/video';
import Hud from 'components/hud';
import Background from 'components/background';

import LoadingMessage from 'components/loading-message';

export default () => (
  <Router>
    <Route
      path="/"
      render={() => (
        <Layout>
          <Background />
          <Video />
          <LoadingMessage />
          <Hud>
            <VideoControls />
          </Hud>
        </Layout>
      )}
    />
  </Router>
);
