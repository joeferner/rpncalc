/// <reference path="../types.d.ts" />

import * as Decimal from 'decimal.js';
import State from '../models/state';
import { AngleMode } from '../models/state';
import * as actions from '../actions';
import * as path from 'path';
import * as nw from 'nw';
import * as utils from '../utils';
var Qty = require('js-quantities');

const PI = Decimal.acos(-1);

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
    state = popStack(state, 1);
    return pushStack(state, newValue);
  }
}

function setError(state: State, error: Error): State {
  return Object.assign({}, state, {
    error: error
  });
}

function binaryOp(state: State, fn: (a: decimal.Decimal, b: decimal.Decimal) => decimal.Decimal): State {
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

function unaryOp(state: State, fn: (a: decimal.Decimal) => decimal.Decimal): State {
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

function performEval(state: State): State {
  state = pushStack(state);
  state = clearInput(state);
  if (state.stack.length < 1) {
    return setError(state, new Error("Not enough items on stack"));
  }
  let v = state.stack[state.stack.length - 1].value;
  let m = v.match(/^'(.*?)'$/);
  if (!m) {
    return setError(state, new Error("Invalid expression"));
  }
  v = eval(m[1]);
  state = popStack(state, 1);
  state = pushStack(state, v);
  return state;
}

function convert(state: State): State {
  state = pushStack(state);
  state = clearInput(state);
  if (state.stack.length < 2) {
    return setError(state, new Error("Not enough items on stack"));
  }
  let a = state.stack[state.stack.length - 2].value;
  let b = state.stack[state.stack.length - 1].value;
  
  let m = a.match(/^'(.+?)'$/);
  if (!m) {
    return setError(state, new Error("Invalid expression to convert from: " + a));
  }
  let fromValue = new Qty(m[1]);
  
  m = b.match(/^'(.+)'$/);
  if (!m) {
    return setError(state, new Error("Invalid expression to convert to: " + b));
  }
  let toUnits = m[1];
  
  let v = "'" + fromValue.to(toUnits).toString() + "'";
  
  state = popStack(state, 2);
  state = pushStack(state, v);
  return state;
}

function toRadians(val: decimal.Decimal, fromAngleMode: AngleMode): decimal.Decimal {
  switch (fromAngleMode) {
    case AngleMode.RADIANS:
      return val;
    case AngleMode.DEGREES:
      return val.mul(PI.div(180));
    default:
      throw new Error('Unhandled angle mode: ' + fromAngleMode);
  }
}

function radiansToAngleMode(radians: decimal.Decimal, angleMode: AngleMode): decimal.Decimal {
  switch (angleMode) {
    case AngleMode.RADIANS:
      return radians;
    case AngleMode.DEGREES:
      return radians.mul(new Decimal(180).div(PI));
    default:
      throw new Error('Unhandled angle mode: ' + angleMode);
  }
}

function convertAngle(state: State, fromAngleMode: AngleMode, toAngleMode: AngleMode): State {
  state = pushStack(state);
  if (state.stack.length < 1) {
    return setError(state, new Error("Not enough items on stack"));
  }
  let val = state.stack[state.stack.length - 1].value;
  val = toRadians(val, fromAngleMode);
  val = radiansToAngleMode(val, toAngleMode);
  state = popStack(state, 1);
  state = pushStack(state, val);
  return state;  
}

function xroot(a: decimal.Decimal, b: decimal.Decimal): decimal.Decimal {
  if (b.toString() == '2') {
    return a.sqrt();
  }
  if (b.toString() == '3') {
    return a.cbrt();
  }
  return new Decimal(Math.pow(a.toNumber(), b.toNumber()));
}

function dup(state: State): State {
  if (state.stack.length < 1) {
    return setError(state, new Error("Not enough items on stack"));
  }
  return pushStack(state, state.stack[state.stack.length - 1].value);
}

function changeAngleMode(state: State, newAngleMode: AngleMode): State {
  return Object.assign({}, state, {
    angleMode: newAngleMode
  });
}

function changeBase(state: State, newBase: number): State {
  return Object.assign({}, state, {
    output: Object.assign({}, state.output, {
      base: newBase
    })
  });
}

function isOperator(op: string): boolean {
  switch(op) {
    case 'dec':
    case 'bin':
    case 'oct':
    case 'hex':
    case 'deg':
    case 'rad':
    case 'deg2rad':
    case 'rad2deg':
    case 'swap':
    case '+':
    case '-':
    case '*':
    case '/':
    case 'pow':
    case 'xroot':
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
    case 'neg':
    case 'drop':
    case 'dup':
    case 'eval':
    case 'convert':
      return true;
      
    default:
      return false;
  }
}

function executeOperator(state: State, op): State {
  switch(op) {
    case 'dec':
      return changeBase(state, 10);
    case 'bin':
      return changeBase(state, 2);
    case 'oct':
      return changeBase(state, 8);
    case 'hex':
      return changeBase(state, 16);
    
    case 'deg':
      return changeAngleMode(state, AngleMode.DEGREES);
    case 'rad':
      return changeAngleMode(state, AngleMode.RADIANS);
    
    case 'deg2rad':
      return convertAngle(state, AngleMode.DEGREES, AngleMode.RADIANS);
    case 'rad2deg':
      return convertAngle(state, AngleMode.RADIANS, AngleMode.DEGREES);
    
    case 'swap':
      return swap(state);
    
    case 'eval':
      return performEval(state);

    case 'convert':
      return convert(state);
    
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
            return toRadians(a, state.angleMode).sin();
          case 'cos':
            return toRadians(a, state.angleMode).cos();
          case 'tan':
            return toRadians(a, state.angleMode).tan();
          case 'asin':
            return radiansToAngleMode(a.asin(), state.angleMode);
          case 'acos':
            return radiansToAngleMode(a.acos(), state.angleMode);
          case 'atan':
            return radiansToAngleMode(a.atan(), state.angleMode);
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

    case 'dup':
      state = dup(state);
      break;
      
    default:
      return setError(state, new Error('Invalid op "' + op + '"'));
  }
  return state;
}

function toDecimal(value) {
  if (typeof value === 'number') {
    return new Decimal(value);
  }
  
  if (typeof value === 'string') {
    value = value.trim();
    if (value === 'pi') {
      value = PI;
    }
    if (utils.isExpression(value)) {
      return value;
    }
    value = value.replace(/[, ]/g, '');
    return new Decimal(value);
  }
  
  if (value instanceof Decimal) {
    return value;
  }
  
  console.error('Unexpected value', value);
  throw new Error('Unexpected value "' + value + '" (type: ' + (typeof value) + ')');
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
  
  if (isOperator(newValue)) {
    return executeOperator(clearInput(state), newValue);
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

function getUserHome(): string {
  var env = nw.root.process.env;
  return env.USERPROFILE || env.HOME || env.USER;
}

function getStateSavePath(): string {
  return path.join(getUserHome(), '.rpncalc', 'state.json');;
}

function saveState(state: State) {
  if (state.error) {
    return;
  }
  
  state = Object.assign({}, state, {
    stack: state.stack.map((item) => {
      return Object.assign({}, item, {
        value: item.value.toString()
      });
    })
  });

  var data = JSON.stringify(state, null, 2);
  
  if (typeof nw !== 'undefined') {
    var stateSavePath = getStateSavePath();
    console.log('saving state', stateSavePath, state);
    if(!nw.fs.existsSync(path.dirname(stateSavePath))) {
      nw.fs.mkdirSync(path.dirname(stateSavePath));
    }
    nw.fs.writeFile(stateSavePath, data);
  } else if (typeof(Storage) !== 'undefined') {
    console.log('saving state', state);
    localStorage.setItem('state', data);
  } else {
    console.error('could not find storage for state');
  }
}

function loadState(): State {
  if (typeof nw !== 'undefined') {
    try {
      var stateSavePath = getStateSavePath();
      console.log('loading state', stateSavePath);
      var data = nw.fs.readFileSync(stateSavePath, 'utf8');
      return convertDecimals(data);
    } catch(e) {
      return new State();
    }
  } else if (typeof(Storage) !== 'undefined') {
    var stateStr = localStorage.getItem('state');
    if (stateStr) {
      return convertDecimals(stateStr);
    } else {
      return new State();
    }
  } else {
    return new State();
  }
  
  function convertDecimals(data: string): State {
    try {
      var state = JSON.parse(data);
      state.stack.forEach((item) => {
        if (!utils.isExpression(item.value)) {
          item.value = new Decimal(item.value);  
        }        
      });
      return state;
    } catch(e) {
      console.error('could not parse state: ' + data, e);
      return new State();
    }
  }
}

function setDigitGrouping(state: State, digitGrouping: boolean): State {
  return Object.assign({}, state, {
    output: Object.assign({}, state.output, {
      digitGrouping: digitGrouping
    })
  });
}

export default function reducer(state: State, action: actions.Action): State {
  if (typeof state === 'undefined') {
    return loadState();
  }

  state = setError(state, null);
  try {
    switch(action.type) {
      case 'EXECUTE_OPERATOR':
        state = executeOperator(state, (<actions.ExecuteOperatorAction>action).op);
        saveState(state);
        return state;
      case 'SET_INPUT_TEXT':
        return Object.assign({}, state, {
          input: (<actions.SetInputTextAction>action).text
        });
      case 'APPEND_INPUT':
        return Object.assign({}, state, {
          input: state.input + (<actions.AppendInputAction>action).text
        });
      case 'PUSH_STACK':
        state = pushStack(state, (<actions.PushStackAction>action).text);
        saveState(state);
        return state;
      case 'SET_DIGIT_GROUPING':
        state = setDigitGrouping(state, (<actions.SetDigitGroupingAction>action).digitGrouping);
        saveState(state);
        return state;
      default:
        console.error('unhandled action: ' + action.type);
        return state;
    }
  } catch(e) {
    console.error('reduce error', e);
    return setError(state, e);
  }
}
