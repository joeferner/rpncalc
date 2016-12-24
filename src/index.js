/// <reference path="./types.d.ts" />
"use strict";
var React = require('react');
var ReactDOM = require('react-dom');
var app_tsx_1 = require('./components/app.tsx');
var react_redux_1 = require('react-redux');
var redux_1 = require('redux');
var redux_thunk_1 = require('redux-thunk');
var createLogger = require('redux-logger');
var reducers_1 = require('./reducers');
var logger = createLogger();
var storeMiddleware = redux_1.applyMiddleware(redux_thunk_1["default"], logger);
var store = redux_1.createStore(reducers_1["default"], storeMiddleware);
ReactDOM.render(React.createElement(react_redux_1.Provider, {store: store}, React.createElement(app_tsx_1["default"], null)), document.getElementById('root'));
