import {Decimal} from "decimal.js";
import {RpnCalculator} from "../RpnCalculator";

test('swap to values', async () => {
    const calc = new RpnCalculator();
    await calc.push(['12', '4', 'swap']);
    let results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(12));
    results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(4));
});

test('swap not enough operands', async () => {
    const calc = new RpnCalculator();
    await calc.push(['12']);
    expect(calc.push('swap')).rejects.toThrow();
});
