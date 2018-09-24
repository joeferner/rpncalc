import {RpnCalculator} from "../RpnCalculator";
import {AngleOutputUnaryOperator} from "./AngleOutputUnaryOperator";
import {Decimal} from "decimal.js";

export class ASinOperator extends AngleOutputUnaryOperator {
    public getKeywords(): string[] {
        return ['asin', 'asine'];
    }

    protected executeUnaryAngleResult(a: Decimal, rpnCalculator: RpnCalculator) {
        return a.asin();
    }
}

