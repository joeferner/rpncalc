/// <reference path="../typings/tsd.d.ts" />

//import * as fs from 'fs';

interface nwjsWindow {
  showDevTools(): void;
}

interface nwjsWindowStatic {
  get(): nwjsWindow;
}

interface nwjsProcess {
  env: any
}

interface nwjsRoot {
  process: nwjsProcess
}

interface nwjsFS {
  writeFile(filename: string, data: any, callback?: (err: NodeJS.ErrnoException) => void): void;
  readFileSync(filename: string, encoding: string): string;
}

interface nwjs {
  Window: nwjsWindowStatic;
  root: nwjsRoot;
  fs: nwjsFS;
}

declare var nw: nwjs;
