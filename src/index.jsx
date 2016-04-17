import React from 'react';
import ReactDOM from 'react-dom';
import App from './components/app.jsx';
import { Provider } from 'react-redux'
import { applyMiddleware, createStore } from 'redux'
import thunk from 'redux-thunk';
import promise from 'redux-promise';
import createLogger from 'redux-logger';
import reducers from './reducers'

if (typeof nw !== 'undefined') {
  var win = nw.Window.get();
  win.showDevTools();
}

const logger = createLogger();
const store = createStore(
  reducers,
  applyMiddleware(thunk, promise, logger)
);

ReactDOM.render(
  <Provider store={store}>
    <App />
  </Provider>,
  document.getElementById('root')
);
