import {RpnCalculator} from "../RpnCalculator";
import {AngleInputUnaryOperator} from "./AngleInputUnaryOperator";

export class SinOperator extends AngleInputUnaryOperator {
    public getKeywords(): string[] {
        return ['sin', 'sine'];
    }

    protected executeUnaryOnAngle(angleInRadians: Decimal, rpnCalculator: RpnCalculator) {
        return angleInRadians.sin();
    }
}

