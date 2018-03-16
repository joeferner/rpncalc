import {Decimal} from "decimal.js";
import {RpnCalculator} from "../RpnCalculator";
import {DecimalStackItem} from "../DecimalStackItem";

test('simple 12/4', async () => {
    const calc = new RpnCalculator();
    await calc.push(['12', '4', '/']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(3));
});

test('divide by 0', async () => {
    const calc = new RpnCalculator();
    await calc.push(['5', '0']);
    expect(calc.push('/')).rejects.toThrow();
});

test('units 3m / 4cm', async () => {
    const calc = new RpnCalculator();
    await calc.push(['3_m', '4_cm', '/']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(75));
    expect((<DecimalStackItem>results[0]).units).toEqual(null);
});

test('units 3m / 4', async () => {
    const calc = new RpnCalculator();
    await calc.push(['3_m', '4', '/']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(0.75));
    expect((<DecimalStackItem>results[0]).units).toEqual('m');
});

test('units 3 / 4m', async () => {
    const calc = new RpnCalculator();
    await calc.push(['3', '4_m', '/']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(0.75));
    expect((<DecimalStackItem>results[0]).units).toEqual('1/m');
});

test('different units 3m / 4s', async () => {
    const calc = new RpnCalculator();
    await calc.push(['3_m', '4_s', '/']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(0.75));
    expect((<DecimalStackItem>results[0]).units).toEqual('m/s');
});
