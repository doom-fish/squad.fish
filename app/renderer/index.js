import { AppContainer } from 'react-hot-loader';
import React from 'react';
import { Provider } from 'react-redux';
import ReactDOM from 'react-dom';
import Router from './router';
import store from 'redux/store';
import 'base-styles';
import initialAcitons from './initial-actions';
initialAcitons(store);

const rootEl = document.getElementById('root');
const render = Component =>
  ReactDOM.render(
    <AppContainer>
      <Provider store={store}>
        <Component />
      </Provider>
    </AppContainer>,
    rootEl
  );

render(Router);
if (module.hot) module.hot.accept('./router', () => render(Router));
