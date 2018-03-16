import {Decimal} from "decimal.js";
import {RpnCalculator} from "../RpnCalculator";

test('simple inverse of 2', async () => {
    const calc = new RpnCalculator();
    await calc.push(['2', 'inv']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(0.5));
});

test('inverse of 0', async () => {
    const calc = new RpnCalculator();
    await calc.push(['0']);
    expect(calc.push('inv')).rejects.toThrow();
});
