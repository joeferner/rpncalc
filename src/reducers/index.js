"use strict";
/// <reference path="../types.d.ts" />
var Decimal = require("decimal.js");
var state_1 = require("../models/state");
var path = require("path");
var nw = require("nw");
var utils = require("../utils");
var Qty = require('js-quantities');
var PI = Decimal.acos(-1);
function clearInput(state) {
    return Object.assign({}, state, {
        input: ''
    });
}
function createStackItemFromDecimalValue(decimalValue) {
    return {
        value: decimalValue
    };
}
function unaryOpInPlaceValue(state, fn) {
    var input = state.input.trim();
    if (input.length > 0) {
        return Object.assign({}, state, {
            input: fn(input)
        });
    }
    else {
        var newValue = fn(state.stack[state.stack.length - 1].value);
        state = popStack(state, 1);
        return pushStack(state, newValue);
    }
}
function setError(state, error) {
    return Object.assign({}, state, {
        error: error
    });
}
function binaryOp(state, fn) {
    state = pushStack(state);
    if (state.stack.length < 2) {
        return setError(state, new Error("Not enough items on stack"));
    }
    var a = state.stack[state.stack.length - 2].value;
    var b = state.stack[state.stack.length - 1].value;
    var newValue = fn(a, b);
    state = popStack(state, 2);
    state = pushStack(state, newValue);
    return state;
}
function unaryOp(state, fn) {
    state = pushStack(state);
    if (state.stack.length < 1) {
        return setError(state, new Error("Not enough items on stack"));
    }
    var a = state.stack[state.stack.length - 1].value;
    var newValue = fn(a);
    state = popStack(state, 1);
    state = pushStack(state, newValue);
    return state;
}
function swap(state) {
    state = pushStack(state);
    state = clearInput(state);
    if (state.stack.length < 2) {
        return setError(state, new Error("Not enough items on stack"));
    }
    var a = state.stack[state.stack.length - 2].value;
    var b = state.stack[state.stack.length - 1].value;
    state = popStack(state, 2);
    state = pushStack(state, b);
    state = pushStack(state, a);
    return state;
}
function performEval(state) {
    state = pushStack(state);
    state = clearInput(state);
    if (state.stack.length < 1) {
        return setError(state, new Error("Not enough items on stack"));
    }
    var v = state.stack[state.stack.length - 1].value;
    var m = v.match(/^'(.*?)'$/);
    if (!m) {
        return setError(state, new Error("Invalid expression"));
    }
    v = eval(m[1]);
    state = popStack(state, 1);
    state = pushStack(state, v);
    return state;
}
function convert(state) {
    state = pushStack(state);
    state = clearInput(state);
    if (state.stack.length < 2) {
        return setError(state, new Error("Not enough items on stack"));
    }
    var a = state.stack[state.stack.length - 2].value;
    var b = state.stack[state.stack.length - 1].value;
    var m = a.match(/^'(.+?)'$/);
    if (!m) {
        return setError(state, new Error("Invalid expression to convert from: " + a));
    }
    var fromValue = new Qty(m[1]);
    m = b.match(/^'(.+)'$/);
    if (!m) {
        return setError(state, new Error("Invalid expression to convert to: " + b));
    }
    var toUnits = m[1];
    var v = "'" + fromValue.to(toUnits).toString() + "'";
    state = popStack(state, 2);
    state = pushStack(state, v);
    return state;
}
function store(state) {
    state = pushStack(state);
    state = clearInput(state);
    if (state.stack.length < 2) {
        return setError(state, new Error("Not enough items on stack"));
    }
    var a = state.stack[state.stack.length - 2].value;
    var b = state.stack[state.stack.length - 1].value;
    var m = b.match(/^'(.+?)'$/);
    if (!m) {
        return setError(state, new Error("Invalid expression"));
    }
    var name = m[1];
    var newStoreItem = {};
    newStoreItem[name] = a;
    state = Object.assign({}, state, {
        store: Object.assign({}, state.store, newStoreItem)
    });
    state = popStack(state, 2);
    return state;
}
function toFraction(state) {
    state = pushStack(state);
    state = clearInput(state);
    if (state.stack.length < 1) {
        return setError(state, new Error("Not enough items on stack"));
    }
    var v = state.stack[state.stack.length - 1].value;
    v = v.toFraction(10000);
    v = "'" + v[0] + '/' + v[1] + "'";
    state = popStack(state, 1);
    state = pushStack(state, v);
    return state;
}
function toRadians(val, fromAngleMode) {
    switch (fromAngleMode) {
        case state_1.AngleMode.RADIANS:
            return val;
        case state_1.AngleMode.DEGREES:
            return val.mul(PI.div(180));
        default:
            throw new Error('Unhandled angle mode: ' + fromAngleMode);
    }
}
function radiansToAngleMode(radians, angleMode) {
    switch (angleMode) {
        case state_1.AngleMode.RADIANS:
            return radians;
        case state_1.AngleMode.DEGREES:
            return radians.mul(new Decimal(180).div(PI));
        default:
            throw new Error('Unhandled angle mode: ' + angleMode);
    }
}
function convertAngle(state, fromAngleMode, toAngleMode) {
    state = pushStack(state);
    if (state.stack.length < 1) {
        return setError(state, new Error("Not enough items on stack"));
    }
    var val = state.stack[state.stack.length - 1].value;
    val = toRadians(val, fromAngleMode);
    val = radiansToAngleMode(val, toAngleMode);
    state = popStack(state, 1);
    state = pushStack(state, val);
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
function dup(state) {
    if (state.stack.length < 1) {
        return setError(state, new Error("Not enough items on stack"));
    }
    return pushStack(state, state.stack[state.stack.length - 1].value);
}
function changeAngleMode(state, newAngleMode) {
    return Object.assign({}, state, {
        angleMode: newAngleMode
    });
}
function changeBase(state, newBase) {
    return Object.assign({}, state, {
        output: Object.assign({}, state.output, {
            base: newBase
        })
    });
}
function isOperator(op) {
    switch (op) {
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
        case 'fraction':
        case 'store':
            return true;
        default:
            return false;
    }
}
function executeOperator(state, op) {
    switch (op) {
        case 'dec':
            return changeBase(state, 10);
        case 'bin':
            return changeBase(state, 2);
        case 'oct':
            return changeBase(state, 8);
        case 'hex':
            return changeBase(state, 16);
        case 'deg':
            return changeAngleMode(state, state_1.AngleMode.DEGREES);
        case 'rad':
            return changeAngleMode(state, state_1.AngleMode.RADIANS);
        case 'deg2rad':
            return convertAngle(state, state_1.AngleMode.DEGREES, state_1.AngleMode.RADIANS);
        case 'rad2deg':
            return convertAngle(state, state_1.AngleMode.RADIANS, state_1.AngleMode.DEGREES);
        case 'swap':
            return swap(state);
        case 'eval':
            return performEval(state);
        case 'convert':
            return convert(state);
        case 'fraction':
            return toFraction(state);
        case 'store':
            return store(state);
        case '+':
        case '-':
        case '*':
        case '/':
        case 'pow':
        case 'xroot':
            state = pushStack(state);
            state = clearInput(state);
            state = binaryOp(state, function (a, b) {
                switch (op) {
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
            state = unaryOp(state, function (a) {
                switch (op) {
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
            state = unaryOpInPlaceValue(state, function (value) {
                if (typeof value === 'string') {
                    if (value[0] == '-') {
                        return value.substr(1);
                    }
                    else {
                        return '-' + value;
                    }
                }
                else {
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
function parseItem(state, value) {
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
        if (state.store && state.store[value]) {
            return parseItem(state, state.store[value]);
        }
        value = value.replace(/[, ]/g, '');
        try {
            return new Decimal(value);
        }
        catch (e) {
            throw new Error('Could not parse decimal: ' + value);
        }
    }
    if (value instanceof Decimal) {
        return value;
    }
    console.error('Unexpected value', value);
    throw new Error('Unexpected value "' + value + '" (type: ' + (typeof value) + ')');
}
function popStack(state, count) {
    if (count === void 0) { count = 1; }
    return Object.assign({}, state, {
        stack: state.stack.slice(0, state.stack.length - count)
    });
}
function pushStack(state, newValue) {
    if (newValue === void 0) { newValue = null; }
    newValue = newValue || state.input;
    if (newValue.length === 0) {
        return state;
    }
    if (isOperator(newValue)) {
        return executeOperator(clearInput(state), newValue);
    }
    var dec = parseItem(state, newValue);
    return Object.assign({}, state, {
        input: '',
        stack: state.stack.concat([
            createStackItemFromDecimalValue(dec)
        ])
    });
}
function getUserHome() {
    var processEnv = nw.processEnv;
    var userHome = processEnv.USERPROFILE || processEnv.HOME || processEnv.USER;
    if (!userHome) {
        throw new Error('Could not get user home directory');
    }
    return userHome;
}
function getStateSavePath() {
    return path.join(getUserHome(), '.rpncalc', 'state.json');
}
function saveState(state) {
    if (state.error) {
        return;
    }
    state = Object.assign({}, state, {
        stack: state.stack.map(function (item) {
            return Object.assign({}, item, {
                value: item.value.toString()
            });
        })
    });
    var data = JSON.stringify(state, null, 2);
    if (typeof nw !== 'undefined') {
        var stateSavePath = getStateSavePath();
        console.log('saving state', stateSavePath, state);
        if (!nw.fs.existsSync(path.dirname(stateSavePath))) {
            nw.fs.mkdirSync(path.dirname(stateSavePath));
        }
        nw.fs.writeFile(stateSavePath, data);
    }
    else if (typeof (Storage) !== 'undefined') {
        console.log('saving state', state);
        localStorage.setItem('state', data);
    }
    else {
        console.error('could not find storage for state');
    }
}
function loadState() {
    if (typeof nw !== 'undefined') {
        try {
            var stateSavePath = getStateSavePath();
            console.log('loading state', stateSavePath);
            var data = nw.fs.readFileSync(stateSavePath, 'utf8');
            return convertDecimals(data);
        }
        catch (e) {
            return new state_1["default"]();
        }
    }
    else if (typeof (Storage) !== 'undefined') {
        var stateStr = localStorage.getItem('state');
        if (stateStr) {
            return convertDecimals(stateStr);
        }
        else {
            return new state_1["default"]();
        }
    }
    else {
        return new state_1["default"]();
    }
    function convertDecimals(data) {
        try {
            var state = JSON.parse(data);
            state.stack.forEach(function (item) {
                if (!utils.isExpression(item.value)) {
                    item.value = new Decimal(item.value);
                }
            });
            return state;
        }
        catch (e) {
            console.error('could not parse state: ' + data, e);
            return new state_1["default"]();
        }
    }
}
function setDigitGrouping(state, digitGrouping) {
    return Object.assign({}, state, {
        output: Object.assign({}, state.output, {
            digitGrouping: digitGrouping
        })
    });
}
function reducer(state, action) {
    if (typeof state === 'undefined') {
        return loadState();
    }
    state = setError(state, null);
    try {
        switch (action.type) {
            case 'EXECUTE_OPERATOR':
                state = executeOperator(state, action.op);
                saveState(state);
                return state;
            case 'SET_INPUT_TEXT':
                return Object.assign({}, state, {
                    input: action.text
                });
            case 'APPEND_INPUT':
                return Object.assign({}, state, {
                    input: state.input + action.text
                });
            case 'PUSH_STACK':
                state = pushStack(state, action.text);
                saveState(state);
                return state;
            case 'SET_DIGIT_GROUPING':
                state = setDigitGrouping(state, action.digitGrouping);
                saveState(state);
                return state;
            default:
                console.error('unhandled action: ' + action.type);
                return state;
        }
    }
    catch (e) {
        console.error('reduce error', e);
        return setError(state, e);
    }
}
exports.__esModule = true;
exports["default"] = reducer;
