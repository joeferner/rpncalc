import {ExpressionParser} from "./ExpressionParser";
import {Decimal} from "decimal.js";
import {RpnCalculator} from "../rpn/RpnCalculator";

const parser = new ExpressionParser();

test('simple 1 + 1', async () => {
    const calc = new RpnCalculator();
    const results = await parser.execute('1 + 1', calc);
    expect(results[0].value).toEqual(new Decimal(2));
});

test('negate -42', async () => {
    const calc = new RpnCalculator();
    const results = await parser.execute('-42', calc);
    expect(results[0].value).toEqual(new Decimal(-42));
});

test('operator precedence 4 + 3 * 2', async () => {
    const calc = new RpnCalculator();
    const results = await parser.execute('4 + 3 * 2', calc);
    expect(results[0].value).toEqual(new Decimal(10));
});

test('operator precedence ( 4 + 3 ) * 2', async () => {
    const calc = new RpnCalculator();
    const results = await parser.execute('( 4 + 3 ) * 2', calc);
    expect(results[0].value).toEqual(new Decimal(14));
});

test('function sin(30)', async () => {
    const calc = new RpnCalculator();
    const results = await parser.execute('sin(30)', calc);
    expect(results[0].value).toEqual(new Decimal(0.5));
});

test('multiple expressions radians; sin(pi/6)', async () => {
    const calc = new RpnCalculator();
    const results = await parser.execute('radians; sin(pi/6)', calc);
    expect(results[0].value).toEqual(new Decimal(0.5));
});

test('variables', async () => {
    const calc = new RpnCalculator();
    await calc.push(['5', "'a'", 'store']);
    const results = await parser.execute('a+2', calc);
    expect(results[0].value).toEqual(new Decimal(7));
});
