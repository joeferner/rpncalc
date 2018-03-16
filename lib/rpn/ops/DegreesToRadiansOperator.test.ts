import {RpnCalculator} from "../RpnCalculator";

test('180', async () => {
    const calc = new RpnCalculator();
    await calc.push(['180', 'deg2rad']);
    const results = await calc.pop();
    expect((await results[0].decimalValue(calc)).toNumber()).toBeCloseTo(3.1415926535897932384626433, 5);
});
