import {Decimal} from "decimal.js";
import {RpnCalculator} from "../RpnCalculator";
import {DecimalStackItem} from "../DecimalStackItem";

test('simple -3', async () => {
    const calc = new RpnCalculator();
    await calc.push(['3', 'neg']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(-3));
});

test('units -3m', async () => {
    const calc = new RpnCalculator();
    await calc.push(['3_m', 'neg']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(-3));
    expect((<DecimalStackItem>results[0]).units).toEqual('m');
});
