import {Decimal} from "decimal.js";
import {RpnCalculator} from "../RpnCalculator";
import {DecimalStackItem} from "../DecimalStackItem";

test('simple 3*4', async () => {
    const calc = new RpnCalculator();
    await calc.push(['3', '4', '*']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(12));
});

test('units 3m * 4cm', async () => {
    const calc = new RpnCalculator();
    await calc.push(['3_m', '4_cm', '*']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(0.12));
    expect((<DecimalStackItem>results[0]).units).toEqual('m2');
});

test('units 3 * 4m', async () => {
    const calc = new RpnCalculator();
    await calc.push(['3', '4_m', '*']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(12));
    expect((<DecimalStackItem>results[0]).units).toEqual('m');
});

test('different units 3s * 4m', async () => {
    const calc = new RpnCalculator();
    await calc.push(['3_s', '4_m', '*']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(12));
    expect((<DecimalStackItem>results[0]).units).toEqual('s*m');
});
