import {RpnCalculator} from "../RpnCalculator";

test('acos degrees', async () => {
    const calc = new RpnCalculator();
    await calc.push(['deg', '0.86602540378', 'acos']);
    const results = await calc.pop();
    expect((await results[0].decimalValue(calc)).toNumber()).toBeCloseTo(30.00, 5);
});

test('acos radians', async () => {
    const calc = new RpnCalculator();
    await calc.push(['rad', '0.86602540378', 'acos']);
    const results = await calc.pop();
    expect((await results[0].decimalValue(calc)).toNumber()).toBeCloseTo(0.5235987756071762, 5);
});
