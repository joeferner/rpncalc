/// <reference path="../typings/tsd.d.ts" />

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
