'use strict';

var StackItem = require('./stackItem');
var Math2 = require('./mathHelpers');

var RpnCalc = module.exports = function() {
  this.clear();
};

RpnCalc.prototype.clear = function() {
  this.angleMode = 'rad';
  this.numBase = 10;
  this.stack = [];
  this.memory = {};
};

RpnCalc.prototype.loadState = function(state) {
  var self = this;
  this.angleMode = state.angleMode || 'rad';
  this.numBase = state.numBase || 10;
  this.stack = state.stack.map(function(item) {
    return new StackItem(item);
  });
  this.memory = {};
  Object.keys(state.memory).forEach(function(key) {
    self.memory[key] = new StackItem(state.memory[key]);
  });
};

RpnCalc.prototype.push = function(val) {
  var m;
  if (val.trim) {
    val = val.trim();
  }
  switch (val) {
  case 'pi':
    this.push(Math.PI);
    break;
  case 'rad':
    this.angleMode = 'rad';
    break;
  case 'deg':
    this.angleMode = 'deg';
    break;
  case 'hex':
    this.numBase = 16;
    break;
  case 'dec':
    this.numBase = 10;
    break;
  case 'oct':
    this.numBase = 8;
    break;
  case 'bin':
    this.numBase = 2;
    break;
  default:
    var numVal;
    if (this[val]) {
      this[val]();
    } else if (val.toString() == 'NaN') {
      console.log('isNaN', val);
      this.stack.unshift(new StackItem({ value: val, type: 'number' }));
    } else if ((numVal = this.tryParseNumber(val)) !== undefined) {
      console.log('tryParseNumber', val);
      this.stack.unshift(new StackItem({ value: numVal, type: 'number' }));
    } else if (m = val.match(/^'(.*)'$/)) {
      console.log('expr', val);
      this.stack.unshift(new StackItem({ value: m[1], type: 'expression' }));
    } else if (this.memory[val]) {
      console.log('memory', val);
      this.stack.unshift(this.memory[val]);
    } else {
      throw new Error("Could not parse expression");
    }
    break;
  }
};

RpnCalc.prototype.tryParseNumber = function(valStr) {
  var m, result;
  valStr = valStr.toString();
  if (m = valStr.match(/^0x([0-9a-fA-F]+)$/)) {
    result = parseInt(m[1], 16);
  } else if (m = valStr.match(/^0b([0-1]+)$/)) {
    result = parseInt(m[1], 2);
  } else if (m = valStr.match(/^0([0-8]+)$/)) {
    result = parseInt(m[1], 8);
  } else {
    result = parseFloat(valStr);
  }
  if (isNaN(result)) {
    return undefined;
  }
  return result;
};

RpnCalc.prototype.pop = function(count) {
  count = count || 1;
  if (this.stack.length < count) {
    throw new Error("Too few items on stack");
  }
  return this.stack.splice(0, count).reverse();
};

RpnCalc.prototype.popValues = function(count) {
  count = count || 1;
  if (this.stack.length < count) {
    throw new Error("Too few items on stack");
  }
  var results = [];
  for (var i = 0; i < count; i++) {
    results.push(this.stack[i].getValue());
  }
  this.stack.splice(0, count);
  return results.reverse();
};

RpnCalc.prototype.drop = function() {
  if (this.stack.length == 0) {
    throw new Error('Drop Error: Too few arguments');
  }
  this.stack = this.stack.slice(1);
};

RpnCalc.prototype.swap = function() {
  if (this.stack.length < 2) {
    throw new Error("Swap Error: Too few arguments");
  }
  var tmp = this.stack[0];
  this.stack[0] = this.stack[1];
  this.stack[1] = tmp;
};

RpnCalc.prototype.pow = function() {
  if (this.stack.length < 2) {
    throw new Error("Pow Error: Too few arguments");
  }
  var args = this.popValues(2);
  var result = Math.pow(args[0], args[1]);
  this.push(result);
};

RpnCalc.prototype.nroot = function() {
  if (this.stack.length < 2) {
    throw new Error("NRoot Error: Too few arguments");
  }
  var args = this.popValues(2);
  var result = Math.pow(args[0], 1 / args[1]);
  this.push(result);
};

RpnCalc.prototype.plus = function() {
  if (this.stack.length < 2) {
    throw new Error("Add Error: Too few arguments");
  }
  var args = this.popValues(2);
  var result = args[0] + args[1];
  this.push(result);
};

