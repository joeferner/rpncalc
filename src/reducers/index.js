import Decimal from 'decimal.js';

function clearInput(state) {
  return Object.assign({}, state, {
    input: ''
  });
}

function createStackItemFromDecimalValue(decimalValue) {
  return {
    value: decimalValue
  }
}

function unaryOpInPlaceValue(state, fn) {
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

function binaryOp(state, fn) {
  state = pushStack(state);
  let a = state.stack[state.stack.length - 2].value;
  let b = state.stack[state.stack.length - 1].value;
  let newValue = fn(a, b);
  state = popStack(state, 2);
  state = pushStack(state, newValue);
  return state;
}

function executeOperator(state, op) {
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

function popStack(state, count = 1) {
  return Object.assign({}, state, {
    stack: state.stack.slice(0, state.stack.length - count)
  })
}

function pushStack(state, newValue) {
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

export default function reducer(state, action) {
  if (typeof state === 'undefined') {
    return {
      input: '12',
      stack: [
        createStackItemFromDecimalValue(new Decimal(10)),
        createStackItemFromDecimalValue(new Decimal(22.2))
      ],
      base: 10
    };
  }
  
  switch(action.type) {
    case 'EXECUTE_OPERATOR':
      return executeOperator(state, action.op);
    case 'SET_INPUT_TEXT':
      return Object.assign({}, state, {
        input: action.text
      });
    case 'PUSH_STACK':
      return pushStack(state, action.text);      
    default:
      return state;
  }
}