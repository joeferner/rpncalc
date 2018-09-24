import {RpnCalculator} from "../RpnCalculator";
import {BinaryOperator} from "./BinaryOperator";
import {StackItem} from "../StackItem";
import {Decimal} from "decimal.js";

export class PowerOperator extends BinaryOperator {
    public getKeywords(): string[] {
        return ['pow', 'power', '^'];
    }

    protected async executeBinary(a: StackItem, b: StackItem, rpnCalculator: RpnCalculator): Promise<Decimal> {
        const aVal = await a.decimalValue(rpnCalculator);
        const bVal = await b.decimalValue(rpnCalculator);
        return aVal.pow(bVal);
    }
}
