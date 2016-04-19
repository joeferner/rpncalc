/// <reference path="../types.d.ts" />

import StackItem from './stackItem';

export default class State {
  input: string = '';
  stack: StackItem[] = [];
  base: number = 10;
  error: Error = null;
}