import {Decimal} from "decimal.js";
import {RpnCalculator} from "../RpnCalculator";
import {DecimalStackItem} from "../DecimalStackItem";

test('convert raw to units', async () => {
    const calc = new RpnCalculator();
    await calc.push(['42', "'mm'", 'convert']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(42));
    expect((<DecimalStackItem>results[0]).units).toEqual('mm');
});

test('convert mm to m', async () => {
    const calc = new RpnCalculator();
    await calc.push(['42_mm', "'m'", 'convert']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(0.042));
    expect((<DecimalStackItem>results[0]).units).toEqual('m');
});

test('convert mm to um', async () => {
    const calc = new RpnCalculator();
    await calc.push(['42_mm', "'um'", 'convert']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(42000));
    expect((<DecimalStackItem>results[0]).units).toEqual('um');
});

test('convert mm to s (bad)', async () => {
    const calc = new RpnCalculator();
    await calc.push(['42_mm', "'s'"]);
    expect(calc.push('convert')).rejects.toThrow();
});
