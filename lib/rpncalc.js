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

RpnCalc.prototype.rad = function() {
  this.angleMode = 'rad';
};
RpnCalc.prototype.rad.description = "Sets the angle mode to radians";

RpnCalc.prototype.deg = function() {
  this.angleMode = 'deg';
};
RpnCalc.prototype.deg.description = "Sets the angle mode to degrees";

RpnCalc.prototype.hex = function() {
  this.numBase = 16;
};
RpnCalc.prototype.hex.description = "Sets the base to 16";

RpnCalc.prototype.dec = function() {
  this.numBase = 10;
};
RpnCalc.prototype.dec.description = "Sets the base to 10";

RpnCalc.prototype.oct = function() {
  this.numBase = 8;
};
RpnCalc.prototype.oct.description = "Sets the base to 8";

RpnCalc.prototype.bin = function() {
  this.numBase = 2;
};
RpnCalc.prototype.bin.description = "Sets the base to 2";

RpnCalc.prototype.drop = function() {
  if (this.stack.length == 0) {
    throw new Error('Drop Error: Too few arguments');
  }
  this.stack = this.stack.slice(1);
};
RpnCalc.prototype.drop.description = "Removes an item from the stack (alias: backspace key)";

RpnCalc.prototype.swap = function() {
  if (this.stack.length < 2) {
    throw new Error("Swap Error: Too few arguments");
  }
  var tmp = this.stack[0];
  this.stack[0] = this.stack[1];
  this.stack[1] = tmp;
};
RpnCalc.prototype.swap.description = "Swaps the first and second items on the stack";

RpnCalc.prototype.pi = function() {
  this.push(Math.PI);
};
RpnCalc.prototype.pi.category = "Math";
RpnCalc.prototype.pi.description = "Adds the value of &pi; to the stack.";

RpnCalc.prototype.pow = function() {
  if (this.stack.length < 2) {
    throw new Error("Pow Error: Too few arguments");
  }
  var args = this.popValues(2);
  var result = Math.pow(args[0], args[1]);
  this.push(result);
};
RpnCalc.prototype.pow.category = "Math";
RpnCalc.prototype.pow.description = "Calculates S<sub>2</sub><sup>S<sub>1</sub></sup>";

RpnCalc.prototype.nroot = function() {
  if (this.stack.length < 2) {
    throw new Error("NRoot Error: Too few arguments");
  }
  var args = this.popValues(2);
  var result = Math.pow(args[0], 1 / args[1]);
  this.push(result);
};
RpnCalc.prototype.nroot.category = "Math";
RpnCalc.prototype.nroot.description = "Calculates <sup>S<sub>1</sub></sup>&radic;S<sub>2</sub>";

RpnCalc.prototype.plus = function() {
  if (this.stack.length < 2) {
    throw new Error("Add Error: Too few arguments");
  }
  var args = this.popValues(2);
  var result = args[0] + args[1];
  this.push(result);
};
RpnCalc.prototype.plus.category = "Math";
RpnCalc.prototype.plus.altName = '+';
RpnCalc.prototype.plus.description = "Calculates S<sub>2</sub> + S<sub>1</sub>";

RpnCalc.prototype.subtract = function() {
  if (this.stack.length < 2) {
    throw new Error("Subtract Error: Too few arguments");
  }
  var args = this.popValues(2);
  var result = args[0] - args[1];
  this.push(result);
};
RpnCalc.prototype.subtract.category = "Math";
RpnCalc.prototype.subtract.altName = '-';
RpnCalc.prototype.subtract.description = "Calculates S<sub>2</sub> - S<sub>1</sub>";

RpnCalc.prototype.multiply = function() {
  if (this.stack.length < 2) {
    throw new Error("Multiply Error: Too few arguments");
  }
  var args = this.popValues(2);
  var result = args[0] * args[1];
  this.push(result);
};
RpnCalc.prototype.multiply.category = "Math";
RpnCalc.prototype.multiply.altName = '*';
RpnCalc.prototype.multiply.description = "Calculates S<sub>2</sub> * S<sub>1</sub>";

RpnCalc.prototype.divide = function() {
  if (this.stack.length < 2) {
    throw new Error("Divide Error: Too few arguments");
  }
  var args = this.popValues(2);
  var result = args[0] / args[1];
  this.push(result);
};
RpnCalc.prototype.divide.category = "Math";
RpnCalc.prototype.divide.altName = '/';
RpnCalc.prototype.divide.description = "Calculates S<sub>2</sub> / S<sub>1</sub>";

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
RpnCalc.prototype.sin.category = "Trigonometry";
RpnCalc.prototype.sin.description = "Calculates sin(S<sub>1</sub>)";

