'use strict';

var jison = require('jison');

var parser = new jison.Parser({
  "lex": {
    "rules": [
      ["\\s+", "/* skip whitespace */"],
      ["[0-9]+(?:\\.[0-9]+)?", "return 'NUMBER';"],
      ["[a-zA-Z_][a-zA-Z0-9_]*", "return 'IDENTIFIER';"],
      ["\\*", "return '*';"],
      ["\\/", "return '/';"],
      ["-", "return '-';"],
      ["\\+", "return '+';"],
      ["\\^", "return '^';"],
      ["\\(", "return '(';"],
      ["\\)", "return ')';"],
      [",", "return ',';"],
      ["$", "return 'EOF';"]
    ]
  },

  "operators": [
    ["left", "+", "-"],
    ["left", "*", "/"],
    ["left", "^"],
    ["left", "UMINUS"]
  ],

  "bnf": {
    "expressions": [
      [ "e EOF", "return $1;"  ]
    ],

    "parameterList": [
      [ "e", "$$ = [ $1 ];" ],
      [ "parameterList , e", "$1.push($3); $$ = $1;" ]
    ],

    "e": [
      [ "IDENTIFIER ( parameterList )", "$$ = { type: 'function', name: $1, parameters: $3 };" ],
      [ "e + e", "$$ = { type: 'binary', op: '+', left: $1, right: $3 };" ],
      [ "e - e", "$$ = { type: 'binary', op: '-', left: $1, right: $3 };" ],
      [ "e * e", "$$ = { type: 'binary', op: '*', left: $1, right: $3 };" ],
      [ "e / e", "$$ = { type: 'binary', op: '/', left: $1, right: $3 };" ],
      [ "e ^ e", "$$ = { type: 'binary', op: 'pow', left: $1, right: $3 };" ],
      [ "- e", "$$ = { type: 'unary', op: '-', operand: $2 };", {"prec": "UMINUS"} ],
      [ "( e )", "$$ = $2;" ],
      [ "IDENTIFIER", "$$ = { type: 'identifier', value: $1 };" ],
      [ "NUMBER", "$$ = { type: 'number', value: $1 };" ]
    ]
  }
});

module.exports = function(str, rpncalc) {
  var expressionTree = parse(str);
  var newRpnCalc = rpncalc.createEmpty();
  runTree(expressionTree, newRpnCalc);
  return newRpnCalc.stack[0];
};

var parse = module.exports.parse = function(str) {
  return parser.parse(str);
};

var runTree = module.exports.runTree = function(tree, rpncalc) {
  switch (tree.type) {
  case 'number':
    return runTreeNumber(tree, rpncalc);
  case 'binary':
    return runTreeBinary(tree, rpncalc);
  case 'unary':
    return runTreeUnary(tree, rpncalc);
  case 'function':
    return runTreeFunction(tree, rpncalc);
  case 'identifier':
    return runTreeIdentifier(tree, rpncalc);
  default:
    throw new Error('Unknown tree type: ' + tree.type);
  }
};

function runTreeIdentifier(tree, rpncalc) {
  rpncalc.push(tree.value);
}

function runTreeFunction(tree, rpncalc) {
  tree.parameters.forEach(function(param) {
    runTree(param, rpncalc);
  });
  rpncalc.push(tree.name);
}

function runTreeUnary(tree, rpncalc) {
  runTree(tree.operand, rpncalc);
  switch (tree.op) {
  case '-':
    return rpncalc.push('neg');
  default:
    throw new Error('Invalid unary operator: ' + tree.op);
  }
}

function runTreeBinary(tree, rpncalc) {
  runTree(tree.left, rpncalc);
  runTree(tree.right, rpncalc);
  switch (tree.op) {
  case '+':
    return rpncalc.push('plus');
  case '-':
    return rpncalc.push('subtract');
  case '/':
    return rpncalc.push('divide');
  case '*':
    return rpncalc.push('multiply');
  case 'pow':
    return rpncalc.push('pow');
  default:
    throw new Error('Invalid binary operator: ' + tree.op);
  }
}

function runTreeNumber(tree, rpncalc) {
  return rpncalc.push(tree.value);
}