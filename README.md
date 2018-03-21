## rpncalc

rpncalc is a simple command line RPN calculator using [decimal.js](https://www.npmjs.com/package/decimal.js) big number support.

- Backspace with nothing in the immediate value will drop the first item on the stack
- Tab will autocomplete
- Switch bases using `hex`, `bin`, `dec`, `oct`
- Switch angle mode using `deg`, `rad`
- Enable digit grouping pushing the following on the stack `'digitGrouping'`, `1`, `set`
- Expressions can be entered surrounded with `'` then evaluated with `eval`
