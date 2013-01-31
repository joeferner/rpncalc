'use strict';

var units = require('./units');

var StackItem = module.exports = function(opts) {
  this.value = opts.value;
  this.type = opts.type || 'number';
  if (opts.units) {
    this.units = opts.units;
  }
  return this;
};

StackItem.prototype.toString = function(numBase) {
  switch (this.type) {
  case 'number':
    if (isNaN(this.value)) {
      return this.value;
    }
    var str = this.value.toString(numBase);
    switch (numBase) {
    case 16:
      str = '0x' + str;
      break;
    case 8:
      str = '0' + str;
      break;
    case 2:
      str = '0b' + str;
      break;
    }

    if (this.units) {
      str += '_' + this.units;
    }

    return str;

  case 'expression':
    return "'" + this.value + "'";

  default:
    return 'unhandled type: ' + this.type;
  }
};

StackItem.prototype.getValue = function() {
  if (this.type == 'number') {
    return this.value;
  } else {
    throw new Error("Could not evaluate expression: " + this.value);
  }
};

StackItem.prototype.getValueInUnits = function(unitStr) {
  if (this.type == 'number') {
    return units.convert(this.value, this.units, unitStr);
  } else {
    throw new Error("Could not evaluate expression: " + this.value);
  }
};
