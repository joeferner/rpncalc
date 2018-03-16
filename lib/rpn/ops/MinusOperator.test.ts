import {Decimal} from "decimal.js";
import {RpnCalculator} from "../RpnCalculator";
import {DecimalStackItem} from "../DecimalStackItem";

test('simple 5-4', async () => {
    const calc = new RpnCalculator();
    await calc.push(['5', '4', '-']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(1));
});

test('units (minus) 1_cm - 20_mm', async () => {
    const calc = new RpnCalculator();
    await calc.push(['3_cm', '20_mm', '-']);
    const results = await calc.pop();
    expect(results[0].value).toEqual(new Decimal(1));
    expect((<DecimalStackItem>results[0]).units).toEqual('cm');
});
