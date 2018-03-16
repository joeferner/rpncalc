import {RpnCalculator} from "../RpnCalculator";

test('asin degrees', async () => {
    const calc = new RpnCalculator();
    await calc.push(['deg', '0.86602540378', 'asin']);
    const results = await calc.pop();
    expect((await results[0].decimalValue(calc)).toNumber()).toBeCloseTo(60.00, 5);
});

test('asin radians', async () => {
    const calc = new RpnCalculator();
    await calc.push(['rad', '0.86602540378', 'asin']);
    const results = await calc.pop();
    expect((await results[0].decimalValue(calc)).toNumber()).toBeCloseTo(1.0471975511877205, 5);
});
