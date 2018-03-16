import {RpnCalculator} from "../RpnCalculator";
import {BinaryOperator} from "./BinaryOperator";
import {Decimal} from "decimal.js";
import {StackItem} from "../StackItem";

export class NRootOperator extends BinaryOperator {
    public getKeywords(): string[] {
        return ['nroot'];
    }

    protected async executeBinary(a: StackItem, b: StackItem, rpnCalculator: RpnCalculator): Promise<Decimal> {
        const aVal = await a.decimalValue(rpnCalculator);
        const bVal = await b.decimalValue(rpnCalculator);
        return aVal.pow(new Decimal(1.0).dividedBy(bVal));
    }
}
