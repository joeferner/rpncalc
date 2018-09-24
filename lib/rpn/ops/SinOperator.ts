import {RpnCalculator} from "../RpnCalculator";
import {AngleInputUnaryOperator} from "./AngleInputUnaryOperator";
import {Decimal} from "decimal.js";

export class SinOperator extends AngleInputUnaryOperator {
    public getKeywords(): string[] {
        return ['sin', 'sine'];
    }

    protected executeUnaryOnAngle(angleInRadians: Decimal, rpnCalculator: RpnCalculator) {
        return angleInRadians.sin();
    }
}

