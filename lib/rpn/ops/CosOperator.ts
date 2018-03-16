import {RpnCalculator} from "../RpnCalculator";
import {AngleInputUnaryOperator} from "./AngleInputUnaryOperator";

export class CosOperator extends AngleInputUnaryOperator {
    public getKeywords(): string[] {
        return ['cos', 'cosine'];
    }

    protected executeUnaryOnAngle(angleInRadians: Decimal, rpnCalculator: RpnCalculator) {
        return angleInRadians.cos();
    }
}

