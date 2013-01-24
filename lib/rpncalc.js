'use strict';

var util = require('util');
var events = require('events');
var keys = require('./keys');

var RpnCalc = module.exports = function() {
  events.EventEmitter.call(this);

  this.angleMode = 'rad';
  this.numBase = 10;
  this.stack = [];
  this.input = '';
};
util.inherits(RpnCalc, events.EventEmitter);

RpnCalc.prototype.pushInput = function() {
  this.push(this.input);
  this.input = '';
  this.emit('change');
};

RpnCalc.prototype.push = function(val) {
  this.stack.push({
    value: val
  });
  this.emit('change');
};

RpnCalc.prototype.drop = function() {
  this.stack = this.stack.slice(1);
  this.emit('change');
};

RpnCalc.prototype.onKeyPress = function(key) {
  switch (event.which) {
  case keys.BACKSPACE:
  case keys.ENTER:
    break;
  default:
    console.log('onKeyPress', event.which);
    var ch = String.fromCharCode(event.which);
    this.input += ch;
    this.emit('change');
  }
};

RpnCalc.prototype.onKeyDown = function(key) {
  switch (event.which) {
  case keys.BACKSPACE:
    if (this.input && this.input.length > 0) {
      this.input = this.input.slice(0, this.input.length - 1);
      this.emit('change');
    } else if (this.stack.length > 0) {
      this.drop();
    } else {
      return this.emit('error', 'Drop Error: Too few arguments');
    }
    break;

  case keys.ENTER:
    this.pushInput();
    break;

  default:
    console.log('onKeyDown', event.which);
  }
};

