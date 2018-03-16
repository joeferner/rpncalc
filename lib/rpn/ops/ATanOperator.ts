import {RpnCalculator} from "../RpnCalculator";
import {AngleOutputUnaryOperator} from "./AngleOutputUnaryOperator";

export class ATanOperator extends AngleOutputUnaryOperator {
    public getKeywords(): string[] {
        return ['atan', 'atangent'];
    }

    protected executeUnaryAngleResult(a: Decimal, rpnCalculator: RpnCalculator) {
        return a.atan();
    }
}

