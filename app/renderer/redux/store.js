import { createStore, applyMiddleware, compose } from 'redux';
import { createEpicMiddleware } from 'redux-observable';
import { rootEpic, rootReducer } from './modules/root';
import { composeWithDevTools } from 'redux-devtools-extension';

const epicMiddleware = createEpicMiddleware(rootEpic);
const store = createStore(
  rootReducer,
  composeWithDevTools(applyMiddleware(epicMiddleware))
);
export default store;
