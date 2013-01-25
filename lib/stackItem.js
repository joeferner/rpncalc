'use strict';

var StackItem = module.exports = function(val, numBase) {
  var m;

  var valStr = val.toString();
  if (m = valStr.match(/^0x([0-9a-fA-F]*)$/)) {
    this.value = parseInt(m[1], 16);
  } else if (m = valStr.match(/^0b([0-1]*)$/)) {
    this.value = parseInt(m[1], 2);
  } else if (m = valStr.match(/^0([0-8]*)$/)) {
    this.value = parseInt(m[1], 8);
  } else {
    switch (numBase) {
    case 16:
    case 8:
    case 2:
      this.value = parseInt(val, numBase);
      break;
    default:
      this.value = parseFloat(val);
      break;
    }
  }
  if (isNaN(this.value)) {
    throw new Error("Could not parse number (base: " + numBase + ")");
  }
  return this;
};

StackItem.prototype.toString = function(numBase) {
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
};
