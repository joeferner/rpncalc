import {RpnCalculator} from "../RpnCalculator";
import {BinaryOperator} from "./BinaryOperator";
import {Decimal} from "decimal.js";
import {AngleOutputUnaryOperator} from "./AngleOutputUnaryOperator";
import {StackItem} from "../StackItem";

export class ATan2Operator extends BinaryOperator {
    public getKeywords(): string[] {
        return ['atan2'];
    }

    protected async executeBinary(a: StackItem, b: StackItem, rpnCalculator: RpnCalculator): Promise<Decimal> {
        const aVal = await a.decimalValue(rpnCalculator);
        const bVal = await b.decimalValue(rpnCalculator);
        return AngleOutputUnaryOperator.fromRadians(Decimal.atan2(aVal, bVal), rpnCalculator);
    }
}
