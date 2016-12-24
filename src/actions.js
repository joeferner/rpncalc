/// <reference path="./types.d.ts" />
"use strict";
exports.setInputText = function (text) {
    return {
        type: 'SET_INPUT_TEXT',
        text: text
    };
};
exports.pushStack = function (text) {
    return {
        type: 'PUSH_STACK',
        text: text
    };
};
exports.executeOperator = function (op) {
    return {
        type: 'EXECUTE_OPERATOR',
        op: op
    };
};
exports.appendInput = function (text) {
    return {
        type: 'APPEND_INPUT',
        text: text
    };
};
exports.setDigitGrouping = function (digitGrouping) {
    return {
        type: 'SET_DIGIT_GROUPING',
        digitGrouping: digitGrouping
    };
};
