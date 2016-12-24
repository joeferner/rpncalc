/// <reference path="../types.d.ts" />
"use strict";
(function (DecimalJsOutputMode) {
    DecimalJsOutputMode[DecimalJsOutputMode["FIXED"] = 0] = "FIXED";
})(exports.DecimalJsOutputMode || (exports.DecimalJsOutputMode = {}));
var DecimalJsOutputMode = exports.DecimalJsOutputMode;
(function (AngleMode) {
    AngleMode[AngleMode["DEGREES"] = 0] = "DEGREES";
    AngleMode[AngleMode["RADIANS"] = 1] = "RADIANS";
})(exports.AngleMode || (exports.AngleMode = {}));
var AngleMode = exports.AngleMode;
var StateOutput = (function () {
    function StateOutput() {
    }
    return StateOutput;
}());
exports.StateOutput = StateOutput;
var State = (function () {
    function State() {
        this.input = '';
        this.stack = [];
        this.angleMode = AngleMode.DEGREES;
        this.output = {
            decimalJsMode: DecimalJsOutputMode.FIXED,
            numberOfDecimalPoints: 20,
            base: 10,
            digitGrouping: false
        };
        this.error = null;
    }
    return State;
}());
exports.__esModule = true;
exports["default"] = State;
