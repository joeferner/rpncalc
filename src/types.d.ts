/// <reference path="../typings/react/react.d.ts" />
/// <reference path="../typings/react/react-dom.d.ts" />
/// <reference path="../typings/react-redux/react-redux.d.ts" />
/// <reference path="../typings/decimal.js/decimal.js.d.ts" />
/// <reference path="../typings/es6-shim/es6-shim.d.ts" />
/// <reference path="../typings/redux-logger/redux-logger.d.ts" />
/// <reference path="../typings/redux-thunk/redux-thunk.d.ts" />
/// <reference path="../typings/redux-promise/redux-promise.d.ts" />

interface nwjsWindow {
  showDevTools(): void;
}

interface nwjsWindowStatic {
  get(): nwjsWindow;
}

interface nwjs {
  Window: nwjsWindowStatic;
}

declare var nw: nwjs;
