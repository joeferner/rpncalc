/// <reference path="../types.d.ts" />

import * as Decimal from 'decimal.js';
import State from '../models/state';
import * as actions from '../actions';

function clearInput(state: State): State {
  return Object.assign({}, state, {
    input: ''
  });
}

function createStackItemFromDecimalValue(decimalValue) {
  return {
    value: decimalValue
  }
}

function unaryOpInPlaceValue(state: State, fn): State {
  var input = state.input.trim();
  if (input.length > 0) {
    return Object.assign({}, state, {
      input: fn(input)
    });
  } else {
    let newValue = fn(state.stack[state.stack.length - 1].value);
    state = popStack(state, 2);
    return pushStack(state, newValue);
  }
}

function binaryOp(state: State, fn): State {
  state = pushStack(state);
  if (state.stack.length < 2) {
    return Object.assign({}, state, {
      error: "Not enough items on stack"
    });
  }
  let a = state.stack[state.stack.length - 2].value;
  let b = state.stack[state.stack.length - 1].value;
  let newValue = fn(a, b);
  state = popStack(state, 2);
  state = pushStack(state, newValue);
  return state;
}

function executeOperator(state: State, op): State {
  switch(op) {
    case '+':
    case '-':
    case '*':
    case '/':
      state = pushStack(state);
      state = clearInput(state);
      state = binaryOp(state, (a, b) => {
        switch(op) {
          case '+':
            return a.add(b);
          case '-':
            return a.sub(b);
          case '*':
            return a.mul(b);
          case '/':
            return a.div(b);
          default:
            throw new Error('unhandled op: ' + op);
        }
      });
      break;
      
    case 'neg':
      state = unaryOpInPlaceValue(state, (value) => {
        console.log(value, typeof value);
        if (typeof value === 'string') {
          if (value[0] == '-') {
            return value.substr(1);
          } else {
            return '-' + value;
          }
        } else {
          return value.neg();
        }
      });
      break;
      
    case 'drop':
      state = popStack(state);
      break;
      
    default:
      console.error('unhandle op: ' + op);
      break;
  }
  return state;
}

function toDecimal(value) {
  if (typeof value === 'string') {
    return new Decimal(value);
  } else {
    return value;
  }
}

function popStack(state: State, count = 1): State {
  return Object.assign({}, state, {
    stack: state.stack.slice(0, state.stack.length - count)
  });
}

function pushStack(state: State, newValue = null): State {
  newValue = newValue || state.input; 
  if (newValue.length === 0) {
    return state;
  }
  let dec = toDecimal(newValue);
  return Object.assign({}, state, {
    input: '',
    stack: [
      ...state.stack,
      createStackItemFromDecimalValue(dec)
    ]
  });
}

export default function reducer(state: State, action: actions.Action): State {
  if (typeof state === 'undefined') {
    return new State();
  }
  
  state = Object.assign({}, state, {
    error: null
  });
  
  switch(action.type) {
    case 'EXECUTE_OPERATOR':
      return executeOperator(state, (<actions.ExecuteOperatorAction>action).op);
    case 'SET_INPUT_TEXT':
      return Object.assign({}, state, {
        input: (<actions.SetInputTextAction>action).text
      });
    case 'APPEND_INPUT':
      return Object.assign({}, state, {
        input: state.input + (<actions.AppendInputAction>action).text
      });
    case 'PUSH_STACK':
      return pushStack(state, (<actions.PushStackAction>action).text);      
    default:
      return state;
  }
}