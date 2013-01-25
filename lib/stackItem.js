'use strict';

var StackItem = module.exports = function(opts) {
  this.value = opts.value;
  this.type = opts.type || 'number';
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