RpnCalc.prototype.subtract = function() {
  if (this.stack.length < 2) {
    throw new Error("Subtract Error: Too few arguments");
  }
  var args = this.popValues(2);
  var result = args[0] - args[1];
  this.push(result);
};

RpnCalc.prototype.multiply = function() {
  if (this.stack.length < 2) {
    throw new Error("Multiply Error: Too few arguments");
  }
  var args = this.popValues(2);
  var result = args[0] * args[1];
  this.push(result);
};

RpnCalc.prototype.divide = function() {
  if (this.stack.length < 2) {
    throw new Error("Divide Error: Too few arguments");
  }
  var args = this.popValues(2);
  var result = args[0] / args[1];
  this.push(result);
};

RpnCalc.prototype.toRadians = function(val) {
  switch (this.angleMode) {
  case 'rad':
    return val;
  case 'deg':
    return val * Math.PI / 180.0;
  default:
    throw new Error('Unhandled angle mode: ' + this.angleMode);
  }
};

RpnCalc.prototype.radiansToAngle = function(val) {
  switch (this.angleMode) {
  case 'rad':
    return val;
  case 'deg':
    return val * 180.0 / Math.PI;
  default:
    throw new Error('Unhandled angle mode: ' + this.angleMode);
  }
};

RpnCalc.prototype.sin = function() {
  if (this.stack.length < 1) {
    throw new Error("Sin Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = Math.sin(this.toRadians(args[0]));
  this.push(result);
};

RpnCalc.prototype.cos = function() {
  if (this.stack.length < 1) {
    throw new Error("Cos Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = Math.cos(this.toRadians(args[0]));
  this.push(result);
};

RpnCalc.prototype.tan = function() {
  if (this.stack.length < 1) {
    throw new Error("Tan Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = Math.tan(this.toRadians(args[0]));
  this.push(result);
};

RpnCalc.prototype.asin = function() {
  if (this.stack.length < 1) {
    throw new Error("Asin Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = this.radiansToAngle(Math.asin(args[0]));
  this.push(result);
};

RpnCalc.prototype.acos = function() {
  if (this.stack.length < 1) {
    throw new Error("Acos Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = this.radiansToAngle(Math.acos(args[0]));
  this.push(result);
};

RpnCalc.prototype.atan = function() {
  if (this.stack.length < 1) {
    throw new Error("Atan Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = this.radiansToAngle(Math.atan(args[0]));
  this.push(result);
};

RpnCalc.prototype.atan2 = function() {
  if (this.stack.length < 2) {
    throw new Error("Atan2 Error: Too few arguments");
  }
  var args = this.popValues(2);
  var result = this.radiansToAngle(Math.atan2(args[0], args[1]));
  this.push(result);
};

RpnCalc.prototype.sqrt = function() {
  if (this.stack.length < 1) {
    throw new Error("Sqrt Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = Math.sqrt(args[0]);
  this.push(result);
};

RpnCalc.prototype.fact = RpnCalc.prototype.factorial = function() {
  if (this.stack.length < 1) {
    throw new Error("Fact Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = Math2.factorial(args[0]);
  this.push(result);
};

RpnCalc.prototype.pow2 = function() {
  if (this.stack.length < 1) {
    throw new Error("Pow2 Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = Math.pow(args[0], 2);
  this.push(result);
};

RpnCalc.prototype.inv = function() {
  if (this.stack.length < 1) {
    throw new Error("1/x Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = 1 / args[0];
  this.push(result);
};

RpnCalc.prototype.log = function() {
  if (this.stack.length < 1) {
    throw new Error("Log Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = Math.log(args[0]) / Math.log(10);
  this.push(result);
};

RpnCalc.prototype.ln = function() {
  if (this.stack.length < 1) {
    throw new Error("Ln Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = Math.log(args[0]);
  this.push(result);
};

RpnCalc.prototype.exp = function() {
  if (this.stack.length < 1) {
    throw new Error("Exp Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = Math.exp(args[0]);
  this.push(result);
};

RpnCalc.prototype.neg = function() {
  if (this.stack.length < 1) {
    throw new Error("Neg Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = -args[0];
  this.push(result);
};

RpnCalc.prototype.sto = RpnCalc.prototype.store = function() {
  if (this.stack.length < 2) {
    throw new Error("Sto Error: Too few arguments");
  }
  if (this.stack[0].type != 'expression') {
    throw new Error("Sto Error: 2nd arg must be expression");
  }
  var args = this.pop(2);
  var variable = args[1];
  var val = args[0];
  this.memory[variable.value] = val;
};