import {RpnCalculator} from "../RpnCalculator";
import {AngleOutputUnaryOperator} from "./AngleOutputUnaryOperator";

export class ASinOperator extends AngleOutputUnaryOperator {
    public getKeywords(): string[] {
        return ['asin', 'asine'];
    }

    protected executeUnaryAngleResult(a: Decimal, rpnCalculator: RpnCalculator) {
        return a.asin();
    }
}

