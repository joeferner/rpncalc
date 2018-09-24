import {RpnCalculator} from "../RpnCalculator";
import {AngleInputUnaryOperator} from "./AngleInputUnaryOperator";
import {Decimal} from "decimal.js";

export class TanOperator extends AngleInputUnaryOperator {
    public getKeywords(): string[] {
        return ['tan', 'tangent'];
    }

    protected executeUnaryOnAngle(angleInRadians: Decimal, rpnCalculator: RpnCalculator) {
        return angleInRadians.tan();
    }
}

