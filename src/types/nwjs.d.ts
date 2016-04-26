/// <reference path="../../typings/node/node.d.ts" />

//import * as fs from 'fs';

declare module 'nw' {
  const root: Root;
  const fs: FS;

  interface FS {
    existsSync(filename: string): boolean;
    mkdirSync(filename: string): void;
    writeFile(filename: string, data: any, callback?: (err: NodeJS.ErrnoException) => void): void;
    readFileSync(filename: string, encoding: string): string;
  }

  interface Process {
    env: any
  }

  interface Root {
    process: Process
  }
  
  interface MenuOptions {
    type?: string
  }
  
  class Menu {
    constructor(options?: MenuOptions);
    append(menuItem: MenuItem);
  }
  
  type MenuItemType = 'normal' | 'checkbox' | 'separator';
  
  interface MenuItemOptions {
    label?: string,
    icon?: string,
    tooltop?: string,
    type?: MenuItemType,
    click?: () => any,
    enabled?: boolean,
    checked?: boolean,
    submenu?: Menu,
    key?: string,
    modifiers?: string
  }
  
  class MenuItem {
    constructor(options: MenuItemOptions);
  }
  
  class Window {
    static get(): Window;

    menu: Menu;
    
    showDevTools(): void;
  }
}
