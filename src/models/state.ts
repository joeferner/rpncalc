/// <reference path="../types.d.ts" />

import StackItem from './stackItem';

export enum StateOutputMode {
  FIXED
}

export class StateOutput {
  mode: StateOutputMode;
  dp: number;
}

export default class State {
  input: string = '';
  stack: StackItem[] = [];
  base: number = 10;
  output: StateOutput = { mode: StateOutputMode.FIXED, dp: 20 };
  error: Error = null;
}