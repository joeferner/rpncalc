import {RpnCalculator} from "../RpnCalculator";
import {AngleOutputUnaryOperator} from "./AngleOutputUnaryOperator";

export class ACosOperator extends AngleOutputUnaryOperator {
    public getKeywords(): string[] {
        return ['acos', 'acosine'];
    }

    protected executeUnaryAngleResult(a: Decimal, rpnCalculator: RpnCalculator) {
        return a.acos();
    }
}

