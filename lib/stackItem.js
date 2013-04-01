'use strict';

var units = require('./units');
var sf = require('sf');

var StackItem = module.exports = function(opts) {
  this.value = opts.value;
  this.type = opts.type || 'number';
  if (opts.units) {
    this.units = opts.units;
  }
  return this;
};

StackItem.prototype.toString = function(numBase, digitGrouping) {
  switch (this.type) {
  case 'number':
    if (isNaN(this.value)) {
      return 'Undefined';
    }
    if (this.value == 'Infinity') {
      return 'Infinity';
    }
    var str;
    switch (numBase) {
    case 10:
      if(digitGrouping) {
        var decimals = this.value.toString(10).match(/\.([0-9]*)$/);
        str = sf('{0:#,##0}', this.value);
        console.log(decimals);
        if(decimals) {
          str += '.' + decimals[1];
        }
      } else {
        str = this.value.toString(10);
      }
      break;
    case 16:
      str = '0x' + this.value.toString(16);
      break;
    case 8:
      str = '0' + this.value.toString(8);
      break;
    case 2:
      str = '0b' + this.value.toString(2);
      break;
    default:
      str = this.value.toString(numBase);
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
