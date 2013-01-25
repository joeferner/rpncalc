'use strict';

exports.factorial = function(x) {
  var result = 1;
  for (var i = 1; i <= x; i++) {
    result *= i;
  }
  return result;
};
