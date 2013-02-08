'use strict';

var expressionEvaluator = require('../lib/expressionEvaluator');
var Rpncalc = require('../lib/rpncalc');

module.exports = {
  setUp: function(callback) {
    this.rpncalc = new Rpncalc();
    this.rpncalc.angleMode = 'deg';
    this.evalExpr = function(expr) {
      return expressionEvaluator(expr, this.rpncalc);
    };
    callback();
  },

  'value': function(test) {
    test.equals(42, this.evalExpr('42'));
    test.done();
  },

  'unary operator': function(test) {
    test.equals(-1, this.evalExpr('-1'));
    test.equals(4, this.evalExpr('5 + -1'));
    test.done();
  },

  'binary operator': function(test) {
    test.equals(2, this.evalExpr('1+1'));
    test.equals(0, this.evalExpr('1-1'));
    test.equals(10, this.evalExpr('5*2'));
    test.equals(0.5, this.evalExpr('1/2'));
    test.done();
  },

  'precedence': function(test) {
    test.equals(11, this.evalExpr('5 + 2 * 3'));
    test.equals(11, this.evalExpr('2 * 3 + 5'));
    test.equals(4, this.evalExpr('5 + 2 - 3'));
    test.equals(21, this.evalExpr('(5 + 2) * 3'));
    test.done();
  },

  'functions': function(test) {
    test.ok(Math.abs(0.5 - this.evalExpr('sin(30)').value) < 0.0001);
    test.equals(8, this.evalExpr('pow(2, 3)'));
    test.done();
  }
};
