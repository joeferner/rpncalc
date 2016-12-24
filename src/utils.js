"use strict";
function isExpression(v) {
    return v.length >= 2 && v[0] === "'" && v[v.length - 1] === "'";
}
exports.isExpression = isExpression;
