import {RpnCalculator} from "../RpnCalculator";

test('3.1415', async () => {
    const calc = new RpnCalculator();
    await calc.push(['3.1415926535897932384626433', 'rad2deg']);
    const results = await calc.pop();
    expect((await results[0].decimalValue(calc)).toNumber()).toBeCloseTo(180.0, 5);
});
