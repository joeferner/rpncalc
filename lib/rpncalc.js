'use strict';

var StackItem = require('./stackItem');

var RpnCalc = module.exports = function() {
  this.angleMode = 'rad';
  this.numBase = 10;
  this.stack = [];
};

RpnCalc.prototype.push = function(val) {
  if (val.trim) {
    val = val.trim();
  }
  switch (val) {
  case 'sqrt':
    this.sqrt();
    break;
  case 'sin':
    this.sin();
    break;
  case 'cos':
    this.cos();
    break;
  case 'tan':
    this.tan();
    break;
  case 'asin':
    this.asin();
    break;
  case 'acos':
    this.acos();
    break;
  case 'atan':
    this.atan();
    break;
  case 'atan2':
    this.atan2();
    break;
  case 'inv':
    this.inv();
    break;
  case 'log':
    this.log();
    break;
  case 'ln':
    this.ln();
    break;
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
    this.stack.unshift(new StackItem(val, this.numBase));
    break;
  }
};

RpnCalc.prototype.pop = function(count) {
  count = count || 1;
  if (this.stack.length < count) {
    throw new Error("Too few items on stack");
  }
  return this.stack.splice(0, count).reverse();
};

RpnCalc.prototype.drop = function() {
  if (this.stack.length == 0) {
    throw new Error('Drop Error: Too few arguments');
  }
  this.stack = this.stack.slice(1);
};

RpnCalc.prototype.plus = function() {
  if (this.stack.length < 2) {
    throw new Error("Add Error: Too few arguments");
  }
  var args = this.pop(2);
  var result = args[0].value + args[1].value;
  this.push(result);
};

RpnCalc.prototype.subtract = function() {
  if (this.stack.length < 2) {
    throw new Error("Subtract Error: Too few arguments");
  }
  var args = this.pop(2);
  var result = args[0].value - args[1].value;
  this.push(result);
};

RpnCalc.prototype.multiply = function() {
  if (this.stack.length < 2) {
    throw new Error("Multiply Error: Too few arguments");
  }
  var args = this.pop(2);
  var result = args[0].value * args[1].value;
  this.push(result);
};

RpnCalc.prototype.divide = function() {
  if (this.stack.length < 2) {
    throw new Error("Divide Error: Too few arguments");
  }
  var args = this.pop(2);
  var result = args[0].value / args[1].value;
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
  var args = this.pop(1);
  var result = Math.sin(this.toRadians(args[0].value));
  this.push(result);
};

RpnCalc.prototype.cos = function() {
  if (this.stack.length < 1) {
    throw new Error("Cos Error: Too few arguments");
  }
  var args = this.pop(1);
  var result = Math.cos(this.toRadians(args[0].value));
  this.push(result);
};

RpnCalc.prototype.tan = function() {
  if (this.stack.length < 1) {
    throw new Error("Tan Error: Too few arguments");
  }
  var args = this.pop(1);
  var result = Math.tan(this.toRadians(args[0].value));
  this.push(result);
};

RpnCalc.prototype.asin = function() {
  if (this.stack.length < 1) {
    throw new Error("Asin Error: Too few arguments");
  }
  var args = this.pop(1);
  var result = this.radiansToAngle(Math.asin(args[0].value));
  this.push(result);
};

RpnCalc.prototype.acos = function() {
  if (this.stack.length < 1) {
    throw new Error("Acos Error: Too few arguments");
  }
  var args = this.pop(1);
  var result = this.radiansToAngle(Math.acos(args[0].value));
  this.push(result);
};

RpnCalc.prototype.atan = function() {
  if (this.stack.length < 1) {
    throw new Error("Atan Error: Too few arguments");
  }
  var args = this.pop(1);
  var result = this.radiansToAngle(Math.atan(args[0].value));
  this.push(result);
};

RpnCalc.prototype.atan2 = function() {
  if (this.stack.length < 2) {
    throw new Error("Atan2 Error: Too few arguments");
  }
  var args = this.pop(2);
  var result = this.radiansToAngle(Math.atan2(args[0].value, args[1].value));
  this.push(result);
};

RpnCalc.prototype.sqrt = function() {
  if (this.stack.length < 1) {
    throw new Error("Sqrt Error: Too few arguments");
  }
  var args = this.pop(1);
  var result = Math.sqrt(args[0].value);
  this.push(result);
};

RpnCalc.prototype.inv = function() {
  if (this.stack.length < 1) {
    throw new Error("1/x Error: Too few arguments");
  }
  var args = this.pop(1);
  var result = 1 / args[0].value;
  this.push(result);
};

RpnCalc.prototype.log = function() {
  if (this.stack.length < 1) {
    throw new Error("Log Error: Too few arguments");
  }
  var args = this.pop(1);
  var result = Math.log(args[0].value) / Math.log(10);
  this.push(result);
};

RpnCalc.prototype.ln = function() {
  if (this.stack.length < 1) {
    throw new Error("Ln Error: Too few arguments");
  }
  var args = this.pop(1);
  var result = Math.log(args[0].value);
  this.push(result);
};