/// <reference path="./types.d.ts" />

import * as React from 'react';
import * as ReactDOM from 'react-dom';
import App from './components/app.tsx';
import { Provider } from 'react-redux'
import { applyMiddleware, createStore } from 'redux'
import thunk from 'redux-thunk';
import * as createLogger from 'redux-logger';
import reducers from './reducers'
import * as nw from 'nw';

const logger = createLogger();
const storeMiddleware = applyMiddleware(thunk, logger);
const store = createStore(reducers, storeMiddleware);

ReactDOM.render(
  <Provider store={store}>
    <App />
  </Provider>,
  document.getElementById('root')
);
