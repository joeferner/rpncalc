/// <reference path="../types.d.ts" />

import StackItem from './stackItem';

export enum DecimalJsOutputMode {
  FIXED
}

export enum AngleMode {
  DEGREES,
  RADIANS
}

export class StateOutput {
  decimalJsMode: DecimalJsOutputMode;
  numberOfDecimalPoints: number;
  base: number;
  digitGrouping: boolean;
}

export default class State {
  input: string = '';
  stack: StackItem[] = [];
  angleMode: AngleMode = AngleMode.DEGREES;
  output: StateOutput = {
    decimalJsMode: DecimalJsOutputMode.FIXED,
    numberOfDecimalPoints: 20,
    base: 10,
    digitGrouping: false
  };
  error: Error = null;
}