RpnCalc.prototype.cos = function() {
  if (this.stack.length < 1) {
    throw new Error("Cos Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = Math.cos(this.toRadians(args[0]));
  this.push(result);
};
RpnCalc.prototype.cos.category = "Trigonometry";
RpnCalc.prototype.cos.description = "Calculates cos(S<sub>1</sub>)";

RpnCalc.prototype.tan = function() {
  if (this.stack.length < 1) {
    throw new Error("Tan Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = Math.tan(this.toRadians(args[0]));
  this.push(result);
};
RpnCalc.prototype.tan.category = "Trigonometry";
RpnCalc.prototype.tan.description = "Calculates tan(S<sub>1</sub>)";

RpnCalc.prototype.asin = function() {
  if (this.stack.length < 1) {
    throw new Error("Asin Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = this.radiansToAngle(Math.asin(args[0]));
  this.push(result);
};
RpnCalc.prototype.asin.category = "Trigonometry";
RpnCalc.prototype.asin.description = "Calculates asin(S<sub>1</sub>)";

RpnCalc.prototype.acos = function() {
  if (this.stack.length < 1) {
    throw new Error("Acos Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = this.radiansToAngle(Math.acos(args[0]));
  this.push(result);
};
RpnCalc.prototype.acos.category = "Trigonometry";
RpnCalc.prototype.acos.description = "Calculates acos(S<sub>1</sub>)";

RpnCalc.prototype.atan = function() {
  if (this.stack.length < 1) {
    throw new Error("Atan Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = this.radiansToAngle(Math.atan(args[0]));
  this.push(result);
};
RpnCalc.prototype.atan.category = "Trigonometry";
RpnCalc.prototype.atan.description = "Calculates atan(S<sub>1</sub>)";

RpnCalc.prototype.atan2 = function() {
  if (this.stack.length < 2) {
    throw new Error("Atan2 Error: Too few arguments");
  }
  var args = this.popValues(2);
  var result = this.radiansToAngle(Math.atan2(args[0], args[1]));
  this.push(result);
};
RpnCalc.prototype.atan2.category = "Trigonometry";
RpnCalc.prototype.atan2.description = "Calculates atan2(S<sub>1</sub>, S<sub>2</sub>)";

RpnCalc.prototype.sqrt = function() {
  if (this.stack.length < 1) {
    throw new Error("Sqrt Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = Math.sqrt(args[0]);
  this.push(result);
};
RpnCalc.prototype.sqrt.category = "Math";
RpnCalc.prototype.sqrt.description = "Calculates &radic;S<sub>1</sub>";

RpnCalc.prototype.fact = function() {
  if (this.stack.length < 1) {
    throw new Error("Fact Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = Math2.factorial(args[0]);
  this.push(result);
};
RpnCalc.prototype.fact.category = "Math";
RpnCalc.prototype.fact.description = "Calculates S<sub>1</sub>!";

RpnCalc.prototype.factorial = function() {
  return this.fact();
};

RpnCalc.prototype.pow2 = function() {
  if (this.stack.length < 1) {
    throw new Error("Pow2 Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = Math.pow(args[0], 2);
  this.push(result);
};
RpnCalc.prototype.pow2.category = "Math";
RpnCalc.prototype.pow2.description = "Calculates S<sub>1</sub><sup>2</sup>";

RpnCalc.prototype.inv = function() {
  if (this.stack.length < 1) {
    throw new Error("1/x Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = 1 / args[0];
  this.push(result);
};
RpnCalc.prototype.inv.category = "Math";
RpnCalc.prototype.inv.description = "Calculates 1/S<sub>1</sub>";

RpnCalc.prototype.log = function() {
  if (this.stack.length < 1) {
    throw new Error("Log Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = Math.log(args[0]) / Math.log(10);
  this.push(result);
};
RpnCalc.prototype.log.category = "Math";
RpnCalc.prototype.log.description = "Calculates log(S<sub>1</sub>)";

RpnCalc.prototype.ln = function() {
  if (this.stack.length < 1) {
    throw new Error("Ln Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = Math.log(args[0]);
  this.push(result);
};
RpnCalc.prototype.ln.category = "Math";
RpnCalc.prototype.ln.description = "Calculates ln(S<sub>1</sub>)";

RpnCalc.prototype.exp = function() {
  if (this.stack.length < 1) {
    throw new Error("Exp Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = Math.exp(args[0]);
  this.push(result);
};
RpnCalc.prototype.exp.category = "Math";
RpnCalc.prototype.exp.description = "Calculates <i>e</i><sup>S<sub>1</sub></sup>";

RpnCalc.prototype.neg = function() {
  if (this.stack.length < 1) {
    throw new Error("Neg Error: Too few arguments");
  }
  var args = this.popValues(1);
  var result = -args[0];
  this.push(result);
};
RpnCalc.prototype.neg.category = "Math";
RpnCalc.prototype.neg.description = "Calculates -S<sub>1</sub>";

RpnCalc.prototype.sto = function() {
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
RpnCalc.prototype.sto.description = "Stores S<sub>2</sub> into S<sub>1</sub>";

RpnCalc.prototype.store = function() {
  return this.sto();
};
