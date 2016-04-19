/// <reference path="./types.d.ts" />

export interface Action {
  type: string;
}

export interface SetInputTextAction extends Action {
  text: string;
}

export const setInputText = function (text: string): SetInputTextAction {
  return {
    type: 'SET_INPUT_TEXT',
    text
  }
};

export interface PushStackAction extends Action {
  text: string;
}

export const pushStack = function (text: string): PushStackAction {
  return {
    type: 'PUSH_STACK',
    text
  }
};

export interface ExecuteOperatorAction extends Action {
  op: string;
}

export const executeOperator = function (op: string): ExecuteOperatorAction  {
  return {
    type: 'EXECUTE_OPERATOR',
    op
  }
};

export interface AppendInputAction extends Action {
  text: string;
}

export const appendInput = function (text: string): AppendInputAction {
  return {
    type: 'APPEND_INPUT',
    text
  }
};
