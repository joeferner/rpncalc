import {Decimal} from "decimal.js";
import {RpnCalculator} from "../RpnCalculator";
import {DecimalStackItem} from "../DecimalStackItem";

test('simple 1+2', async () => {
    const calc = new RpnCalculator();
    await calc.push(['1', '2', '+']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(3));
});

test('units (plus) 1_cm + 20_mm', async () => {
    const calc = new RpnCalculator();
    await calc.push(['1_cm', '20_mm', '+']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(3));
    expect((<DecimalStackItem>results[0]).units).toEqual('cm');
});

test('units (take first units) 20_mm + 1_cm', async () => {
    const calc = new RpnCalculator();
    await calc.push(['20_mm', '1_cm', '+']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(30));
    expect((<DecimalStackItem>results[0]).units).toEqual('mm');
});

test('units (incompatible) 20_m + 1_s', async () => {
    const calc = new RpnCalculator();
    await calc.push(['20_m', '1_s']);
    expect(calc.push('+')).rejects.toThrow();
});
