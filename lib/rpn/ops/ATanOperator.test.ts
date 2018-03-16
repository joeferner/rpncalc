import {RpnCalculator} from "../RpnCalculator";

test('atan degrees', async () => {
    const calc = new RpnCalculator();
    await calc.push(['deg', '0.577350269', 'atan']);
    const results = await calc.pop();
    expect((await results[0].decimalValue(calc)).toNumber()).toBeCloseTo(30.00, 5);
});

test('atan radians', async () => {
    const calc = new RpnCalculator();
    await calc.push(['rad', '0.577350269', 'atan']);
    const results = await calc.pop();
    expect((await results[0].decimalValue(calc)).toNumber()).toBeCloseTo(0.5235987754560796, 5);
});
