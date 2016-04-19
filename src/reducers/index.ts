/// <reference path="../types.d.ts" />

import * as Decimal from 'decimal.js';
import State from '../models/state';
import * as actions from '../actions';

const PI = '3.14159265358979323846264338327950288419716939937510'
           + '58209749445923078164062862089986280348253421170679'
           + '82148086513282306647093844609550582231725359408128'
           + '48111745028410270193852110555964462294895493038196'
           + '44288109756659334461284756482337867831652712019091'
           + '45648566923460348610454326648213393607260249141273'
           + '72458700660631558817488152092096282925409171536436'
           + '78925903600113305305488204665213841469519415116094'
           + '33057270365759591953092186117381932611793105118548';

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

function setError(state: State, error: Error): State {
  return Object.assign({}, state, {
    error: error
  });
}

function binaryOp(state: State, fn: (a: any, b: any) => any): State {
  state = pushStack(state);
  if (state.stack.length < 2) {
    return setError(state, new Error("Not enough items on stack"));
  }
  let a = state.stack[state.stack.length - 2].value;
  let b = state.stack[state.stack.length - 1].value;
  let newValue = fn(a, b);
  state = popStack(state, 2);
  state = pushStack(state, newValue);
  return state;
}

function unaryOp(state: State, fn: (a: any) => any): State {
  state = pushStack(state);
  if (state.stack.length < 1) {
    return setError(state, new Error("Not enough items on stack"));
  }
  let a = state.stack[state.stack.length - 1].value;
  let newValue = fn(a);
  state = popStack(state, 1);
  state = pushStack(state, newValue);
  return state;
}

function swap(state: State): State {
  state = pushStack(state);
  state = clearInput(state);
  if (state.stack.length < 2) {
    return setError(state, new Error("Not enough items on stack"));
  }
  let a = state.stack[state.stack.length - 2].value;
  let b = state.stack[state.stack.length - 1].value;
  state = popStack(state, 2);
  state = pushStack(state, b);
  state = pushStack(state, a);
  return state;
}

function xroot(a, b) {
  if (b.toString() == '2') {
    return a.sqrt();
  }
  if (b.toString() == '3') {
    return a.cbrt();
  }
  return new Decimal(Math.pow(a.toNumber(), b.toNumber()));
}

function executeOperator(state: State, op): State {
  switch(op) {
    case 'swap':
      return swap(state);
    
    case '+':
    case '-':
    case '*':
    case '/':
    case 'pow':
    case 'xroot':
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
          case 'pow':
            return a.pow(b);
          case 'xroot':
            return xroot(a, b);
          default:
            throw new Error('unhandled op: ' + op);
        }
      });
      break;
      
    case 'sin':
    case 'cos':
    case 'tan':
    case 'asin':
    case 'acos':
    case 'atan':
    case 'sqrt':
    case 'log':
    case 'ln':
    case 'pow2':
    case 'exp':
    case 'inv':
      state = pushStack(state);
      state = clearInput(state);
      state = unaryOp(state, (a) => {
        switch(op) {
          case 'sin':
            return a.sin();
          case 'cos':
            return a.cos();
          case 'tan':
            return a.tan();
          case 'asin':
            return a.asin();
          case 'acos':
            return a.acos();
          case 'atan':
            return a.atan();
          case 'sqrt':
            return a.sqrt();
          case 'log':
            return a.log();
          case 'ln':
            return a.ln();
          case 'pow2':
            return a.pow(2);
          case 'exp':
            return a.exp();
          case 'inv':
            return new Decimal(1).div(a);
          default:
            throw new Error('unhandled op: ' + op);
        }
      });
      break;
      
    case 'neg':
      state = unaryOpInPlaceValue(state, (value) => {
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
      return setError(state, new Error('Invalid op "' + op + '"'));
  }
  return state;
}

function toDecimal(value) {
  if (typeof value === 'string') {
    value = value.trim();
    if (value === 'pi') {
      value = PI;
    }
    return new Decimal(value);
  } else if (value instanceof Decimal) {
    return value;
  } else {
    console.error('Unexpected value', value);
    throw new Error('Unexpected value "' + value + '" (type: ' + (typeof value) + ')');
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

  state = setError(state, null);
  try {
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
  } catch(e) {
    console.error('reduce error', e);
    return setError(state, e);
  }
}